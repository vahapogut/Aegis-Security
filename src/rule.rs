use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::{Result, Context};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Confidence {
    HIGH,
    MEDIUM,
    LOW,
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
