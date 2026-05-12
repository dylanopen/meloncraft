use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleWallBanner {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for PurpleWallBanner {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North { return 13021; }
        if block_state.r#facing == Facing::South { return 13022; }
        if block_state.r#facing == Facing::East { return 13024; }
        if block_state.r#facing == Facing::West { return 13023; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13021 {
            return Some(PurpleWallBanner {
                r#facing: Facing::North,
            });
        }
        if state_id == 13022 {
            return Some(PurpleWallBanner {
                r#facing: Facing::South,
            });
        }
        if state_id == 13024 {
            return Some(PurpleWallBanner {
                r#facing: Facing::East,
            });
        }
        if state_id == 13023 {
            return Some(PurpleWallBanner {
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

