use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrangeGlazedTerracotta {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for OrangeGlazedTerracotta {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West { return 14770; }
        if block_state.r#facing == Facing::East { return 14771; }
        if block_state.r#facing == Facing::North { return 14768; }
        if block_state.r#facing == Facing::South { return 14769; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14770 {
            return Some(OrangeGlazedTerracotta {
                r#facing: Facing::West,
            });
        }
        if state_id == 14771 {
            return Some(OrangeGlazedTerracotta {
                r#facing: Facing::East,
            });
        }
        if state_id == 14768 {
            return Some(OrangeGlazedTerracotta {
                r#facing: Facing::North,
            });
        }
        if state_id == 14769 {
            return Some(OrangeGlazedTerracotta {
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

