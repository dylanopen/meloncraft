use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DamagedAnvil {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for DamagedAnvil {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::East { return 11004; }
        if block_state.r#facing == Facing::North { return 11001; }
        if block_state.r#facing == Facing::South { return 11002; }
        if block_state.r#facing == Facing::West { return 11003; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11004 {
            return Some(DamagedAnvil {
                r#facing: Facing::East,
            });
        }
        if state_id == 11001 {
            return Some(DamagedAnvil {
                r#facing: Facing::North,
            });
        }
        if state_id == 11002 {
            return Some(DamagedAnvil {
                r#facing: Facing::South,
            });
        }
        if state_id == 11003 {
            return Some(DamagedAnvil {
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

