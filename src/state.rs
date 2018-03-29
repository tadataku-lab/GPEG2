pub mod state{

    use tree::tree::Tree;
    use std::collections::{BitSet};

    #[derive(Debug, Clone)]
    pub struct State{
        pub pos: i32,
        pub tree: Vec<Tree>
    }

    #[derive(Debug, Clone)]
    pub struct State{
        pub pos: BitSet,
        pub tree: Vec<Vec<Tree>>
    }

    impl State{
        pub fn new() -> State{
            let mut new = State{pos: BitSet::new(), tree: Vec::new()};
            new.pos.insert(0);
            new
        }

        pub fn new_child(pos: usize) -> State {
            let mut new = State{pos: BitSet::new(), tree: Vec::new()};
            new.pos.insert(pos);
            new
        }

        pub fn set(&mut self, other: State){
            *self = other;
        }

        pub fn is_empty(& self) -> bool{
            self.pos.is_empty()
        }

        pub fn make_leaf(&mut self, c: char, pos: usize, tree: Vec<Tree>){
            self.pos.insert(pos + 1);
            self.tree[pos + 1] = tree.push(Tree::Leaf(c));
        }

        pub fn make_node(&mut self, symbol: usize, prev_tree: Vec<Tree>, child: State){
            for pos in child.pos.iter() {
                match self.tree[pos as usize].last_mut() {
                    Some(last) => match last {
                        &mut Tree::Leaf(_) => self.tree[pos as usize] = vec![Tree::Amb{trees: vec![self.tree[pos as usize]]}, prev_tree.push(Tree::Node{sym: symbol, child.tree[pos as usize].clone()})],
                        &mut Tree::Node{sym: _, child: _} => self.tree[pos as usize] = vec![Tree::Amb{trees: vec![self.tree[pos as usize]]}, prev_tree.push(Tree::Node{sym: symbol, child.tree[pos as usize].clone()})],
                        &mut Tree::Amb{trees: ref mut trees} => trees.push(prev_tree.push(Tree::Node{sym: symbol, child.tree[pos as usize].clone()}))
                    },
                    None => self.tree[pos as usize] = prev_tree.push(Tree::Node{sym: symbol, child.tree[pos as usize].clone()})
                }
            }
        }

        pub fn merge(&mut self, mut other: State){
            other.tree.append(&mut self.tree);
            self.tree = vec![Tree::Node{sym: 0, child: other.tree.clone()}];
        }
    }
}