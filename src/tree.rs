pub mod tree{

    #[derive(Debug, Clone)]
    pub enum Tree{
        Leaf(char),
        Node{sym: usize, child: Vec<ChildTree>},
        //Amb{trees: Vec<ChildTree>}
    }

    impl Tree{
        pub fn to_string(&self, symbol: &[&'static str]) -> String {
            match self {
                &Tree::Leaf(c) => format!("{}", c),
                &Tree::Node{ref sym, ref child} => format!("[{}{}]", symbol[*sym], ChildTree::vec_to_string(child, symbol)),
                //&Tree::Amb{ref trees} => format!("[Amb[{}]", trees.iter().fold("".to_string(), |ts, t| format!("{}{}", if ts == "" {ts} else {format!("{},", ts)}, t.to_string(symbol))))
            }
        }
    }

    #[derive(Debug, Clone)] 
    pub struct ChildTree{
        val: Box<Tree>,
        prev: Box<Vec<ChildTree>>
    }
    /*
    pub enum ChildTree{
        Nil,
        Val{val: Box<Tree>, prev: Box<Vec<ChildTree>>},
    }
    */

    impl ChildTree{
        pub fn new_val(tree: Tree) -> ChildTree{
            ChildTree{val: Box::new(tree), prev: Box::new(vec![])}
        }

        pub fn push_val(tree: Tree, prev: Vec<ChildTree>) -> Vec<ChildTree>{
            vec![ChildTree{val: Box::new(tree), prev: Box::new(prev)}]
        }

        pub fn to_string(&self, symbol: &[&'static str]) -> String{
            /*
            match self {
                & ChildTree::Nil => "".to_string(),
                & ChildTree::Val{ ref val, ref prev} => format!("{}{}", Self::vec_to_string(prev, symbol), val.to_string(symbol))
            }
            */
            format!("{}{}", Self::vec_to_string(& self.prev, symbol), self.val.to_string(symbol))
        }

        pub fn vec_to_string(vec: & Vec<ChildTree>, symbol: &[&'static str]) -> String{
            if vec.len() > 1{
                vec.iter().fold("".to_string(), |ts, t| format!("[Amb {}{}]", if ts == "" {ts} else {format!("{},", ts)}, t.to_string(symbol)))
            }else{
                vec.iter().fold("".to_string(), |ts, t| format!("{}{}", if ts == "" {ts} else {format!("{},", ts)}, t.to_string(symbol)))
            }
        }
    }
}