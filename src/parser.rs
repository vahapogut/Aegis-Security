use tree_sitter::{Language, Parser, Tree};
use anyhow::{Result, Context};

pub struct TsParser {
    ts_parser: Parser,
    py_parser: Parser,
}

impl TsParser {
    pub fn new() -> Result<Self> {
        let mut ts_parser = Parser::new();
        ts_parser.set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .context("Failed to set Tree-sitter language to TypeScript")?;

        let mut py_parser = Parser::new();
        py_parser.set_language(&tree_sitter_python::LANGUAGE.into())
            .context("Failed to set Tree-sitter language to Python")?;
        
        Ok(Self { ts_parser, py_parser })
    }

    pub fn parse(&mut self, source_code: &str, lang: &str) -> Option<Tree> {
        match lang {
            "python" => self.py_parser.parse(source_code, None),
            _ => self.ts_parser.parse(source_code, None),
        }
    }

    pub fn get_language(&self, lang: &str) -> Language {
        match lang {
            "python" => tree_sitter_python::LANGUAGE.into(),
            _ => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        }
    }
}
