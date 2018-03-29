extern crate gpeg2;

use gpeg2::parser_context::parser_context::ParserContext;
use gpeg2::gpeg_parser::gpeg_parser::*;

fn main() {
    let p = ParserContext::new(
        String::from("bb").into_bytes(),
        vec![
            alt(nonterm(1, nonterm(0, nonterm(0, succ()))), nonterm(1, succ())),
            alt(ch('b', nonterm(1, succ())), ch('b', succ()))
        ]
    );
    p.rules[0](&p);
    p.show_tree(&["S", "S'"]);
}
