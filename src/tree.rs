pub mod tree{

    #[derive(Debug, Clone)]
    pub enum Tree{
        Leaf(char),
        Node{sym: usize, child: Box<ChildTree>},
        Amb{trees: Box<ChildTree>}
    }

    impl Tree{
        pub fn to_string(&self, symbol: &[&'static str]) -> String {
            match self {
                &Tree::Leaf(c) => format!("{}", c),
                &Tree::Node{ref sym, ref child} => format!("[{}{}]", symbol[*sym], child.to_string(symbol)),
                &Tree::Amb{ref trees} => format!("[Amb[{}]", trees.to_string(symbol))
            }
        }

        pub fn new_leaf(c: char) -> Box<Tree>{
            Box::new(Tree::Leaf(c))
        }

        pub fn new_node(sym: usize, child: Box<ChildTree>) -> Box<Tree>{
            Box::new(Tree::Node{sym: sym, child: child})
        }

        pub fn new_amb(trees: Box<ChildTree>) -> Box<Tree>{
            Box::new(Tree::Amb{trees: trees})
        }
    }

    #[derive(Debug, Clone)] 
    pub enum ChildTree{
        Nil,
        Val{val: Box<Tree>, prev: Box<ChildTree>},
    }
    

    impl ChildTree{
        pub fn new_val(tree: Box<Tree>) -> Box<ChildTree>{
            Box::new(ChildTree::Val{val: tree, prev: Box::new(ChildTree::Nil)})
        }

        pub fn push_val(tree: Box<Tree>, prev: Box<ChildTree>) -> Box<ChildTree>{
            Box::new(ChildTree::Val{val: tree, prev: prev})
        }

        pub fn to_string(&self, symbol: &[&'static str]) -> String{
            match self {
                & ChildTree::Nil => "".to_string(),
                & ChildTree::Val{ ref val, ref prev} => format!("{}{}", prev.to_string(symbol), val.to_string(symbol))
            }
        }

        pub fn make_amb(trees: Box<ChildTree>, prev: Box<ChildTree>) -> Box<ChildTree>{
            match *prev {
                ChildTree::Nil => trees,
                ChildTree::Val{val: _, prev: _} => ChildTree::push_val(Tree::new_amb(trees), prev),
            }
        }

    }
}