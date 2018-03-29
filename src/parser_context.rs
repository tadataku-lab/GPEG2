pub mod parser_context{

    use std::cell::RefCell;
    use state::state::State;
    use tree::tree::Tree;

    pub struct ParserContext{
        pub input: Vec<u8>,
        pub new: Vec<Vec<Tree>>,
        pub rules: Vec<Box<Fn(& ParserContext) -> bool>>,
        pub state: RefCell<State>
    }

    impl ParserContext{
        pub fn new(input: Vec<u8>, rules: Vec<Box<Fn(& ParserContext) -> bool>>) -> ParserContext{
            ParserContext{
                input: input.clone(),
                new: Self::fill(input.len() + 1),
                rules: rules,
                state: RefCell::new(State::new(Self::fill(input.len() + 1)))
            }
        }

        fn fill(size: usize) -> Vec<Vec<Tree>>{
            let mut new = Vec::new();
            for _ in 0..size{
                new.push(vec![Tree::Nil]);
            }
            new
        }
    }

}
