use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleWallHangingSign {
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

impl BlockState for JungleWallHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 6519; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 6521; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 6516; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 6515; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 6514; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 6517; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 6518; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 6520; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6519 {
            return Some(JungleWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 6521 {
            return Some(JungleWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 6516 {
            return Some(JungleWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 6515 {
            return Some(JungleWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6514 {
            return Some(JungleWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 6517 {
            return Some(JungleWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 6518 {
            return Some(JungleWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 6520 {
            return Some(JungleWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

