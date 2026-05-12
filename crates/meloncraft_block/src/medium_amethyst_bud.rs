use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediumAmethystBud {
    pub r#facing: Facing,
    pub waterlogged: bool,
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

impl BlockState for MediumAmethystBud {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::Up { return 23235; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 23228; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 23229; }
        if block_state.r#facing == Facing::Down && block_state.r#waterlogged == false { return 23237; }
        if block_state.r#facing == Facing::Up && block_state.r#waterlogged == true { return 23234; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 23226; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 23231; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 23233; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::Down { return 23236; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 23230; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 23232; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 23227; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23235 {
            return Some(MediumAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 23228 {
            return Some(MediumAmethystBud {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 23229 {
            return Some(MediumAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 23237 {
            return Some(MediumAmethystBud {
                r#facing: Facing::Down,
                r#waterlogged: false,
            });
        }
        if state_id == 23234 {
            return Some(MediumAmethystBud {
                r#facing: Facing::Up,
                r#waterlogged: true,
            });
        }
        if state_id == 23226 {
            return Some(MediumAmethystBud {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 23231 {
            return Some(MediumAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 23233 {
            return Some(MediumAmethystBud {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 23236 {
            return Some(MediumAmethystBud {
                r#waterlogged: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 23230 {
            return Some(MediumAmethystBud {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 23232 {
            return Some(MediumAmethystBud {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 23227 {
            return Some(MediumAmethystBud {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

