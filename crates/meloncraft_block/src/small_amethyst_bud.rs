use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmallAmethystBud {
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

impl BlockState for SmallAmethystBud {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 23245; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 23243; }
        if block_state.r#facing == Facing::Up && block_state.r#waterlogged == true { return 23246; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 23239; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 23240; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 23242; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 23238; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 23241; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 23244; }
        if block_state.r#facing == Facing::Down && block_state.r#waterlogged == true { return 23248; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::Up { return 23247; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::Down { return 23249; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23245 {
            return Some(SmallAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 23243 {
            return Some(SmallAmethystBud {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 23246 {
            return Some(SmallAmethystBud {
                r#facing: Facing::Up,
                r#waterlogged: true,
            });
        }
        if state_id == 23239 {
            return Some(SmallAmethystBud {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 23240 {
            return Some(SmallAmethystBud {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 23242 {
            return Some(SmallAmethystBud {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 23238 {
            return Some(SmallAmethystBud {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 23241 {
            return Some(SmallAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 23244 {
            return Some(SmallAmethystBud {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 23248 {
            return Some(SmallAmethystBud {
                r#facing: Facing::Down,
                r#waterlogged: true,
            });
        }
        if state_id == 23247 {
            return Some(SmallAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 23249 {
            return Some(SmallAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::Down,
            });
        }
        return None;
    }
}

