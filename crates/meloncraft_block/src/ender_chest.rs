use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnderChest {
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

impl BlockState for EnderChest {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 9377; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 9374; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 9381; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 9376; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 9379; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 9378; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 9380; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 9375; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9377 {
            return Some(EnderChest {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 9374 {
            return Some(EnderChest {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 9381 {
            return Some(EnderChest {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 9376 {
            return Some(EnderChest {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 9379 {
            return Some(EnderChest {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 9378 {
            return Some(EnderChest {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 9380 {
            return Some(EnderChest {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 9375 {
            return Some(EnderChest {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

