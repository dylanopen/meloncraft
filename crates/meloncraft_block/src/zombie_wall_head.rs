use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZombieWallHead {
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

impl BlockState for ZombieWallHead {
    fn to_id(self) -> i32 {
        if block_state.r#powered == false && block_state.r#facing == Facing::North { return 10826; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true { return 10825; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South { return 10827; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true { return 10831; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West { return 10829; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false { return 10832; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false { return 10828; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false { return 10830; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10826 {
            return Some(ZombieWallHead {
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 10825 {
            return Some(ZombieWallHead {
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 10827 {
            return Some(ZombieWallHead {
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 10831 {
            return Some(ZombieWallHead {
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10829 {
            return Some(ZombieWallHead {
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10832 {
            return Some(ZombieWallHead {
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10828 {
            return Some(ZombieWallHead {
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10830 {
            return Some(ZombieWallHead {
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        return None;
    }
}

