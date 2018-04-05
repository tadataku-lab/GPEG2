pub mod parser_context{

    use std::cell::RefCell;
    use state::state::State;
    use tree::tree::{ChildTree};

    pub struct ParserContext{
        pub input: Vec<u8>,
        pub new: Vec<ChildTree>,
        pub rules: Vec<Box<Fn(& ParserContext) -> bool>>,
        pub state: RefCell<State>,
        pub memo: RefCell<Vec<Option<State>>>,
        pub bias: usize,
        pub bench: RefCell<[i32;1]>
    }

    impl ParserContext{
        pub fn new(input: Vec<u8>, rules: Vec<Box<Fn(& ParserContext) -> bool>>) -> ParserContext{
            ParserContext{
                input: input.clone(),
                new: Self::fill(input.len() + 1),
                memo: RefCell::new(Self::new_memo( (input.len() + 1) * rules.len() )),
                state: RefCell::new(State::start(Self::fill(input.len() + 1))),
                bias: rules.len(),
                rules: rules,
                bench: RefCell::new([0])
            }
        }

        fn fill(size: usize) -> Vec<ChildTree>{
            let mut new = Vec::new();
            for _ in 0..size{
                new.push(ChildTree::Nil);
            }
            new
        }

        fn new_memo(size: usize) -> Vec<Option<State>> {
            let mut new = Vec::new();
            for _ in 0..size{
                new.push(None);
            }
            new
        }

        pub fn show_tree(&self, symbol: &[&'static str]) -> String{
            self.state.borrow().show_tree(symbol)
        }

        pub fn lookup(&self, pos: usize, symbol: usize) -> Option<State> {
            self.bench.borrow_mut()[0] += 1;
            self.memo.borrow()[ pos * self.bias + symbol].clone()
        }

        pub fn memo(&self, pos: usize, symbol: usize, state: State) {
            self.memo.borrow_mut()[ pos * self.bias + symbol] = Some(state);
            //self.bench.borrow_mut()[0] += 1;
        }
    }

}
