use crate::rule::Rule;
use crate::parser::TsParser;
use tree_sitter::{Query, QueryCursor, StreamingIterator};
use anyhow::{Result, anyhow};
use colored::*;

#[derive(Debug)]
pub struct Violation {
    pub rule: Rule,
    pub file_path: String,
    pub line_number: usize,
    pub snippet: String,
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
            Some("ts") | Some("tsx") => Some("typescript"),
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
