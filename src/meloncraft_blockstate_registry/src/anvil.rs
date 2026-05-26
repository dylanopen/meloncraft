use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Anvil {
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for Anvil {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West {
            return 10995;
        }
        if self.r#facing == Facing::South {
            return 10994;
        }
        if self.r#facing == Facing::North {
            return 10993;
        }
        if self.r#facing == Facing::East {
            return 10996;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10995 {
            return Some(Anvil {
                r#facing: Facing::West,
            });
        }
        if state_id == 10994 {
            return Some(Anvil {
                r#facing: Facing::South,
            });
        }
        if state_id == 10993 {
            return Some(Anvil {
                r#facing: Facing::North,
            });
        }
        if state_id == 10996 {
            return Some(Anvil {
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
