use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimeGlazedTerracotta {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for LimeGlazedTerracotta {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North { return 14784; }
        if block_state.r#facing == Facing::West { return 14786; }
        if block_state.r#facing == Facing::East { return 14787; }
        if block_state.r#facing == Facing::South { return 14785; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14784 {
            return Some(LimeGlazedTerracotta {
                r#facing: Facing::North,
            });
        }
        if state_id == 14786 {
            return Some(LimeGlazedTerracotta {
                r#facing: Facing::West,
            });
        }
        if state_id == 14787 {
            return Some(LimeGlazedTerracotta {
                r#facing: Facing::East,
            });
        }
        if state_id == 14785 {
            return Some(LimeGlazedTerracotta {
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

