use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayWallBanner {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for GrayWallBanner {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West { return 13011; }
        if block_state.r#facing == Facing::North { return 13009; }
        if block_state.r#facing == Facing::South { return 13010; }
        if block_state.r#facing == Facing::East { return 13012; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13011 {
            return Some(GrayWallBanner {
                r#facing: Facing::West,
            });
        }
        if state_id == 13009 {
            return Some(GrayWallBanner {
                r#facing: Facing::North,
            });
        }
        if state_id == 13010 {
            return Some(GrayWallBanner {
                r#facing: Facing::South,
            });
        }
        if state_id == 13012 {
            return Some(GrayWallBanner {
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

