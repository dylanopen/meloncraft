use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedWallBanner {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for RedWallBanner {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North { return 13037; }
        if block_state.r#facing == Facing::West { return 13039; }
        if block_state.r#facing == Facing::South { return 13038; }
        if block_state.r#facing == Facing::East { return 13040; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13037 {
            return Some(RedWallBanner {
                r#facing: Facing::North,
            });
        }
        if state_id == 13039 {
            return Some(RedWallBanner {
                r#facing: Facing::West,
            });
        }
        if state_id == 13038 {
            return Some(RedWallBanner {
                r#facing: Facing::South,
            });
        }
        if state_id == 13040 {
            return Some(RedWallBanner {
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

