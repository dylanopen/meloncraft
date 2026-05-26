use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WallTorch {
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for WallTorch {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North {
            return 3170;
        }
        if self.r#facing == Facing::West {
            return 3172;
        }
        if self.r#facing == Facing::South {
            return 3171;
        }
        if self.r#facing == Facing::East {
            return 3173;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 3170 {
            return Some(WallTorch {
                r#facing: Facing::North,
            });
        }
        if state_id == 3172 {
            return Some(WallTorch {
                r#facing: Facing::West,
            });
        }
        if state_id == 3171 {
            return Some(WallTorch {
                r#facing: Facing::South,
            });
        }
        if state_id == 3173 {
            return Some(WallTorch {
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
