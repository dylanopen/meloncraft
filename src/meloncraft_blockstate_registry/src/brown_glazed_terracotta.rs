use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrownGlazedTerracotta {
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for BrownGlazedTerracotta {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North {
            return 14812;
        }
        if self.r#facing == Facing::South {
            return 14813;
        }
        if self.r#facing == Facing::East {
            return 14815;
        }
        if self.r#facing == Facing::West {
            return 14814;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14812 {
            return Some(BrownGlazedTerracotta {
                r#facing: Facing::North,
            });
        }
        if state_id == 14813 {
            return Some(BrownGlazedTerracotta {
                r#facing: Facing::South,
            });
        }
        if state_id == 14815 {
            return Some(BrownGlazedTerracotta {
                r#facing: Facing::East,
            });
        }
        if state_id == 14814 {
            return Some(BrownGlazedTerracotta {
                r#facing: Facing::West,
            });
        }
        return None;
    }
}
