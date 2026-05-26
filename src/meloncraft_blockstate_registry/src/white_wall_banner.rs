use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteWallBanner {
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for WhiteWallBanner {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South {
            return 12982;
        }
        if self.r#facing == Facing::East {
            return 12984;
        }
        if self.r#facing == Facing::North {
            return 12981;
        }
        if self.r#facing == Facing::West {
            return 12983;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12982 {
            return Some(WhiteWallBanner {
                r#facing: Facing::South,
            });
        }
        if state_id == 12984 {
            return Some(WhiteWallBanner {
                r#facing: Facing::East,
            });
        }
        if state_id == 12981 {
            return Some(WhiteWallBanner {
                r#facing: Facing::North,
            });
        }
        if state_id == 12983 {
            return Some(WhiteWallBanner {
                r#facing: Facing::West,
            });
        }
        return None;
    }
}
