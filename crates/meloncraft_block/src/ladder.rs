use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ladder {
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

impl BlockState for Ladder {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 5520; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 5518; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 5525; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 5524; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 5523; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 5519; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 5521; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 5522; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5520 {
            return Some(Ladder {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 5518 {
            return Some(Ladder {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 5525 {
            return Some(Ladder {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 5524 {
            return Some(Ladder {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 5523 {
            return Some(Ladder {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 5519 {
            return Some(Ladder {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 5521 {
            return Some(Ladder {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 5522 {
            return Some(Ladder {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

