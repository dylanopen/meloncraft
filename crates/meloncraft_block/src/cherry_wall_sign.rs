use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryWallSign {
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

impl BlockState for CherryWallSign {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 5658; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 5661; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 5662; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 5665; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 5664; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 5660; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 5659; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 5663; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5658 {
            return Some(CherryWallSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 5661 {
            return Some(CherryWallSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 5662 {
            return Some(CherryWallSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 5665 {
            return Some(CherryWallSign {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 5664 {
            return Some(CherryWallSign {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 5660 {
            return Some(CherryWallSign {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 5659 {
            return Some(CherryWallSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 5663 {
            return Some(CherryWallSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

