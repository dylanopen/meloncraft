use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loom {
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for Loom {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West {
            return 20538;
        }
        if self.r#facing == Facing::South {
            return 20537;
        }
        if self.r#facing == Facing::East {
            return 20539;
        }
        if self.r#facing == Facing::North {
            return 20536;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20538 {
            return Some(Loom {
                r#facing: Facing::West,
            });
        }
        if state_id == 20537 {
            return Some(Loom {
                r#facing: Facing::South,
            });
        }
        if state_id == 20539 {
            return Some(Loom {
                r#facing: Facing::East,
            });
        }
        if state_id == 20536 {
            return Some(Loom {
                r#facing: Facing::North,
            });
        }
        return None;
    }
}
