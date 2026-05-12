use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakWallSign {
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

impl BlockState for DarkOakWallSign {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 5676; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 5677; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 5680; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 5675; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 5674; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 5678; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 5681; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 5679; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5676 {
            return Some(DarkOakWallSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 5677 {
            return Some(DarkOakWallSign {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 5680 {
            return Some(DarkOakWallSign {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 5675 {
            return Some(DarkOakWallSign {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 5674 {
            return Some(DarkOakWallSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 5678 {
            return Some(DarkOakWallSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 5681 {
            return Some(DarkOakWallSign {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 5679 {
            return Some(DarkOakWallSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

