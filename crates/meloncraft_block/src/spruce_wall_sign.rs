use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceWallSign {
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

impl BlockState for SpruceWallSign {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 5635; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 5641; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 5637; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 5638; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 5634; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 5640; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 5636; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 5639; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5635 {
            return Some(SpruceWallSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 5641 {
            return Some(SpruceWallSign {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 5637 {
            return Some(SpruceWallSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 5638 {
            return Some(SpruceWallSign {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 5634 {
            return Some(SpruceWallSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 5640 {
            return Some(SpruceWallSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 5636 {
            return Some(SpruceWallSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 5639 {
            return Some(SpruceWallSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

