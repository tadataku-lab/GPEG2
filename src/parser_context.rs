pub mod parser_context{

    use std::cell::RefCell;
    use state::state::State;
    use memo::memo::Memo;
    use std::rc::Rc;

    pub struct ParserContext{
        pub input: Vec<u8>,
        pub rules: Vec<Box<Fn(& ParserContext) -> bool>>,
        pub state: RefCell<State>,
        pub memo: RefCell<Vec<Memo>>,
        pub bias: usize,
        pub bench: RefCell<[i32;1]>
    }

    impl ParserContext{
        pub fn new(input: Vec<u8>, rules: Vec<Box<Fn(& ParserContext) -> bool>>) -> ParserContext{
            ParserContext{
                input: input.clone(),
                memo: RefCell::new(Self::new_memo( (input.len() + 1) * rules.len() )),
                state: RefCell::new(State::start()),
                bias: rules.len(),
                rules: rules,
                bench: RefCell::new([0])
            }
        }

        fn new_memo(size: usize) -> Vec<Memo> {
            let mut new = Vec::new();
            for _ in 0..size{
                new.push(Memo::Nil);
            }
            new
        }

        pub fn show_tree(&self, symbol: &[&'static str]) -> String{
            self.state.borrow().show_tree(symbol)
        }

        pub fn lookup(&self, pos: usize, symbol: usize) -> Memo {
            //self.bench.borrow_mut()[0] += 1;
            self.memo.borrow()[ pos * self.bias + symbol].clone()
        }

        pub fn succ_memo(&self, pos: usize, symbol: usize, state: State) {
            self.memo.borrow_mut()[ pos * self.bias + symbol] = Memo::Succ(state);
            //self.bench.borrow_mut()[0] += 1;
        }

        pub fn fail_memo(&self, pos: usize, symbol: usize) {
            self.memo.borrow_mut()[ pos * self.bias + symbol] = Memo::Fail;
            //self.bench.borrow_mut()[0] += 1;
        }
    }

}
