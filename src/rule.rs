use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::{Result, Context};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Confidence {
    HIGH,
    MEDIUM,
    LOW,
}

impl std::fmt::Display for Confidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Confidence::HIGH => write!(f, "HIGH"),
            Confidence::MEDIUM => write!(f, "MEDIUM"),
            Confidence::LOW => write!(f, "LOW"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuleConfig {
    pub rule: Rule,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rule {
    pub id: String,
    pub language: String,
    pub confidence: Confidence,
    pub message: String,
    pub explanation: String,
    /// Tree-sitter S-expression query
    pub query: String,
}

impl RuleConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read rule file: {:?}", path.as_ref()))?;
        let config: RuleConfig = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse YAML from rule file: {:?}", path.as_ref()))?;
        Ok(config)
    }

    pub fn load_all_from_dir<P: AsRef<Path>>(dir: P) -> Result<Vec<Rule>> {
        let mut rules = Vec::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                rules.extend(Self::load_all_from_dir(&path)?);
            } else if path.extension().and_then(|s| s.to_str()) == Some("yaml") ||
                      path.extension().and_then(|s| s.to_str()) == Some("yml") {
                let config = Self::load_from_file(&path)?;
                rules.push(config.rule);
            }
        }
        Ok(rules)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_confidence_display() {
        assert_eq!(Confidence::HIGH.to_string(), "HIGH");
        assert_eq!(Confidence::MEDIUM.to_string(), "MEDIUM");
        assert_eq!(Confidence::LOW.to_string(), "LOW");
    }

    #[test]
    fn test_confidence_serialize() {
        let high = serde_json::to_string(&Confidence::HIGH).unwrap();
        assert_eq!(high, "\"HIGH\"");
    }

    #[test]
    fn test_load_valid_yaml() {
        let dir = TempDir::new().unwrap();
        let yaml = r#"
rule:
  id: "test-rule"
  language: "typescript"
  confidence: "HIGH"
  message: "Test message"
  explanation: "Test explanation"
  query: "(program) @test"
"#;
        std::fs::write(dir.path().join("test.yaml"), yaml).unwrap();
        let config = RuleConfig::load_from_file(dir.path().join("test.yaml")).unwrap();
        assert_eq!(config.rule.id, "test-rule");
        assert_eq!(config.rule.language, "typescript");
        assert_eq!(config.rule.confidence, Confidence::HIGH);
        assert_eq!(config.rule.message, "Test message");
    }

    #[test]
    fn test_load_all_from_dir() {
        let dir = TempDir::new().unwrap();
        let ts_dir = dir.path().join("typescript");
        let py_dir = dir.path().join("python");
        std::fs::create_dir(&ts_dir).unwrap();
        std::fs::create_dir(&py_dir).unwrap();

        let yaml1 = r#"
rule:
  id: "rule-1"
  language: "typescript"
  confidence: "HIGH"
  message: "M1"
  explanation: "E1"
  query: "(program) @test"
"#;
        let yaml2 = r#"
rule:
  id: "rule-2"
  language: "python"
  confidence: "MEDIUM"
  message: "M2"
  explanation: "E2"
  query: "(module) @test"
"#;
        std::fs::write(ts_dir.join("r1.yaml"), yaml1).unwrap();
        std::fs::write(py_dir.join("r2.yaml"), yaml2).unwrap();

        let rules = RuleConfig::load_all_from_dir(dir.path()).unwrap();
        assert_eq!(rules.len(), 2);
    }

    #[test]
    fn test_load_missing_file() {
        let result = RuleConfig::load_from_file("nonexistent.yaml");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_invalid_yaml() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("bad.yaml"), "!!! not: valid: yaml: [[").unwrap();
        let result = RuleConfig::load_from_file(dir.path().join("bad.yaml"));
        assert!(result.is_err());
    }

    #[test]
    fn test_load_from_file_with_yml_extension() {
        let dir = TempDir::new().unwrap();
        let yaml = r#"
rule:
  id: "yml-rule"
  language: "typescript"
  confidence: "LOW"
  message: "YML test"
  explanation: "Testing .yml extension"
  query: "(program) @test"
"#;
        std::fs::write(dir.path().join("test.yml"), yaml).unwrap();
        let config = RuleConfig::load_from_file(dir.path().join("test.yml")).unwrap();
        assert_eq!(config.rule.id, "yml-rule");
    }
}
