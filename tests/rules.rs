extern crate gpeg2;

#[cfg(test)]
mod rules {

    use gpeg2::parser_context::parser_context::ParserContext;
    use gpeg2::gpeg_parser::gpeg_parser::*;

/*
    #[test]
    // S <- S' S S / S', S' <- b S' / S'
    fn rules1() {
        let p = ParserContext::new(
            String::from("bbb").into_bytes(),
            vec![
                choice(nonterm(1, nonterm(0, nonterm(0, succ()))), nonterm(1, succ()), succ()),
                choice(ch('b', nonterm(1, succ())), ch('b', succ()), succ())
            ]
        );
        p.rules[0](&p);
        assert!(p.show_tree(&["S", "S'"]) == "[S [S' b [S' b [S' b]]]]");
    }
*/

    #[test]
    // S <- a | ab
    fn rules2() {
        let p = ParserContext::new(
            String::from("ab").into_bytes(),
            vec![
                alt(ch('a', succ()), ch('a', ch('b', succ()))),
            ]
        );
        p.rules[0](&p);
        assert!(p.show_tree(&["S"]) == "[S a,ab]");
    }

/*
    #[test]
    // S <- ab / a
    fn rules3() {
        let p = ParserContext::new(
            String::from("a").into_bytes(),
            vec![
                choice(ch('a', ch('b', succ())), ch('a', succ()),succ()),
            ]
        );
        p.rules[0](&p);
        assert!(p.show_tree(&["S"]) == "[S a]");
    }
*/

    #[test]
    // S <- (a | ab) b
    fn rules4() {
        let p = ParserContext::new(
            String::from("abb").into_bytes(),
            vec![
                alt(ch('a', ch('b', succ())), ch('a', ch('b', ch('b', succ()))))
            ]
        );
        p.rules[0](&p);
        assert!(p.show_tree(&["S"]) == "[S ab,abb]");
    }

    #[test]
    // S <- S' S S | S', S' <- b S' | b
    fn rules5() {
        let p = ParserContext::new(
            String::from("bb").into_bytes(),
            vec![
                alt(nonterm(1, nonterm(0, nonterm(0, succ()))), nonterm(1, succ())),
                alt(ch('b', nonterm(1, succ())), ch('b', succ()))
            ]
        );
        p.rules[0](&p);
        assert!(p.show_tree(&["S", "S'"]) == "[S [S' b],[S' b[S' b]]]");
    }

    #[test]
    // S <- S' S S | S', S' <- b S' | b
    fn rules6() {
        let p = ParserContext::new(
            String::from("bbb").into_bytes(),
            vec![
                alt(nonterm(1, nonterm(0, nonterm(0, succ()))), nonterm(1, succ())),
                alt(ch('b', nonterm(1, succ())), ch('b', succ()))
            ]
        );
        p.rules[0](&p);
        assert!(p.show_tree(&["S", "S'"]) == "[S [S' b],[S' b[S' b]],[Amb[[S' b][S [S' b]][S [S' b]],[S' b[S' b[S' b]]]]]");
    }

    #[test]
    // S <- S' S S | S', S' <- b S' | b
    fn rules7() {
        let p = ParserContext::new(
            String::from("bbbb").into_bytes(),
            vec![
                alt(nonterm(1, nonterm(0, nonterm(0, succ()))), nonterm(1, succ())),
                alt(ch('b', nonterm(1, succ())), ch('b', succ()))
            ]
        );
        p.rules[0](&p);
        assert!(p.show_tree(&["S", "S'"]) == "[S [S' b],[S' b[S' b]],[Amb[[S' b][S [S' b]][S [S' b]],[S' b[S' b[S' b]]]],[Amb[[S' b][S [S' b]][S [S' b[S' b]]],[Amb[[S' b][S [S' b[S' b]]],[S' b[S' b]][S [S' b]]][S [S' b]],[S' b[S' b[S' b[S' b]]]]]]");
    }
}
