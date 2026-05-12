use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LargeAmethystBud {
    pub waterlogged: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl BlockState for LargeAmethystBud {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 23217; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::Up { return 23222; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 23214; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 23215; }
        if block_state.r#facing == Facing::Up && block_state.r#waterlogged == false { return 23223; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 23219; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 23220; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 23216; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 23218; }
        if block_state.r#facing == Facing::Down && block_state.r#waterlogged == true { return 23224; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::Down { return 23225; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 23221; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23217 {
            return Some(LargeAmethystBud {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 23222 {
            return Some(LargeAmethystBud {
                r#waterlogged: true,
                r#facing: Facing::Up,
            });
        }
        if state_id == 23214 {
            return Some(LargeAmethystBud {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 23215 {
            return Some(LargeAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 23223 {
            return Some(LargeAmethystBud {
                r#facing: Facing::Up,
                r#waterlogged: false,
            });
        }
        if state_id == 23219 {
            return Some(LargeAmethystBud {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 23220 {
            return Some(LargeAmethystBud {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 23216 {
            return Some(LargeAmethystBud {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 23218 {
            return Some(LargeAmethystBud {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 23224 {
            return Some(LargeAmethystBud {
                r#facing: Facing::Down,
                r#waterlogged: true,
            });
        }
        if state_id == 23225 {
            return Some(LargeAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::Down,
            });
        }
        if state_id == 23221 {
            return Some(LargeAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

