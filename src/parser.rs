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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_new() {
        let parser = TsParser::new();
        assert!(parser.is_ok());
    }

    #[test]
    fn test_parse_typescript() {
        let mut parser = TsParser::new().unwrap();
        let tree = parser.parse("const x: number = 1;", "typescript");
        assert!(tree.is_some());
        let tree = tree.unwrap();
        assert!(tree.root_node().to_sexp().contains("program"));
    }

    #[test]
    fn test_parse_typescript_jsx() {
        let mut parser = TsParser::new().unwrap();
        let tree = parser.parse("const el = <div />;", "typescript");
        assert!(tree.is_some());
    }

    #[test]
    fn test_parse_python() {
        let mut parser = TsParser::new().unwrap();
        let tree = parser.parse("x = 1", "python");
        assert!(tree.is_some());
        let tree = tree.unwrap();
        assert!(tree.root_node().to_sexp().contains("module"));
    }

    #[test]
    fn test_parse_invalid_syntax() {
        let mut parser = TsParser::new().unwrap();
        let tree = parser.parse("!!!! @@@ invalid @@@", "typescript");
        if let Some(tree) = tree {
            // Tree-sitter handles errors gracefully — should still parse but with ERROR nodes
            assert!(tree.root_node().has_error());
        }
    }

    #[test]
    fn test_get_language_typescript() {
        let parser = TsParser::new().unwrap();
        let lang = parser.get_language("typescript");
        // Just verify it doesn't crash and returns valid language
        assert!(lang.version() > 0);
    }

    #[test]
    fn test_get_language_python() {
        let parser = TsParser::new().unwrap();
        let lang = parser.get_language("python");
        assert!(lang.version() > 0);
    }
}
