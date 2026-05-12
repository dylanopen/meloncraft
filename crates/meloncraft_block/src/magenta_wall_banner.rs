use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaWallBanner {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for MagentaWallBanner {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South { return 12990; }
        if block_state.r#facing == Facing::East { return 12992; }
        if block_state.r#facing == Facing::West { return 12991; }
        if block_state.r#facing == Facing::North { return 12989; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12990 {
            return Some(MagentaWallBanner {
                r#facing: Facing::South,
            });
        }
        if state_id == 12992 {
            return Some(MagentaWallBanner {
                r#facing: Facing::East,
            });
        }
        if state_id == 12991 {
            return Some(MagentaWallBanner {
                r#facing: Facing::West,
            });
        }
        if state_id == 12989 {
            return Some(MagentaWallBanner {
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

