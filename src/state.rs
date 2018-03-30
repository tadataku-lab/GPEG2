pub mod state{
    extern crate bit_set;
    use tree::tree::{Tree, ChildTree};
    use self::bit_set::BitSet;
    use std::rc::Rc;

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

        pub fn make_leaf(&mut self, c: char, pos: usize, mut tree: ChildTree){
            self.pos.remove(pos);
            self.pos.insert(pos + 1);
            self.tree[pos + 1] = ChildTree::Val{val: Tree::Leaf(c), tree: tree};
            self.tree[pos] = ChildTree::Nil;
        }

        pub fn make_node(&mut self, symbol: usize, prev_tree: ChildTree, child: State){
            for pos in child.pos.iter() {
                let new_tree = ChildTree::Val{val: Tree::Node{sym: symbol, child: child.tree[pos as usize].clone()}, prev: Rc::new(prev_tree.clone())};
                match self.tree[pos as usize].clone().last() {
                    Some(last) => match last {
                        & Tree::Nil => self.tree[pos as usize] = new_tree,
                        & Tree::Leaf(_) => self.tree[pos as usize] = vec![Tree::Amb{trees: vec![self.tree[pos as usize].clone(), new_tree]}],
                        & Tree::Node{sym: _, child: _} => self.tree[pos as usize] = vec![Tree::Amb{trees: vec![self.tree[pos as usize].clone(), new_tree]}],
                        & Tree::Amb{trees: _} => if let Some(&mut Tree::Amb{ref mut trees}) = self.tree[pos as usize].last_mut() {
                            trees.push(new_tree);
                        }
                    },
                    None => self.tree[pos as usize] = new_tree
                }
            }
            self.pos.union_with(&child.pos);
        }

        pub fn merge(&mut self, other: State){
            for pos in other.pos.iter() {
                match self.tree[pos as usize].clone().last() {
                    Some(last) => match last {
                        & Tree::Nil => self.tree[pos as usize] = other.tree[pos as usize].clone(),
                        & Tree::Leaf(_) => self.tree[pos as usize] = vec![Tree::Amb{trees: vec![self.tree[pos as usize].clone(), other.tree[pos as usize].clone()]}],
                        & Tree::Node{sym: _, child: _} => self.tree[pos as usize] = vec![Tree::Amb{trees: vec![self.tree[pos as usize].clone(), other.tree[pos as usize].clone()]}],
                        & Tree::Amb{trees: _} =>  if let Some(&mut Tree::Amb{ref mut trees}) = self.tree[pos as usize].last_mut() {
                            trees.push(other.tree[pos as usize].clone());
                        }
                    },
                    None => self.tree[pos as usize] = other.tree[pos as usize].clone()
                }
            }
            self.pos.union_with(&other.pos);
        }
    }
}