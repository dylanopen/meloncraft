use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleWallSign {
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

impl BlockState for JungleWallSign {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 5666; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 5667; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 5673; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 5668; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 5669; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 5670; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 5671; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 5672; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5666 {
            return Some(JungleWallSign {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 5667 {
            return Some(JungleWallSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 5673 {
            return Some(JungleWallSign {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 5668 {
            return Some(JungleWallSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 5669 {
            return Some(JungleWallSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 5670 {
            return Some(JungleWallSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 5671 {
            return Some(JungleWallSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 5672 {
            return Some(JungleWallSign {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

