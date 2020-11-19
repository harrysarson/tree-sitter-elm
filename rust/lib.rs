#![warn(clippy::pedantic)]

use tree_sitter::Language;

extern "C" {
    #[link_name = "tree_sitter_elm"]
    fn raw_tree_sitter_elm() -> Language;
}

#[must_use]
pub fn tree_sitter_elm() -> Language {
    unsafe { raw_tree_sitter_elm() }
}

#[cfg(test)]
mod tests {
    use tree_sitter::Parser;

    #[test]
    fn smoke() {
        let source_code = "test : Test.Test";
        let tree = {
            let mut parser = Parser::new();
            let language = super::tree_sitter_elm();
            parser.set_language(language).unwrap();
            parser.parse(source_code, None).unwrap()
        };
        let root_node = tree.root_node();

        assert_eq!(root_node.kind(), "file");
        assert_eq!(root_node.start_position().column, 0);
        assert_eq!(root_node.end_position().column, 16);
    }
}