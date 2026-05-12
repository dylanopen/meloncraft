use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Barrel {
    pub open: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl BlockState for Barrel {
    fn to_id(self) -> i32 {
        if block_state.r#open == false && block_state.r#facing == Facing::North { return 20541; }
        if block_state.r#facing == Facing::West && block_state.r#open == false { return 20547; }
        if block_state.r#open == true && block_state.r#facing == Facing::West { return 20546; }
        if block_state.r#facing == Facing::East && block_state.r#open == true { return 20542; }
        if block_state.r#open == false && block_state.r#facing == Facing::Down { return 20551; }
        if block_state.r#open == false && block_state.r#facing == Facing::South { return 20545; }
        if block_state.r#facing == Facing::Down && block_state.r#open == true { return 20550; }
        if block_state.r#facing == Facing::North && block_state.r#open == true { return 20540; }
        if block_state.r#open == true && block_state.r#facing == Facing::South { return 20544; }
        if block_state.r#open == false && block_state.r#facing == Facing::Up { return 20549; }
        if block_state.r#open == false && block_state.r#facing == Facing::East { return 20543; }
        if block_state.r#open == true && block_state.r#facing == Facing::Up { return 20548; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20541 {
            return Some(Barrel {
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 20547 {
            return Some(Barrel {
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 20546 {
            return Some(Barrel {
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 20542 {
            return Some(Barrel {
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 20551 {
            return Some(Barrel {
                r#open: false,
                r#facing: Facing::Down,
            });
        }
        if state_id == 20545 {
            return Some(Barrel {
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 20550 {
            return Some(Barrel {
                r#facing: Facing::Down,
                r#open: true,
            });
        }
        if state_id == 20540 {
            return Some(Barrel {
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 20544 {
            return Some(Barrel {
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 20549 {
            return Some(Barrel {
                r#open: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 20543 {
            return Some(Barrel {
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 20548 {
            return Some(Barrel {
                r#open: true,
                r#facing: Facing::Up,
            });
        }
        return None;
    }
}

