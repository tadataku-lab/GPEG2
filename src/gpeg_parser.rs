pub mod gpeg_parser{

    use parser_context::parser_context::ParserContext;
    use state::state::State;
    use memo::memo::Memo;

    #[allow(unused_variables)]
    pub fn succ() -> Box<Fn(& ParserContext) -> bool> {
        Box::new(move |p: & ParserContext| -> bool {
            true
        })
    }

    pub fn ch(c: char, e: Box<Fn(& ParserContext) -> bool>) -> Box<Fn(& ParserContext) -> bool> {
        Box::new(move |p: & ParserContext| -> bool {

            let mut new_state = State::new();
            let old_state = p.state.borrow().clone();
            
            for pos in old_state.pos.iter() {
                if pos as usize >= p.input.len() {
                    break;
                }else if p.input[pos as usize] == c as u8 {
                    new_state.make_leaf(c, pos, old_state.tree[&pos].clone());
                }
            }

            {
                p.state.borrow_mut().set(new_state);
            }
            if !p.state.borrow().is_empty() { e(p) } else {false}
            
        })
    }

    pub fn nonterm(symbol: usize, e: Box<Fn(& ParserContext) -> bool>) -> Box<Fn(& ParserContext) -> bool> {
        Box::new(move |p: & ParserContext| -> bool {
            
            let mut new_state = State::new();
            let old_state = p.state.borrow().clone();

            for pos in old_state.pos.iter() {

                match p.lookup(pos, symbol) {
                    Memo::Succ(memo) => new_state.make_node(symbol, old_state.tree[&pos].clone(), memo),
                    Memo::Fail => (),
                    Memo::Nil => {
                        {
                            p.state.borrow_mut().set(State::new_child(pos));
                        }
                        if p.rules[symbol](p) {
                            new_state.make_node(symbol, old_state.tree[&pos].clone(), p.state.borrow().clone());
                            p.succ_memo(pos, symbol, p.state.borrow().clone());
                        }else{
                            p.fail_memo(pos, symbol);
                        }
                    },
                }
                
            }

            {
                p.state.borrow_mut().set(new_state);
            }

            if !p.state.borrow().is_empty() { e(p) } else {false}

        })
    }

    pub fn choice(left: Box<Fn(& ParserContext) -> bool>, right: Box<Fn(& ParserContext) -> bool>, e: Box<Fn(& ParserContext) -> bool>) -> Box<Fn(& ParserContext) -> bool> {
        Box::new(move |p: & ParserContext| -> bool {
            let mut new_state = State::new();
            let old_state = p.state.borrow().clone();
            for pos in old_state.pos.iter() {
                let back_state = p.state.borrow().new_back(pos);
                {
                    p.state.borrow_mut().set(back_state.clone());
                }
                if left(p) {
                    new_state.merge(p.state.borrow().clone());
                } else{
                    {
                        p.state.borrow_mut().set(back_state);
                    }
                    if right(p) { new_state.merge(p.state.borrow().clone())}
                }
            }

            {
                p.state.borrow_mut().set(new_state);
            }

            if !p.state.borrow().is_empty() { e(p) } else {false}
        })
    }

    pub fn alt(left: Box<Fn(& ParserContext) -> bool>, right: Box<Fn(& ParserContext) -> bool>) -> Box<Fn(& ParserContext) -> bool> {
        Box::new(move |p: & ParserContext| -> bool {
            let mut new_state = State::new();
            let old_state = p.state.borrow().clone();
            for pos in old_state.pos.iter() {
                let back_state = p.state.borrow().new_back(pos);
                {
                    p.state.borrow_mut().set(back_state.clone());
                }
                if left(p) {
                    let left_state = p.state.borrow().clone();
                    {
                        p.state.borrow_mut().set(back_state);
                    }
                    if right(p) {
                        new_state.merge(left_state);
                        new_state.merge(p.state.borrow().clone());
                    } else{
                        new_state.merge(left_state);
                    }
                } else{
                    {
                        p.state.borrow_mut().set(back_state);
                    }
                    if right(p) { new_state.merge(p.state.borrow().clone())}
                }
            }

            {
                p.state.borrow_mut().set(new_state);
            }

            !p.state.borrow().is_empty()
        })
    }
}