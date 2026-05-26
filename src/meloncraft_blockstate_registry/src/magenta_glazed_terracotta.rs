use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaGlazedTerracotta {
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for MagentaGlazedTerracotta {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North {
            return 14772;
        }
        if self.r#facing == Facing::South {
            return 14773;
        }
        if self.r#facing == Facing::West {
            return 14774;
        }
        if self.r#facing == Facing::East {
            return 14775;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14772 {
            return Some(MagentaGlazedTerracotta {
                r#facing: Facing::North,
            });
        }
        if state_id == 14773 {
            return Some(MagentaGlazedTerracotta {
                r#facing: Facing::South,
            });
        }
        if state_id == 14774 {
            return Some(MagentaGlazedTerracotta {
                r#facing: Facing::West,
            });
        }
        if state_id == 14775 {
            return Some(MagentaGlazedTerracotta {
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
