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
        }

        pub fn make_node(&mut self, symbol: usize, prev_tree: ChildTree, child: State){
            
            for pos in child.pos.iter() {
                self.tree[pos as usize] = match & self.tree[pos as usize] {
                    & ChildTree::Nil => ChildTree::push_val(Tree::Node{sym: symbol, child: child.tree[pos as usize].clone()}, prev_tree.clone()),
                    & ChildTree::Val{ref val, prev: _} => val.make_amb(ChildTree::push_val(Tree::Node{sym: symbol, child: child.tree[pos as usize].clone()}, prev_tree.clone()), self.tree[pos as usize].clone()),
                }
            }
            
            self.pos.union_with(&child.pos);
        }

        pub fn merge(&mut self, other: State){
            
            for pos in other.pos.iter() {
                self.tree[pos as usize] = match & self.tree[pos as usize] {
                    & ChildTree::Nil => other.tree[pos as usize].clone(),
                    & ChildTree::Val{ref val, prev: _} => val.make_amb(other.tree[pos as usize].clone(), self.tree[pos as usize].clone()),
                }
            }
            
            self.pos.union_with(&other.pos);
        }

        pub fn show_tree(&self, symbol: &[&'static str]) -> String{
            format!("[{} {}]", symbol[0], self.tree.iter().fold("".to_string(), |ts, t| format!("{}{}", if ts == "" {ts} else {format!("{},", ts)}, t.to_string(symbol))))
        }
    }
}