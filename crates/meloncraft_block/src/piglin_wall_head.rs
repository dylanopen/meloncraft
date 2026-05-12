use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiglinWallHead {
    pub powered: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for PiglinWallHead {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#powered == true { return 10987; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false { return 10986; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West { return 10990; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true { return 10991; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false { return 10992; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true { return 10989; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true { return 10985; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false { return 10988; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10987 {
            return Some(PiglinWallHead {
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10986 {
            return Some(PiglinWallHead {
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10990 {
            return Some(PiglinWallHead {
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 10991 {
            return Some(PiglinWallHead {
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10992 {
            return Some(PiglinWallHead {
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10989 {
            return Some(PiglinWallHead {
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10985 {
            return Some(PiglinWallHead {
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 10988 {
            return Some(PiglinWallHead {
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        return None;
    }
}

