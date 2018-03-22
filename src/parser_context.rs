pub mod parser_context{

    use std::cell::RefCell;
    use state::state::State;

    pub struct ParserContext{
        pub input: Vec<u8>,
        pub rules: Vec<Box<Fn(& ParserContext) -> bool>>,
        pub state: RefCell<State>
    }

}
