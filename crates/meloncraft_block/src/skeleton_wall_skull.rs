use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkeletonWallSkull {
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

impl BlockState for SkeletonWallSkull {
    fn to_id(self) -> i32 {
        if block_state.r#powered == false && block_state.r#facing == Facing::West { return 10750; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true { return 10751; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false { return 10752; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North { return 10746; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West { return 10749; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true { return 10745; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false { return 10748; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true { return 10747; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10750 {
            return Some(SkeletonWallSkull {
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 10751 {
            return Some(SkeletonWallSkull {
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10752 {
            return Some(SkeletonWallSkull {
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10746 {
            return Some(SkeletonWallSkull {
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 10749 {
            return Some(SkeletonWallSkull {
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10745 {
            return Some(SkeletonWallSkull {
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 10748 {
            return Some(SkeletonWallSkull {
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10747 {
            return Some(SkeletonWallSkull {
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        return None;
    }
}

