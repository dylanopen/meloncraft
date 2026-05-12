use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveWallSign {
    pub r#facing: Facing,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for MangroveWallSign {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 5696; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 5692; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 5690; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 5693; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 5697; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 5695; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 5694; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 5691; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5696 {
            return Some(MangroveWallSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 5692 {
            return Some(MangroveWallSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 5690 {
            return Some(MangroveWallSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 5693 {
            return Some(MangroveWallSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 5697 {
            return Some(MangroveWallSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 5695 {
            return Some(MangroveWallSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 5694 {
            return Some(MangroveWallSign {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 5691 {
            return Some(MangroveWallSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

