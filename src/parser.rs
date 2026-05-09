use tree_sitter::{Language, Parser, Tree};
use anyhow::Result;

pub struct TsParser {
    ts_parser: Parser,
    py_parser: Parser,
}

impl TsParser {
    pub fn new() -> Result<Self> {
        let mut ts_parser = Parser::new();
        let ts_lang: Language = tree_sitter_typescript::language_typescript();
        ts_parser.set_language(&ts_lang)
            .map_err(|e| anyhow::anyhow!("Failed to set TypeScript language: {:?}", e))?;

        let mut py_parser = Parser::new();
        let py_lang: Language = tree_sitter_python::language();
        py_parser.set_language(&py_lang)
            .map_err(|e| anyhow::anyhow!("Failed to set Python language: {:?}", e))?;
        
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
            "python" => tree_sitter_python::language(),
            _ => tree_sitter_typescript::language_typescript(),
        }
    }
}
