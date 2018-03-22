pub mod gpeg_parser{

    use parser_context::parser_context::ParserContext;
    use tree::tree::Tree;
    use std::cell::RefCell;

    fn make_leaf(c: char, p: & ParserContext) -> bool{
        let mut mut_child = p.tree.borrow_mut();
        mut_child.push(Tree::Leaf(c));
        true
    }

    fn make_node(sym: usize, prev: RefCell<Vec<Tree>>, p: & ParserContext) -> bool {
        {
            let mut mut_prev = prev.borrow_mut();
            let moved_tree = p.tree.clone();
            mut_prev.push(Tree::Node{sym: sym, child: moved_tree.into_inner()});
        }
        {
            let mut mut_child = p.tree.borrow_mut();
            mut_child.clear();
            mut_child.append(&mut prev.into_inner()); 
        }
        true
    }

    fn next1(p: & ParserContext) -> bool{
        p.pos.set(p.pos.get() + 1);
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
            if p.pos.get() as usize >= p.input.len() {
                false
            }else {
            if p.input[p.pos.get() as usize] == c as u8 { make_leaf(c, p) && next1(p) && e(p) } else {false} 
            }
        })
    }

    pub fn nonterm(symbol: usize, e: Box<Fn(& ParserContext) -> bool>) -> Box<Fn(& ParserContext) -> bool> {
        Box::new(move |p: & ParserContext| -> bool {
            let prev_tree = p.tree.clone();
            p.tree.borrow_mut().clear();
            if p.rules[symbol](p) {make_node(symbol, prev_tree, p) && e(p)} else {false}
        })
    }

    pub fn choice(left: Box<Fn(& ParserContext) -> bool>, right: Box<Fn(& ParserContext) -> bool>, e: Box<Fn(& ParserContext) -> bool>) -> Box<Fn(& ParserContext) -> bool> {
        Box::new(move |p: & ParserContext| -> bool {
            let back_pos = p.pos.get();
            let mut back_tree = p.tree.clone();
            if left(p) { e(p) } else{ 
                p.pos.set(back_pos);
                {
                    let mut prev_tree = p.tree.borrow_mut();
                    prev_tree.clear();
                    prev_tree.append(&mut back_tree.into_inner());
                }
                right(p) && e(p)
            } 
        })
    }

    pub fn alt(left: Box<Fn(& ParserContext) -> bool>, right: Box<Fn(& ParserContext) -> bool>) -> Box<Fn(& ParserContext) -> bool> {
        Box::new(move |p: & ParserContext| -> bool {
            //let right_p = p.clone();
            true
        })
    }
}