use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchWallSign {
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

impl BlockState for BirchWallSign {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 5647; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 5648; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 5643; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 5649; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 5645; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 5642; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 5644; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 5646; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5647 {
            return Some(BirchWallSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 5648 {
            return Some(BirchWallSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 5643 {
            return Some(BirchWallSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 5649 {
            return Some(BirchWallSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 5645 {
            return Some(BirchWallSign {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 5642 {
            return Some(BirchWallSign {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 5644 {
            return Some(BirchWallSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 5646 {
            return Some(BirchWallSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

