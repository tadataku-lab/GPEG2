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
            /*
            if p.state.borrow_mut().pos as usize >= p.input.len() {
                false
            }else {
            if p.input[p.state.borrow_mut().pos as usize] == c as u8 { make_leaf(c, p) && e(p) } else {false} 
            }
            */
            let mut new_state = State::new();

            for pos in p.state.borrow().pos.iter() {
                if pos as usize >= p.input.len() {
                    break;
                }else if p.input[pos as usize] == c as u8 {
                    new_state.make_leaf(c, pos, p.state.borrow_mut().tree[pos as usize]);
                }
            }

            p.state.borrow_mut().set(new_state);

            !new_state.is_empty() && e(p)
        })
    }

    pub fn nonterm(symbol: usize, e: Box<Fn(& ParserContext) -> bool>) -> Box<Fn(& ParserContext) -> bool> {
        Box::new(move |p: & ParserContext| -> bool {
            /*
            let prev_tree = p.state.borrow_mut().tree.clone();
            p.state.borrow_mut().tree.clear();
            if p.rules[symbol](p) {make_node(symbol, prev_tree, p) && e(p)} else {false}
            */

            let mut new_state = State::new();
            let old_state = p.state.borrow().clone();

            for pos in old_state.pos.iter() {
                p.state.borrow_mut().set(State::new_child(pos as usize));
                if p.rules[symbol](p) {
                    new_state.make_node(symbol, pos as usize, old_state.tree[pos], p.state.into_inner())
                }
            }

            p.state.borrow_mut().set(new_state);

            !new_state.is_empty() && e(p)

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