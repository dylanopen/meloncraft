use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DragonWallHead {
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

impl BlockState for DragonWallHead {
    fn to_id(self) -> i32 {
        if block_state.r#powered == false && block_state.r#facing == Facing::North { return 10946; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South { return 10948; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true { return 10951; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East { return 10952; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South { return 10947; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false { return 10950; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true { return 10949; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North { return 10945; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10946 {
            return Some(DragonWallHead {
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 10948 {
            return Some(DragonWallHead {
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 10951 {
            return Some(DragonWallHead {
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10952 {
            return Some(DragonWallHead {
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 10947 {
            return Some(DragonWallHead {
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 10950 {
            return Some(DragonWallHead {
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10949 {
            return Some(DragonWallHead {
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10945 {
            return Some(DragonWallHead {
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

