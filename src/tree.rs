pub mod tree{
    #[derive(Debug, Clone)]
    pub enum Tree{
        Nil,
        Leaf(char),
        Node{sym: usize, child: Vec<Tree>},
        Amb{trees: Vec<Vec<Tree>>}
    }

    impl Tree{
        pub fn to_string(&self, symbol: &[&'static str]) -> String {
            match self {
                &Tree::Nil => "".to_string(),
                &Tree::Leaf(c) => format!("{}", c),
                &Tree::Node{ref sym, ref child} => format!("[{}{}]", symbol[*sym], child.iter().fold("".to_string(), |ts, t| format!("{} {}",ts, t.to_string(symbol)))),
                &Tree::Amb{ref trees} => format!("[Amb[{}]", trees.iter().fold("".to_string(), |ts, t| format!("{}{}", if ts == "" {ts} else {format!("{},", ts)}, t.iter().fold("".to_string(), |ts, t| format!("{}{}",ts, t.to_string(symbol))))))
            }
        }
    }
}