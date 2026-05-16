use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimeWallBanner {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for LimeWallBanner {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East { return 13004; }
        if self.r#facing == Facing::West { return 13003; }
        if self.r#facing == Facing::North { return 13001; }
        if self.r#facing == Facing::South { return 13002; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13004 {
            return Some(LimeWallBanner {
                r#facing: Facing::East,
            });
        }
        if state_id == 13003 {
            return Some(LimeWallBanner {
                r#facing: Facing::West,
            });
        }
        if state_id == 13001 {
            return Some(LimeWallBanner {
                r#facing: Facing::North,
            });
        }
        if state_id == 13002 {
            return Some(LimeWallBanner {
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

