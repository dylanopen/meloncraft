#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Biome {
    pub state_id: i32,
}

impl Biome {
    #[must_use]
    pub const fn new(state_id: i32) -> Self {
        return Self { state_id };
    }
}
