use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChippedAnvil {
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for ChippedAnvil {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West {
            return 10999;
        }
        if self.r#facing == Facing::South {
            return 10998;
        }
        if self.r#facing == Facing::East {
            return 11000;
        }
        if self.r#facing == Facing::North {
            return 10997;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10999 {
            return Some(ChippedAnvil {
                r#facing: Facing::West,
            });
        }
        if state_id == 10998 {
            return Some(ChippedAnvil {
                r#facing: Facing::South,
            });
        }
        if state_id == 11000 {
            return Some(ChippedAnvil {
                r#facing: Facing::East,
            });
        }
        if state_id == 10997 {
            return Some(ChippedAnvil {
                r#facing: Facing::North,
            });
        }
        return None;
    }
}
