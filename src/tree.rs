pub mod tree{

    use std::rc::Rc;

    #[derive(Debug, Clone)]
    pub enum Tree{
        Leaf(char),
        Node{sym: usize, child: Rc<ChildTree>},
        Amb{trees: Rc<ChildTree>}
    }

    impl Tree{
        pub fn to_string(&self, symbol: &[&'static str]) -> String {
            match self {
                &Tree::Leaf(c) => format!("{}", c),
                &Tree::Node{ref sym, ref child} => format!("[{}{}]", symbol[*sym], child.to_string(symbol)),
                &Tree::Amb{ref trees} => format!("[Amb[{}]", trees.to_string(symbol))
            }
        }

        pub fn new_leaf(c: char) -> Rc<Tree>{
            Rc::new(Tree::Leaf(c))
        }

        pub fn new_node(sym: usize, child: Rc<ChildTree>) -> Rc<Tree>{
            Rc::new(Tree::Node{sym: sym, child: child})
        }

        pub fn new_amb(trees: Rc<ChildTree>) -> Rc<Tree>{
            Rc::new(Tree::Amb{trees: trees})
        }
    }

    #[derive(Debug, Clone)] 
    pub enum ChildTree{
        Nil,
        Val{val: Rc<Tree>, prev: Rc<ChildTree>},
    }
    

    impl ChildTree{
        pub fn new_val(tree: Rc<Tree>) -> Rc<ChildTree>{
            Rc::new(ChildTree::Val{val: tree, prev: Rc::new(ChildTree::Nil)})
        }

        pub fn push_val(tree: Rc<Tree>, prev: Rc<ChildTree>) -> Rc<ChildTree>{
            Rc::new(ChildTree::Val{val: tree, prev: prev})
        }

        pub fn to_string(&self, symbol: &[&'static str]) -> String{
            match self {
                & ChildTree::Nil => "".to_string(),
                & ChildTree::Val{ ref val, ref prev} => format!("{}{}", prev.to_string(symbol), val.to_string(symbol))
            }
        }

        pub fn make_amb(trees: Rc<ChildTree>, prev: Rc<ChildTree>) -> Rc<ChildTree>{
            match *prev {
                ChildTree::Nil => trees,
                ChildTree::Val{val: _, prev: _} => ChildTree::push_val(Tree::new_amb(trees), prev),
            }
        }

    }
}