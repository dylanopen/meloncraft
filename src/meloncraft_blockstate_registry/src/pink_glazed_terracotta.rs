use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkGlazedTerracotta {
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for PinkGlazedTerracotta {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East {
            return 14791;
        }
        if self.r#facing == Facing::North {
            return 14788;
        }
        if self.r#facing == Facing::West {
            return 14790;
        }
        if self.r#facing == Facing::South {
            return 14789;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14791 {
            return Some(PinkGlazedTerracotta {
                r#facing: Facing::East,
            });
        }
        if state_id == 14788 {
            return Some(PinkGlazedTerracotta {
                r#facing: Facing::North,
            });
        }
        if state_id == 14790 {
            return Some(PinkGlazedTerracotta {
                r#facing: Facing::West,
            });
        }
        if state_id == 14789 {
            return Some(PinkGlazedTerracotta {
                r#facing: Facing::South,
            });
        }
        return None;
    }
}
