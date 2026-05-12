use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonWallSign {
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

impl BlockState for CrimsonWallSign {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 21506; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 21505; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 21509; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 21511; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 21508; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 21510; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 21504; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 21507; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21506 {
            return Some(CrimsonWallSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 21505 {
            return Some(CrimsonWallSign {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 21509 {
            return Some(CrimsonWallSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 21511 {
            return Some(CrimsonWallSign {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 21508 {
            return Some(CrimsonWallSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 21510 {
            return Some(CrimsonWallSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 21504 {
            return Some(CrimsonWallSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 21507 {
            return Some(CrimsonWallSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

