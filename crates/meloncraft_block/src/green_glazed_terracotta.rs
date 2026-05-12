use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreenGlazedTerracotta {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for GreenGlazedTerracotta {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West { return 14818; }
        if block_state.r#facing == Facing::North { return 14816; }
        if block_state.r#facing == Facing::East { return 14819; }
        if block_state.r#facing == Facing::South { return 14817; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14818 {
            return Some(GreenGlazedTerracotta {
                r#facing: Facing::West,
            });
        }
        if state_id == 14816 {
            return Some(GreenGlazedTerracotta {
                r#facing: Facing::North,
            });
        }
        if state_id == 14819 {
            return Some(GreenGlazedTerracotta {
                r#facing: Facing::East,
            });
        }
        if state_id == 14817 {
            return Some(GreenGlazedTerracotta {
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

