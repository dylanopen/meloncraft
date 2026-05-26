use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackGlazedTerracotta {
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for BlackGlazedTerracotta {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East {
            return 14827;
        }
        if self.r#facing == Facing::West {
            return 14826;
        }
        if self.r#facing == Facing::North {
            return 14824;
        }
        if self.r#facing == Facing::South {
            return 14825;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14827 {
            return Some(BlackGlazedTerracotta {
                r#facing: Facing::East,
            });
        }
        if state_id == 14826 {
            return Some(BlackGlazedTerracotta {
                r#facing: Facing::West,
            });
        }
        if state_id == 14824 {
            return Some(BlackGlazedTerracotta {
                r#facing: Facing::North,
            });
        }
        if state_id == 14825 {
            return Some(BlackGlazedTerracotta {
                r#facing: Facing::South,
            });
        }
        return None;
    }
}
