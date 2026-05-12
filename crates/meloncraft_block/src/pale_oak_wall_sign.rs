use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakWallSign {
    pub waterlogged: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for PaleOakWallSign {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 5689; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 5682; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 5688; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 5684; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 5683; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 5685; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 5686; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 5687; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5689 {
            return Some(PaleOakWallSign {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 5682 {
            return Some(PaleOakWallSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 5688 {
            return Some(PaleOakWallSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 5684 {
            return Some(PaleOakWallSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 5683 {
            return Some(PaleOakWallSign {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 5685 {
            return Some(PaleOakWallSign {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 5686 {
            return Some(PaleOakWallSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 5687 {
            return Some(PaleOakWallSign {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

