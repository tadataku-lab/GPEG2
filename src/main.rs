extern crate gpeg2;

use std::cell::RefCell;
use gpeg2::parser_context::parser_context::ParserContext;
use gpeg2::tree::tree::Tree;
use gpeg2::state::state::State;
use gpeg2::gpeg_parser::gpeg_parser::*;

fn main() {
    let p = ParserContext{
        input: String::from("ab").into_bytes(),
        rules: vec![
            alt(ch('a', succ()), ch('a', ch('b', succ()))),
            ],
        state: RefCell::new(State{pos: 0, tree: Vec::new()})
    };
    p.rules[0](&p);
    println!("{}", Tree::Node{sym: 1, child: p.state.into_inner().tree}.to_string(&["Amb", "S"]));
}
