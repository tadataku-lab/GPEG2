extern crate gpeg2;

#[cfg(test)]
mod rules {

    use gpeg2::parser_context::parser_context::ParserContext;
    use gpeg2::tree::tree::Tree;
    use gpeg2::state::state::State;
    use gpeg2::gpeg_parser::gpeg_parser::*;
    use std::cell::RefCell;

    #[test]
    // S <- S' S S / S', S' <- b S' / S'
    fn rules1() {
        let p = ParserContext{
            input: String::from("bbb").into_bytes(),
            rules: vec![
                choice(nonterm(1, nonterm(0, nonterm(0, succ()))), nonterm(1, succ()), succ()),
                choice(ch('b', nonterm(1, succ())), ch('b', succ()), succ())
                ],
            state: RefCell::new(State{pos: 0, tree: Vec::new()})
        };
        p.rules[0](&p);
        assert!(Tree::Node{sym: 0, child: p.state.into_inner().tree}.to_string(&["S", "S'"]) == "[S [S' b [S' b [S' b]]]]");
    }

    #[test]
    // S <- a | ab
    fn rules2() {
        let p = ParserContext{
            input: String::from("ab").into_bytes(),
            rules: vec![
                alt(ch('a', succ()), ch('a', ch('b', succ()))),
                ],
            state: RefCell::new(State{pos: 0, tree: Vec::new()})
        };
        p.rules[0](&p);
        assert!(Tree::Node{sym: 1, child: p.state.into_inner().tree}.to_string(&["Amb", "S"]) == "[S [Amb a a b]]");
    }

    #[test]
    // S <- ab / a
    fn rules3() {
        let p = ParserContext{
            input: String::from("a").into_bytes(),
            rules: vec![
                choice(ch('a', ch('b', succ())), ch('a', succ()),succ()),
                ],
            state: RefCell::new(State{pos: 0, tree: Vec::new()})
        };
        p.rules[0](&p);
        assert!(Tree::Node{sym: 0, child: p.state.into_inner().tree}.to_string(&["S"]) == "[S a]");
    }

    #[test]
    // S <- (a | ab) b
    fn rules4() {
        let p = ParserContext{
            input: String::from("abb").into_bytes(),
            rules: vec![
                alt(ch('a', ch('b', succ())), ch('a', ch('b', ch('b', succ()))))
                ],
            state: RefCell::new(State{pos: 0, tree: Vec::new()})
        };
        p.rules[0](&p);
        assert!(Tree::Node{sym: 1, child: p.state.into_inner().tree}.to_string(&["Amb", "S"]) == "[S [Amb a a b] b]");
    }
}
