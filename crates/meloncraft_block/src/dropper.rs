use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dropper {
    pub r#facing: Facing,
    pub triggered: bool,
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

impl BlockState for Dropper {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::Up && block_state.r#triggered == true { return 11238; }
        if block_state.r#triggered == false && block_state.r#facing == Facing::West { return 11237; }
        if block_state.r#triggered == false && block_state.r#facing == Facing::Up { return 11239; }
        if block_state.r#facing == Facing::Down && block_state.r#triggered == true { return 11240; }
        if block_state.r#facing == Facing::North && block_state.r#triggered == true { return 11230; }
        if block_state.r#facing == Facing::North && block_state.r#triggered == false { return 11231; }
        if block_state.r#facing == Facing::Down && block_state.r#triggered == false { return 11241; }
        if block_state.r#facing == Facing::South && block_state.r#triggered == false { return 11235; }
        if block_state.r#facing == Facing::South && block_state.r#triggered == true { return 11234; }
        if block_state.r#facing == Facing::West && block_state.r#triggered == true { return 11236; }
        if block_state.r#facing == Facing::East && block_state.r#triggered == true { return 11232; }
        if block_state.r#facing == Facing::East && block_state.r#triggered == false { return 11233; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11238 {
            return Some(Dropper {
                r#facing: Facing::Up,
                r#triggered: true,
            });
        }
        if state_id == 11237 {
            return Some(Dropper {
                r#triggered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 11239 {
            return Some(Dropper {
                r#triggered: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 11240 {
            return Some(Dropper {
                r#facing: Facing::Down,
                r#triggered: true,
            });
        }
        if state_id == 11230 {
            return Some(Dropper {
                r#facing: Facing::North,
                r#triggered: true,
            });
        }
        if state_id == 11231 {
            return Some(Dropper {
                r#facing: Facing::North,
                r#triggered: false,
            });
        }
        if state_id == 11241 {
            return Some(Dropper {
                r#facing: Facing::Down,
                r#triggered: false,
            });
        }
        if state_id == 11235 {
            return Some(Dropper {
                r#facing: Facing::South,
                r#triggered: false,
            });
        }
        if state_id == 11234 {
            return Some(Dropper {
                r#facing: Facing::South,
                r#triggered: true,
            });
        }
        if state_id == 11236 {
            return Some(Dropper {
                r#facing: Facing::West,
                r#triggered: true,
            });
        }
        if state_id == 11232 {
            return Some(Dropper {
                r#facing: Facing::East,
                r#triggered: true,
            });
        }
        if state_id == 11233 {
            return Some(Dropper {
                r#facing: Facing::East,
                r#triggered: false,
            });
        }
        return None;
    }
}

