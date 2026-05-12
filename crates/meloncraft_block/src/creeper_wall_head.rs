use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreeperWallHead {
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

impl BlockState for CreeperWallHead {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::East && block_state.r#powered == false { return 10912; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true { return 10911; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true { return 10905; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true { return 10909; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false { return 10906; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West { return 10910; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false { return 10908; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South { return 10907; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10912 {
            return Some(CreeperWallHead {
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10911 {
            return Some(CreeperWallHead {
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10905 {
            return Some(CreeperWallHead {
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 10909 {
            return Some(CreeperWallHead {
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10906 {
            return Some(CreeperWallHead {
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10910 {
            return Some(CreeperWallHead {
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 10908 {
            return Some(CreeperWallHead {
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10907 {
            return Some(CreeperWallHead {
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

