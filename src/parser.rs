use tree_sitter::{Language, Parser, Tree};
use anyhow::{Result, Context};

pub struct TsParser {
    parser: Parser,
}

impl TsParser {
    pub fn new() -> Result<Self> {
        let mut parser = Parser::new();
        let language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT;
        parser.set_language(&language.into())
            .context("Failed to set Tree-sitter language to TypeScript")?;
        
        Ok(Self { parser })
    }

    pub fn parse(&mut self, source_code: &str) -> Option<Tree> {
        self.parser.parse(source_code, None)
    }

    pub fn get_language(&self) -> Language {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
    }
}
