#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block {
    pub state_id: i32,
}

impl Block {
    pub const fn new(state_id: i32) -> Self {
        Self { state_id }
    }
}