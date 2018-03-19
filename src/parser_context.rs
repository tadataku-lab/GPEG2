pub mod parser_context{

    use std::cell::Cell;
    use std::cell::RefCell;
    use tree::tree::Tree;

    pub struct ParserContext{
        pub input: Vec<u8>,
        pub rules: Vec<Box<Fn(& ParserContext) -> bool>>,
        pub pos: Cell<i32>,
        pub tree: RefCell<Vec<Tree>>
    }
}
