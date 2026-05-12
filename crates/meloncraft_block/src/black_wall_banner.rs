use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackWallBanner {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for BlackWallBanner {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South { return 13042; }
        if self.r#facing == Facing::West { return 13043; }
        if self.r#facing == Facing::East { return 13044; }
        if self.r#facing == Facing::North { return 13041; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13042 {
            return Some(BlackWallBanner {
                r#facing: Facing::South,
            });
        }
        if state_id == 13043 {
            return Some(BlackWallBanner {
                r#facing: Facing::West,
            });
        }
        if state_id == 13044 {
            return Some(BlackWallBanner {
                r#facing: Facing::East,
            });
        }
        if state_id == 13041 {
            return Some(BlackWallBanner {
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

