pub mod parser_context{
    pub struct ParserContext{
        input: String,
        pos: i32,
        rules: Vec<Box<Fn(&mut ParserContext) -> bool>>
    }
}
