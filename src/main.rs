extern crate gpeg2;

use gpeg2::parser_context::parser_context::ParserContext;
use gpeg2::gpeg_parser::gpeg_parser::*;

fn main() {
    let p = ParserContext::new(
        String::from("ab").into_bytes(),
        vec![
            alt(ch('a', succ()), ch('a', ch('b', succ()))),
        ]
    );
    p.rules[0](&p);
    println!("[{}{}]", "S", p.state.into_inner().tree.iter().fold("".to_string(), |ts, t| format!("{} {}", ts, format!("[{}]", t.iter().fold("".to_string(), |ts, t| format!("{}{}",ts, t.to_string(&["Amb", "S"])))))));
}
