use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachedPumpkinStem {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for AttachedPumpkinStem {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South { return 8134; }
        if block_state.r#facing == Facing::North { return 8133; }
        if block_state.r#facing == Facing::West { return 8135; }
        if block_state.r#facing == Facing::East { return 8136; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8134 {
            return Some(AttachedPumpkinStem {
                r#facing: Facing::South,
            });
        }
        if state_id == 8133 {
            return Some(AttachedPumpkinStem {
                r#facing: Facing::North,
            });
        }
        if state_id == 8135 {
            return Some(AttachedPumpkinStem {
                r#facing: Facing::West,
            });
        }
        if state_id == 8136 {
            return Some(AttachedPumpkinStem {
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

