#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block {
    pub state_id: i32,
}

impl Block {
    #[must_use]
    pub const fn new(state_id: i32) -> Self {
        return Block { state_id };
    }
}