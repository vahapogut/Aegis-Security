use crate::rule::{Rule, Confidence};
use crate::parser::TsParser;
use tree_sitter::{Query, QueryCursor};
use anyhow::{Result, anyhow};
use colored::*;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Violation {
    pub rule: Rule,
    pub file_path: String,
    pub line_number: usize,
    #[serde(skip_serializing)]
    pub snippet: String,
}

/// SARIF result object for a single violation.
#[derive(Debug, Serialize)]
pub struct SarifResult {
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    pub level: String,
    pub message: SarifMessage,
    pub locations: Vec<SarifLocation>,
}

#[derive(Debug, Serialize)]
pub struct SarifMessage {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct SarifLocation {
    #[serde(rename = "physicalLocation")]
    pub physical_location: SarifPhysicalLocation,
}

#[derive(Debug, Serialize)]
pub struct SarifPhysicalLocation {
    #[serde(rename = "artifactLocation")]
    pub artifact_location: SarifArtifactLocation,
    pub region: SarifRegion,
}

#[derive(Debug, Serialize)]
pub struct SarifArtifactLocation {
    pub uri: String,
}

#[derive(Debug, Serialize)]
pub struct SarifRegion {
    #[serde(rename = "startLine")]
    pub start_line: usize,
}

/// Generate a SARIF v2.1.0 JSON report from a list of violations.
pub fn generate_sarif(violations: &[Violation], tool_name: &str) -> serde_json::Value {
    let results: Vec<SarifResult> = violations
        .iter()
        .map(|v| {
            let level = match v.rule.confidence {
                Confidence::HIGH => "error",
                Confidence::MEDIUM => "warning",
                Confidence::LOW => "note",
            };
            SarifResult {
                rule_id: v.rule.id.clone(),
                level: level.to_string(),
                message: SarifMessage {
                    text: format!("{} — {}", v.rule.message, v.rule.explanation),
                },
                locations: vec![SarifLocation {
                    physical_location: SarifPhysicalLocation {
                        artifact_location: SarifArtifactLocation {
                            uri: v.file_path.clone(),
                        },
                        region: SarifRegion {
                            start_line: v.line_number,
                        },
                    },
                }],
            }
        })
        .collect();

    serde_json::json!({
        "version": "2.1.0",
        "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
        "runs": [{
            "tool": {
                "driver": {
                    "name": tool_name,
                    "informationUri": "https://github.com/vahapogut/Aegis-Security",
                    "rules": violations.iter().map(|v| {
                        serde_json::json!({
                            "id": v.rule.id,
                            "shortDescription": { "text": v.rule.message },
                            "fullDescription": { "text": v.rule.explanation },
                            "defaultConfiguration": {
                                "level": match v.rule.confidence {
                                    Confidence::HIGH => "error",
                                    Confidence::MEDIUM => "warning",
                                    Confidence::LOW => "note",
                                }
                            }
                        })
                    }).collect::<Vec<_>>()
                }
            },
            "results": results,
        }]
    })
}

pub struct Engine {
    parser: TsParser,
}

impl Engine {
    pub fn new() -> Result<Self> {
        Ok(Self {
            parser: TsParser::new()?,
        })
    }

    pub fn detect_language(file_path: &str) -> Option<&str> {
        match file_path.rsplit('.').next() {
            Some("ts") | Some("tsx") | Some("js") | Some("jsx") => Some("typescript"),
            Some("py") => Some("python"),
            _ => None,
        }
    }

    pub fn scan_file(&mut self, file_path: &str, rules: &[Rule]) -> Result<Vec<Violation>> {
        let lang = match Self::detect_language(file_path) {
            Some(l) => l,
            None => return Ok(vec![]),
        };

        let source_code = std::fs::read_to_string(file_path)?;
        let tree = self.parser.parse(&source_code, lang)
            .ok_or_else(|| anyhow!("Failed to parse file: {}", file_path))?;
        let root_node = tree.root_node();
        let language = self.parser.get_language(lang);

        let mut violations = Vec::new();
        let mut cursor = QueryCursor::new();

        for rule in rules {
            if rule.language != lang {
                continue;
            }

            let query = match Query::new(&language, &rule.query) {
                Ok(q) => q,
                Err(e) => {
                    eprintln!("{} Failed to compile query for rule '{}': {}", "[WARNING]".yellow(), rule.id, e);
                    continue;
                }
            };

            let mut matches = cursor.matches(&query, root_node, source_code.as_bytes());

            while let Some(m) = matches.next() {
                if let Some(capture) = m.captures.first() {
                    let start_position = capture.node.start_position();
                    let line_number = start_position.row + 1;

                    let start_byte = capture.node.start_byte();
                    let end_byte = capture.node.end_byte();

                    let snippet = if end_byte > start_byte && end_byte <= source_code.len() {
                        source_code[start_byte..end_byte].to_string()
                    } else {
                        "".to_string()
                    };

                    violations.push(Violation {
                        rule: rule.clone(),
                        file_path: file_path.to_string(),
                        line_number,
                        snippet,
                    });
                }
            }
        }

        Ok(violations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::{Rule, Confidence};
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_detect_language_typescript() {
        assert_eq!(Engine::detect_language("src/app.ts"), Some("typescript"));
        assert_eq!(Engine::detect_language("components/Button.tsx"), Some("typescript"));
    }

    #[test]
    fn test_detect_language_javascript() {
        assert_eq!(Engine::detect_language("src/utils.js"), Some("typescript"));
        assert_eq!(Engine::detect_language("pages/index.jsx"), Some("typescript"));
    }

    #[test]
    fn test_detect_language_python() {
        assert_eq!(Engine::detect_language("src/main.py"), Some("python"));
    }

    #[test]
    fn test_detect_language_unknown() {
        assert_eq!(Engine::detect_language("README.md"), None);
        assert_eq!(Engine::detect_language("src/lib.rs"), None);
    }

    #[test]
    fn test_scan_file_no_rules() {
        let mut engine = Engine::new().unwrap();
        let mut tmp = NamedTempFile::new().unwrap();
        write!(tmp, "const x = 1;").unwrap();
        let path = tmp.path().to_str().unwrap().to_string();
        let renamed_path = format!("{}.ts", &path);
        std::fs::rename(&path, &renamed_path).unwrap();

        let violations = engine.scan_file(&renamed_path, &[]).unwrap();
        assert!(violations.is_empty());
    }

    #[test]
    fn test_scan_file_unknown_extension() {
        let mut engine = Engine::new().unwrap();
        let violations = engine.scan_file("README.md", &[]).unwrap();
        assert!(violations.is_empty());
    }

    #[test]
    fn test_scan_file_nonexistent() {
        let mut engine = Engine::new().unwrap();
        let result = engine.scan_file("nonexistent.ts", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_sarif_empty() {
        let sarif = generate_sarif(&[], "aegis");
        assert_eq!(sarif["version"], "2.1.0");
        let results = &sarif["runs"][0]["results"];
        assert!(results.as_array().unwrap().is_empty());
    }

    #[test]
    fn test_sarif_with_violations() {
        let rule = Rule {
            id: "test-rule".into(),
            language: "typescript".into(),
            confidence: Confidence::HIGH,
            message: "Test message".into(),
            explanation: "Test explanation".into(),
            query: "(program) @test".into(),
        };
        let violations = vec![Violation {
            rule,
            file_path: "src/test.ts".into(),
            line_number: 42,
            snippet: "bad_code".into(),
        }];
        let sarif = generate_sarif(&violations, "aegis");
        let results = &sarif["runs"][0]["results"];
        assert_eq!(results[0]["ruleId"], "test-rule");
        assert_eq!(results[0]["level"], "error");
        assert_eq!(results[0]["locations"][0]["physicalLocation"]["region"]["startLine"], 42);
    }

    #[test]
    fn test_sarif_confidence_levels() {
        let make_rule = |c: Confidence| Rule {
            id: "test".into(), language: "typescript".into(),
            confidence: c, message: "m".into(), explanation: "e".into(),
            query: "(program) @test".into(),
        };
        for (confidence, expected_level) in [
            (Confidence::HIGH, "error"),
            (Confidence::MEDIUM, "warning"),
            (Confidence::LOW, "note"),
        ] {
            let violations = vec![Violation {
                rule: make_rule(confidence),
                file_path: "f.ts".into(), line_number: 1, snippet: "".into(),
            }];
            let sarif = generate_sarif(&violations, "aegis");
            assert_eq!(sarif["runs"][0]["results"][0]["level"], expected_level);
        }
    }
}
