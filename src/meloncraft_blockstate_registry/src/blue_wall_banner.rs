use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueWallBanner {
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for BlueWallBanner {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South {
            return 13026;
        }
        if self.r#facing == Facing::North {
            return 13025;
        }
        if self.r#facing == Facing::West {
            return 13027;
        }
        if self.r#facing == Facing::East {
            return 13028;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13026 {
            return Some(BlueWallBanner {
                r#facing: Facing::South,
            });
        }
        if state_id == 13025 {
            return Some(BlueWallBanner {
                r#facing: Facing::North,
            });
        }
        if state_id == 13027 {
            return Some(BlueWallBanner {
                r#facing: Facing::West,
            });
        }
        if state_id == 13028 {
            return Some(BlueWallBanner {
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
