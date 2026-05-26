use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightBlueGlazedTerracotta {
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for LightBlueGlazedTerracotta {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West {
            return 14778;
        }
        if self.r#facing == Facing::North {
            return 14776;
        }
        if self.r#facing == Facing::South {
            return 14777;
        }
        if self.r#facing == Facing::East {
            return 14779;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14778 {
            return Some(LightBlueGlazedTerracotta {
                r#facing: Facing::West,
            });
        }
        if state_id == 14776 {
            return Some(LightBlueGlazedTerracotta {
                r#facing: Facing::North,
            });
        }
        if state_id == 14777 {
            return Some(LightBlueGlazedTerracotta {
                r#facing: Facing::South,
            });
        }
        if state_id == 14779 {
            return Some(LightBlueGlazedTerracotta {
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
