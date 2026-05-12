use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaWallSign {
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

impl BlockState for AcaciaWallSign {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 5657; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 5655; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 5652; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 5656; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 5650; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 5653; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 5651; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 5654; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5657 {
            return Some(AcaciaWallSign {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 5655 {
            return Some(AcaciaWallSign {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 5652 {
            return Some(AcaciaWallSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 5656 {
            return Some(AcaciaWallSign {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 5650 {
            return Some(AcaciaWallSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 5653 {
            return Some(AcaciaWallSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 5651 {
            return Some(AcaciaWallSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 5654 {
            return Some(AcaciaWallSign {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

