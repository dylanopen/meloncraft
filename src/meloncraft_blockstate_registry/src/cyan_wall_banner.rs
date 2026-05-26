use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyanWallBanner {
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for CyanWallBanner {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East {
            return 13020;
        }
        if self.r#facing == Facing::North {
            return 13017;
        }
        if self.r#facing == Facing::South {
            return 13018;
        }
        if self.r#facing == Facing::West {
            return 13019;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13020 {
            return Some(CyanWallBanner {
                r#facing: Facing::East,
            });
        }
        if state_id == 13017 {
            return Some(CyanWallBanner {
                r#facing: Facing::North,
            });
        }
        if state_id == 13018 {
            return Some(CyanWallBanner {
                r#facing: Facing::South,
            });
        }
        if state_id == 13019 {
            return Some(CyanWallBanner {
                r#facing: Facing::West,
            });
        }
        return None;
    }
}
