use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YellowGlazedTerracotta {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for YellowGlazedTerracotta {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North { return 14780; }
        if block_state.r#facing == Facing::East { return 14783; }
        if block_state.r#facing == Facing::South { return 14781; }
        if block_state.r#facing == Facing::West { return 14782; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14780 {
            return Some(YellowGlazedTerracotta {
                r#facing: Facing::North,
            });
        }
        if state_id == 14783 {
            return Some(YellowGlazedTerracotta {
                r#facing: Facing::East,
            });
        }
        if state_id == 14781 {
            return Some(YellowGlazedTerracotta {
                r#facing: Facing::South,
            });
        }
        if state_id == 14782 {
            return Some(YellowGlazedTerracotta {
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

