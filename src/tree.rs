pub mod tree{

    #[derive(Debug, Clone)]
    pub enum Tree{
        Leaf(char),
        Node{sym: usize, child: ChildTree},
        Amb{trees: ChildTree}
    }

    impl Tree{
        pub fn to_string(&self, symbol: &[&'static str]) -> String {
            match self {
                &Tree::Leaf(c) => format!("{}", c),
                &Tree::Node{ref sym, ref child} => format!("[{}{}]", symbol[*sym], child.to_string(symbol)),
                &Tree::Amb{ref trees} => format!("[Amb[{}]", trees.to_string(symbol))
            }
        }

        pub fn make_amb(& self, trees: ChildTree, prev: ChildTree) -> ChildTree{
            match self {
                & Tree::Amb {trees: _} => ChildTree::Val{val: Box::new(Tree::Amb{trees: trees}), prev: Box::new(prev)},
                _ => ChildTree::Val{val: Box::new(Tree::Amb{trees: trees}), prev: Box::new(prev)}
            }
        } 
    }

    #[derive(Debug, Clone)] 
    pub enum ChildTree{
        Nil,
        Val{val: Box<Tree>, prev: Box<ChildTree>},
    }
    

    impl ChildTree{
        pub fn new_val(tree: Tree) -> ChildTree{
            ChildTree::Val{val: Box::new(tree), prev: Box::new(ChildTree::Nil)}
        }

        pub fn push_val(tree: Tree, prev: ChildTree) -> ChildTree{
            ChildTree::Val{val: Box::new(tree), prev: Box::new(prev)}
        }

        pub fn to_string(&self, symbol: &[&'static str]) -> String{
            match self {
                & ChildTree::Nil => "".to_string(),
                & ChildTree::Val{ ref val, ref prev} => format!("{}{}", prev.to_string(symbol), val.to_string(symbol))
            }
        }

    }
}