pub mod memo{

    use state::state::State;

    #[derive(Debug, Clone)]
    pub enum Memo{
        Nil,
        Fail,
        Succ(State),
    }
}