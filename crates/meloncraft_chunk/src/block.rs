#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub state_id: i32,
}

impl Block {
    pub fn new(state_id: i32) -> Self {
        Self { state_id }
    }
}