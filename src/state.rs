pub mod state{
    extern crate bit_set;
    use tree::tree::{Tree, ChildTree};
    use self::bit_set::BitSet; 

    #[derive(Debug, Clone)]
    pub struct State{
        pub pos: BitSet,
        pub tree: Vec<ChildTree>
    }

    impl State{

        pub fn start(new: Vec<ChildTree>) -> State{
            let mut new = State{pos: BitSet::new(), tree: new};
            new.pos.insert(0);
            new
        }

        pub fn new(new: Vec<ChildTree>) -> State{
            State{pos: BitSet::new(), tree: new}
        }

        pub fn new_child(pos: usize, new: Vec<ChildTree>) -> State {
            let mut new = State{pos: BitSet::new(), tree: new};
            new.pos.insert(pos);
            new
        }

        pub fn new_back(&self, pos: usize, new: Vec<ChildTree>) -> State {
            let mut new_state = State{pos: BitSet::new(), tree: new};
            new_state.pos.insert(pos);
            if self.tree.len() > pos{new_state.tree[pos] = self.tree[pos].clone()}
            new_state
        }

        pub fn set(&mut self, other: State){
            *self = other;
        }

        pub fn is_empty(& self) -> bool{
            self.pos.is_empty()
        }

        pub fn make_leaf(&mut self, c: char, pos: usize, tree: ChildTree){
            self.pos.remove(pos);
            self.pos.insert(pos + 1);
            self.tree[pos + 1] = ChildTree::push_val(Tree::Leaf(c), tree);
            self.tree[pos] = ChildTree::Nil;
        }

        pub fn make_node(&mut self, symbol: usize, prev_tree: ChildTree, child: State){
            for pos in child.pos.iter() {
                match self.tree[pos as usize].clone(){
                    ChildTree::Nil => self.tree[pos as usize] = ChildTree::push_val(Tree::Node{sym: symbol, child: child.tree[pos as usize].clone()}, prev_tree.clone()),
                    ChildTree::Val{ val, prev: _} => match *val{
                        Tree::Leaf(_) => self.tree[pos as usize] = ChildTree::new_val(Tree::Amb{trees: vec![self.tree[pos as usize].clone(), ChildTree::push_val(Tree::Node{sym: symbol, child: child.tree[pos as usize].clone()}, prev_tree.clone())]}),
                        Tree::Node{sym: _, child: _} => self.tree[pos as usize] = ChildTree::new_val(Tree::Amb{trees: vec![self.tree[pos as usize].clone(), ChildTree::push_val(Tree::Node{sym: symbol, child: child.tree[pos as usize].clone()}, prev_tree.clone())]}),
                        Tree::Amb{trees: _} => if let &mut ChildTree::Val{ ref mut val, prev: _} = &mut self.tree[pos as usize] {
                            if let &mut Tree::Amb{ref mut trees} = &mut **val {
                                trees.push(ChildTree::push_val(Tree::Node{sym: symbol, child: child.tree[pos as usize].clone()}, prev_tree.clone()))
                            }
                        }
                    }
                }
            }
            self.pos.union_with(&child.pos);
        }

        pub fn merge(&mut self, other: State){
            for pos in other.pos.iter() {
                match self.tree[pos as usize].clone() {
                    ChildTree::Nil => self.tree[pos as usize] = other.tree[pos as usize].clone(),
                    ChildTree::Val{ val, prev: _} => match *val{
                        Tree::Leaf(_) => self.tree[pos as usize] = ChildTree::new_val(Tree::Amb{trees: vec![self.tree[pos as usize].clone(), other.tree[pos as usize].clone()]}),
                        Tree::Node{sym: _, child: _} => self.tree[pos as usize] = ChildTree::new_val(Tree::Amb{trees: vec![self.tree[pos as usize].clone(), other.tree[pos as usize].clone()]}),
                        Tree::Amb{ trees: _} => if let &mut ChildTree::Val{ ref mut val, prev: _} = &mut self.tree[pos as usize] {
                            if let &mut Tree::Amb{ ref mut trees} = &mut **val {
                                trees.push(other.tree[pos as usize].clone())
                            }
                        }
                    }
                }
            }
            self.pos.union_with(&other.pos);
        }
    }
}