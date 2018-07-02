pub mod state{
    extern crate bit_set;
    use tree::tree::{Tree, ChildTree};
    use self::bit_set::BitSet;
    use std::rc::Rc;
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct State{
        pub pos: BitSet,
        pub tree: HashMap<usize, Rc<ChildTree>>
    }

    impl State{

        pub fn start() -> State{
            Self::new_child(0)
        }

        pub fn new() -> State{
            State{pos: BitSet::new(), tree: HashMap::new()}
        }

        pub fn new_child(pos: usize) -> State {
            let mut new = Self::new();
            new.pos.insert(pos);
            new.tree.insert(pos, Rc::new(ChildTree::Nil));
            new
        }

        pub fn new_back(&self, pos: usize) -> State {
            let mut new_state = Self::new();
            new_state.pos.insert(pos);
            new_state.tree.insert(pos, self.tree[&pos].clone());
            new_state
        }

        pub fn set(&mut self, other: State){
            *self = other;
        }

        pub fn is_empty(& self) -> bool{
            self.pos.is_empty()
        }

        pub fn make_leaf(&mut self, c: char, pos: usize, tree: Rc<ChildTree>){
            self.pos.insert(pos + 1);
            self.tree.insert(pos + 1, ChildTree::push_val(Tree::new_leaf(c), tree));
        }

        pub fn make_node(&mut self, symbol: usize, prev_tree: Rc<ChildTree>, child: State){

            for pos in child.pos.difference(&self.pos) {
                self.tree.insert(pos , ChildTree::push_val(Tree::new_node(symbol, child.tree[&pos].clone()), prev_tree.clone()));
            }

            for pos in child.pos.intersection(&self.pos) {
                let buf = self.tree[&pos].clone();
                self.tree.insert(pos , ChildTree::make_amb(ChildTree::push_val(Tree::new_node(symbol, child.tree[&pos].clone()), prev_tree.clone()), buf));
            }

            self.pos.union_with(&child.pos);
            
        }

        pub fn merge(&mut self, other: State){

            for pos in other.pos.difference(&self.pos){
                self.tree.insert(pos , other.tree[&pos].clone());
            }
            
            for pos in other.pos.intersection(&self.pos) {
                let buf = self.tree[&pos].clone();
                self.tree.insert(pos , ChildTree::make_amb(other.tree[&pos].clone(), buf));
            }
            
            self.pos.union_with(&other.pos);
        }

        pub fn show_tree(&self, symbol: &[&'static str]) -> String{
            format!("[{} {}]", symbol[0], self.tree.iter().fold("".to_string(), |ts, (_pos, t)| format!("{}{}", if ts == "" {ts} else {format!("{},", ts)}, t.to_string(symbol))))
        }
    }
}