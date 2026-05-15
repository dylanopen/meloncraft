#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Biome {
    pub state_id: i32,
}

impl Biome {
    pub fn new(state_id: i32) -> Self {
        Self { state_id }
    }
}
