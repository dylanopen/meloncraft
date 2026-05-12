use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryWallHangingSign {
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

impl BlockState for CherryWallHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 6509; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 6511; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 6508; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 6506; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 6510; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 6512; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 6507; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 6513; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6509 {
            return Some(CherryWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 6511 {
            return Some(CherryWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 6508 {
            return Some(CherryWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 6506 {
            return Some(CherryWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 6510 {
            return Some(CherryWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 6512 {
            return Some(CherryWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 6507 {
            return Some(CherryWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6513 {
            return Some(CherryWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

