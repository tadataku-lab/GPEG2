extern crate gpeg2;

use std::cell::Cell;
use std::cell::RefCell;
use gpeg2::parser_context::parser_context::ParserContext;
use gpeg2::tree::tree::Tree;
use gpeg2::gpeg_parser::gpeg_parser::*;

fn main() {
    let p = ParserContext{
        input: String::from("a").into_bytes(),
        rules: vec![
            choice(ch('a', ch('b', succ())), ch('a', succ()),succ()),
            ],
        pos: Cell::new(0),
        tree: RefCell::new(Vec::new())
    };
    p.rules[0](&p);
    println!("{}", Tree::Node{sym: 0, child: p.tree.into_inner()}.to_string(&["S"]));
}
