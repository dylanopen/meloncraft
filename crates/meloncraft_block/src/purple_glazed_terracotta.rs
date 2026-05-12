use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleGlazedTerracotta {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for PurpleGlazedTerracotta {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North { return 14804; }
        if block_state.r#facing == Facing::South { return 14805; }
        if block_state.r#facing == Facing::East { return 14807; }
        if block_state.r#facing == Facing::West { return 14806; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14804 {
            return Some(PurpleGlazedTerracotta {
                r#facing: Facing::North,
            });
        }
        if state_id == 14805 {
            return Some(PurpleGlazedTerracotta {
                r#facing: Facing::South,
            });
        }
        if state_id == 14807 {
            return Some(PurpleGlazedTerracotta {
                r#facing: Facing::East,
            });
        }
        if state_id == 14806 {
            return Some(PurpleGlazedTerracotta {
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

