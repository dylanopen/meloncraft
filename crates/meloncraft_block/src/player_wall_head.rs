use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerWallHead {
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

impl BlockState for PlayerWallHead {
    fn to_id(self) -> i32 {
        if block_state.r#powered == false && block_state.r#facing == Facing::East { return 10872; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true { return 10869; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true { return 10865; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South { return 10867; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false { return 10866; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East { return 10871; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false { return 10868; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false { return 10870; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10872 {
            return Some(PlayerWallHead {
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 10869 {
            return Some(PlayerWallHead {
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10865 {
            return Some(PlayerWallHead {
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 10867 {
            return Some(PlayerWallHead {
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 10866 {
            return Some(PlayerWallHead {
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10871 {
            return Some(PlayerWallHead {
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 10868 {
            return Some(PlayerWallHead {
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10870 {
            return Some(PlayerWallHead {
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        return None;
    }
}

