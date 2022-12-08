use tree_sitter::Parser;
use tree_sitter_traversal::{traverse_tree, Order};

pub fn parser() {
    let mut parser = Parser::new();
    let code = r#"
    use std::collections::HashMap;

    fn main() {
        let mut map = HashMap::new();
        map.insert("Hello", "World");
        dbg!(map);
        let mut set = std::collections::HashSet::new();
        set.insert("Hello");
        dbg!(set);
    }
"#;
    parser
        .set_language(tree_sitter_rust::language())
        .expect("Error loading rust grammar");
    let tree = parser.parse(code, None).unwrap();
    let code_vec = code.chars().map(String::from).collect::<Vec<_>>();
    for o in traverse_tree(&tree, Order::Pre) {
        let range = o.range();
        let expr = code_vec[range.start_byte..range.end_byte].to_vec().join("");
        // dbg!(o.kind());
        println!("{expr}");
        // dbg!(expr);
        // dbg!(o.next_sibling());
        // dbg!(o.prev_sibling());
        // dbg!(o.named_child_count());
        // dbg!(o.to_sexp());
    }
    let node = tree.root_node();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_works() {
        parser();
        assert_eq!(5, 5);
    }
}
