use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakWallSign {
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

impl BlockState for OakWallSign {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 5626; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 5630; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 5627; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 5629; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 5633; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 5631; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 5632; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 5628; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5626 {
            return Some(OakWallSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 5630 {
            return Some(OakWallSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 5627 {
            return Some(OakWallSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 5629 {
            return Some(OakWallSign {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 5633 {
            return Some(OakWallSign {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 5631 {
            return Some(OakWallSign {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 5632 {
            return Some(OakWallSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 5628 {
            return Some(OakWallSign {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

