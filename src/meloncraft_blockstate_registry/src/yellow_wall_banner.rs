use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YellowWallBanner {
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for YellowWallBanner {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North {
            return 12997;
        }
        if self.r#facing == Facing::East {
            return 13000;
        }
        if self.r#facing == Facing::South {
            return 12998;
        }
        if self.r#facing == Facing::West {
            return 12999;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12997 {
            return Some(YellowWallBanner {
                r#facing: Facing::North,
            });
        }
        if state_id == 13000 {
            return Some(YellowWallBanner {
                r#facing: Facing::East,
            });
        }
        if state_id == 12998 {
            return Some(YellowWallBanner {
                r#facing: Facing::South,
            });
        }
        if state_id == 12999 {
            return Some(YellowWallBanner {
                r#facing: Facing::West,
            });
        }
        return None;
    }
}
