use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooWallHangingSign {
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

impl BlockState for BambooWallHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 6568; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 6567; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 6562; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 6569; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 6563; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 6564; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 6565; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 6566; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6568 {
            return Some(BambooWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 6567 {
            return Some(BambooWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 6562 {
            return Some(BambooWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 6569 {
            return Some(BambooWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 6563 {
            return Some(BambooWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 6564 {
            return Some(BambooWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 6565 {
            return Some(BambooWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 6566 {
            return Some(BambooWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

