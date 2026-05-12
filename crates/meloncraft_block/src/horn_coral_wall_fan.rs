use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HornCoralWallFan {
    pub r#facing: Facing,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for HornCoralWallFan {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 15060; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 15064; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 15063; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 15061; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 15057; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 15058; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 15059; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 15062; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15060 {
            return Some(HornCoralWallFan {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 15064 {
            return Some(HornCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15063 {
            return Some(HornCoralWallFan {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 15061 {
            return Some(HornCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 15057 {
            return Some(HornCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 15058 {
            return Some(HornCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15059 {
            return Some(HornCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 15062 {
            return Some(HornCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

