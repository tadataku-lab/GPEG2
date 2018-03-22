pub mod gpeg_parser{

    use parser_context::parser_context::ParserContext;
    use tree::tree::Tree;

    fn make_leaf(c: char, p: & ParserContext) -> bool{
        p.state.borrow_mut().make_leaf(c);
        true
    }

    fn make_node(sym: usize, mut prev: Vec<Tree>, p: & ParserContext) -> bool {
        prev.push(Tree::Node{sym: sym, child: p.state.borrow_mut().tree.clone()});
        {
            let mut mut_child = p.state.borrow_mut();
            mut_child.tree.clear();
            mut_child.tree.append(&mut prev); 
        }
        true
    }

    #[allow(unused_variables)]
    pub fn succ() -> Box<Fn(& ParserContext) -> bool> {
        Box::new(move |p: & ParserContext| -> bool {
            true
        })
    }

    pub fn ch(c: char, e: Box<Fn(& ParserContext) -> bool>) -> Box<Fn(& ParserContext) -> bool> {
        Box::new(move |p: & ParserContext| -> bool {
            if p.state.borrow_mut().pos as usize >= p.input.len() {
                false
            }else {
            if p.input[p.state.borrow_mut().pos as usize] == c as u8 { make_leaf(c, p) && e(p) } else {false} 
            }
        })
    }

    pub fn nonterm(symbol: usize, e: Box<Fn(& ParserContext) -> bool>) -> Box<Fn(& ParserContext) -> bool> {
        Box::new(move |p: & ParserContext| -> bool {
            let prev_tree = p.state.borrow_mut().tree.clone();
            p.state.borrow_mut().tree.clear();
            if p.rules[symbol](p) {make_node(symbol, prev_tree, p) && e(p)} else {false}
        })
    }

    pub fn choice(left: Box<Fn(& ParserContext) -> bool>, right: Box<Fn(& ParserContext) -> bool>, e: Box<Fn(& ParserContext) -> bool>) -> Box<Fn(& ParserContext) -> bool> {
        Box::new(move |p: & ParserContext| -> bool {
            let back_state = p.state.clone();
            if left(p) { e(p) } else{ 
                p.state.borrow_mut().set(back_state.into_inner());
                right(p) && e(p)
            } 
        })
    }

    pub fn alt(left: Box<Fn(& ParserContext) -> bool>, right: Box<Fn(& ParserContext) -> bool>) -> Box<Fn(& ParserContext) -> bool> {
        Box::new(move |p: & ParserContext| -> bool {
            let back_state = p.state.clone();
            if left(p) {
                let left_state = p.state.clone();
                p.state.borrow_mut().set(back_state.into_inner());
                if right(p) {
                    p.state.borrow_mut().merge(left_state.into_inner());
                    true
                } else{
                    p.state.borrow_mut().set(left_state.into_inner());
                    true
                }
            } else{
                p.state.borrow_mut().set(back_state.into_inner());
                right(p)
            }
        })
    }
}