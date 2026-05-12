use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitherSkeletonWallSkull {
    pub r#facing: Facing,
    pub powered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for WitherSkeletonWallSkull {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West && block_state.r#powered == false { return 10790; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South { return 10788; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North { return 10785; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false { return 10786; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true { return 10789; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false { return 10792; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South { return 10787; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true { return 10791; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10790 {
            return Some(WitherSkeletonWallSkull {
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10788 {
            return Some(WitherSkeletonWallSkull {
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 10785 {
            return Some(WitherSkeletonWallSkull {
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 10786 {
            return Some(WitherSkeletonWallSkull {
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10789 {
            return Some(WitherSkeletonWallSkull {
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10792 {
            return Some(WitherSkeletonWallSkull {
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10787 {
            return Some(WitherSkeletonWallSkull {
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 10791 {
            return Some(WitherSkeletonWallSkull {
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        return None;
    }
}

