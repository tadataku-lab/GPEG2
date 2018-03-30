extern crate gpeg2;

use gpeg2::parser_context::parser_context::ParserContext;
use gpeg2::gpeg_parser::gpeg_parser::*;
use std::time::{Instant};

macro_rules! measure {
  ( $x:expr) => {
    {
      let start = Instant::now();
      let result = $x;
      let end = start.elapsed();
      println!("{}.{:03}[sec]", end.as_secs(), end.subsec_nanos() / 1_000_000);
      result
    }
  };
}

fn main() {
    let p = ParserContext::new(
        String::from("bbb").into_bytes(),
        vec![
            alt(nonterm(1, nonterm(0, nonterm(0, succ()))), nonterm(1, succ())),
            alt(ch('b', nonterm(1, succ())), ch('b', succ()))
        ]
    );

    measure!({
        p.rules[0](&p);
    });
    println!("{}", p.show_tree(&["S", "S'"])); 
}
