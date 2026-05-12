use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueGlazedTerracotta {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for BlueGlazedTerracotta {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North { return 14808; }
        if block_state.r#facing == Facing::South { return 14809; }
        if block_state.r#facing == Facing::West { return 14810; }
        if block_state.r#facing == Facing::East { return 14811; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14808 {
            return Some(BlueGlazedTerracotta {
                r#facing: Facing::North,
            });
        }
        if state_id == 14809 {
            return Some(BlueGlazedTerracotta {
                r#facing: Facing::South,
            });
        }
        if state_id == 14810 {
            return Some(BlueGlazedTerracotta {
                r#facing: Facing::West,
            });
        }
        if state_id == 14811 {
            return Some(BlueGlazedTerracotta {
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

