extern crate gpeg2;

use std::cell::Cell;
use std::cell::RefCell;
use gpeg2::parser_context::parser_context::ParserContext;
use gpeg2::tree::tree::Tree;
use gpeg2::gpeg_parser::gpeg_parser::*;

fn e0() -> Box<Fn(& ParserContext) -> bool> {
    choice(nonterm(1, nonterm(0, nonterm(0, succ()))), nonterm(1, succ()), succ())
}

fn e1() -> Box<Fn(& ParserContext) -> bool> {
    choice(ch('b', nonterm(1, succ())), ch('b', succ()), succ())
}

fn main() {
    let p = ParserContext{ input: String::from("bbb").into_bytes(), rules: vec![e0(),e1()],pos: Cell::new(0), tree: RefCell::new(Vec::new())};
    //println!("{}", char1(&mut p, 'a' as u8) && char1(&mut p, 'b' as u8))
    println!("{}", p.rules[0](&p));
    println!("{}", Tree::Node{sym: 0, child: p.tree.into_inner()}.to_string(&["S", "S'"]));
}
