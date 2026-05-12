use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooWallSign {
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

impl BlockState for BambooWallSign {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 5704; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 5702; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 5700; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 5701; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 5699; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 5703; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 5698; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 5705; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5704 {
            return Some(BambooWallSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 5702 {
            return Some(BambooWallSign {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 5700 {
            return Some(BambooWallSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 5701 {
            return Some(BambooWallSign {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 5699 {
            return Some(BambooWallSign {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 5703 {
            return Some(BambooWallSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 5698 {
            return Some(BambooWallSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 5705 {
            return Some(BambooWallSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

