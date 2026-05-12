use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedstoneWallTorch {
    pub lit: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for RedstoneWallTorch {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#lit == false { return 6687; }
        if block_state.r#lit == false && block_state.r#facing == Facing::South { return 6689; }
        if block_state.r#facing == Facing::West && block_state.r#lit == true { return 6690; }
        if block_state.r#lit == true && block_state.r#facing == Facing::South { return 6688; }
        if block_state.r#facing == Facing::West && block_state.r#lit == false { return 6691; }
        if block_state.r#facing == Facing::East && block_state.r#lit == true { return 6692; }
        if block_state.r#lit == true && block_state.r#facing == Facing::North { return 6686; }
        if block_state.r#facing == Facing::East && block_state.r#lit == false { return 6693; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6687 {
            return Some(RedstoneWallTorch {
                r#facing: Facing::North,
                r#lit: false,
            });
        }
        if state_id == 6689 {
            return Some(RedstoneWallTorch {
                r#lit: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 6690 {
            return Some(RedstoneWallTorch {
                r#facing: Facing::West,
                r#lit: true,
            });
        }
        if state_id == 6688 {
            return Some(RedstoneWallTorch {
                r#lit: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 6691 {
            return Some(RedstoneWallTorch {
                r#facing: Facing::West,
                r#lit: false,
            });
        }
        if state_id == 6692 {
            return Some(RedstoneWallTorch {
                r#facing: Facing::East,
                r#lit: true,
            });
        }
        if state_id == 6686 {
            return Some(RedstoneWallTorch {
                r#lit: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 6693 {
            return Some(RedstoneWallTorch {
                r#facing: Facing::East,
                r#lit: false,
            });
        }
        return None;
    }
}

