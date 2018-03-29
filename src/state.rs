pub mod state{
    extern crate bit_set;
    use tree::tree::Tree;
    use self::bit_set::BitSet;

    #[derive(Debug, Clone)]
    pub struct State{
        pub pos: BitSet,
        pub tree: Vec<Vec<Tree>>
    }

    impl State{

        pub fn new(new: Vec<Vec<Tree>>) -> State{
            let mut new = State{pos: BitSet::new(), tree: new};
            new.pos.insert(0);
            new
        }

        pub fn new_child(pos: usize, new: Vec<Vec<Tree>>) -> State {
            let mut new = State{pos: BitSet::new(), tree: new};
            new.pos.insert(pos);
            new
        }

        pub fn new_back(&self, pos: usize, new: Vec<Vec<Tree>>) -> State {
            let mut new = State{pos: BitSet::new(), tree: new};
            new.pos.insert(pos);
            if self.tree.len() > pos{new.tree[pos] = self.tree[pos].clone()}
            new
        }

        pub fn set(&mut self, other: State){
            *self = other;
        }

        pub fn is_empty(& self) -> bool{
            self.pos.is_empty()
        }

        pub fn make_leaf(&mut self, c: char, pos: usize, mut tree: Vec<Tree>){
            self.pos.insert(pos + 1);
            tree.push(Tree::Leaf(c));
            self.tree[pos + 1] = tree;
        }

        pub fn make_node(&mut self, symbol: usize, prev_tree: Vec<Tree>, child: State){
            for pos in child.pos.iter() {
                let mut new_tree = prev_tree.clone();
                new_tree.push(Tree::Node{sym: symbol, child: child.tree[pos as usize].clone()});
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