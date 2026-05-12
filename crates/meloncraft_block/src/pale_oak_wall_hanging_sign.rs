use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakWallHangingSign {
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

impl BlockState for PaleOakWallHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 6532; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 6534; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 6530; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 6536; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 6533; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 6535; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 6537; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 6531; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6532 {
            return Some(PaleOakWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 6534 {
            return Some(PaleOakWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 6530 {
            return Some(PaleOakWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 6536 {
            return Some(PaleOakWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 6533 {
            return Some(PaleOakWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 6535 {
            return Some(PaleOakWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 6537 {
            return Some(PaleOakWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 6531 {
            return Some(PaleOakWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

