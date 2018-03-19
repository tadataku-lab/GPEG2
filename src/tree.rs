pub mod tree{
    #[derive(Debug, Clone)]
    pub enum Tree{
        Leaf(char),
        Node{sym: usize, child: Vec<Tree>}
    }

    impl Tree{
        pub fn to_string(&self, symbol: &[&'static str]) -> String {
            match self {
                &Tree::Leaf(c) => format!("{}", c),
                &Tree::Node{ref sym, ref child} => format!("[{}{}]", symbol[*sym], child.iter().fold("".to_string(), |ts, t| format!("{} {}",ts, t.to_string(symbol)))),
            }
        }
    }
}