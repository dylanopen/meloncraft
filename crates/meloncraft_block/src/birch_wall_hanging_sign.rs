use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchWallHangingSign {
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

impl BlockState for BirchWallHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 6493; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 6497; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 6490; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 6494; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 6496; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 6492; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 6495; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 6491; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6493 {
            return Some(BirchWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 6497 {
            return Some(BirchWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 6490 {
            return Some(BirchWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 6494 {
            return Some(BirchWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 6496 {
            return Some(BirchWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 6492 {
            return Some(BirchWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 6495 {
            return Some(BirchWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 6491 {
            return Some(BirchWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

