use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightGrayWallBanner {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for LightGrayWallBanner {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West { return 13015; }
        if block_state.r#facing == Facing::East { return 13016; }
        if block_state.r#facing == Facing::South { return 13014; }
        if block_state.r#facing == Facing::North { return 13013; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13015 {
            return Some(LightGrayWallBanner {
                r#facing: Facing::West,
            });
        }
        if state_id == 13016 {
            return Some(LightGrayWallBanner {
                r#facing: Facing::East,
            });
        }
        if state_id == 13014 {
            return Some(LightGrayWallBanner {
                r#facing: Facing::South,
            });
        }
        if state_id == 13013 {
            return Some(LightGrayWallBanner {
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

