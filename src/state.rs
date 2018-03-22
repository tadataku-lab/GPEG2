pub mod state{

    use tree::tree::Tree;

    #[derive(Debug, Clone)]
    pub struct State{
        pub pos: i32,
        pub tree: Vec<Tree>
    }

    impl State{
        pub fn set(&mut self, other: State){
            *self = other;
        }

        pub fn make_leaf(&mut self, c: char){
            self.pos += 1;
            self.tree.push(Tree::Leaf(c));
        }

        pub fn merge(&mut self, mut other: State){
            other.tree.append(&mut self.tree);
            self.tree = vec![Tree::Node{sym: 0, child: other.tree.clone()}];
        }
    }
}