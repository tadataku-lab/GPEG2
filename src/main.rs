extern crate gpeg2;

use gpeg2::parser_context::parser_context::ParserContext;
use gpeg2::gpeg_parser::gpeg_parser::*;
use std::time::{Instant};
use std::env;

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

    let args: Vec<String> = env::args().collect();

    let mut many_b = "".to_string();

    for _ in 0..args[1].parse().unwrap(){
        many_b = format!("{}{}", many_b, "b")
    }

    // S = S S S | S S | b
    #[allow(unused_variables)]
    let fullamb = vec![
            alt(nonterm(1, nonterm(0, nonterm(0, succ()))), nonterm(1, succ())),
            alt(ch('b', nonterm(1, succ())), ch('b', succ()))
        ];
    
    // S = S S S / S S | b
    #[allow(unused_variables)]
    let order1 = vec![
            choice(nonterm(1, nonterm(0, nonterm(0, succ()))), nonterm(1, succ()), succ()),
            alt(ch('b', nonterm(1, succ())), ch('b', succ()))
        ];
    
    // S = S S S | S S / b
    #[allow(unused_variables)]
    let order2 = vec![
            alt(nonterm(1, nonterm(0, nonterm(0, succ()))), nonterm(1, succ())),
            choice(ch('b', nonterm(1, succ())), ch('b', succ()), succ())
        ];
    
    // S = S S S / S S / b
    #[allow(unused_variables)]
    let determin = vec![
            choice(nonterm(1, nonterm(0, nonterm(0, succ()))), nonterm(1, succ()), succ()),
            choice(ch('b', nonterm(1, succ())), ch('b', succ()), succ())
        ];

    let p = ParserContext::new(
        many_b.into_bytes(),
        order1
    );

    measure!({
        p.rules[0](&p);
    });
    //println!("{}", p.show_tree(&["S", "S'"]));
    //println!("{:?}", p.bench.borrow());
}
