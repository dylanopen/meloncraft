use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dispenser {
    pub triggered: bool,
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

impl BlockState for Dispenser {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#triggered == true { return 566; }
        if block_state.r#triggered == false && block_state.r#facing == Facing::East { return 569; }
        if block_state.r#facing == Facing::Up && block_state.r#triggered == true { return 574; }
        if block_state.r#triggered == false && block_state.r#facing == Facing::West { return 573; }
        if block_state.r#facing == Facing::Up && block_state.r#triggered == false { return 575; }
        if block_state.r#facing == Facing::West && block_state.r#triggered == true { return 572; }
        if block_state.r#triggered == true && block_state.r#facing == Facing::East { return 568; }
        if block_state.r#triggered == false && block_state.r#facing == Facing::North { return 567; }
        if block_state.r#triggered == true && block_state.r#facing == Facing::Down { return 576; }
        if block_state.r#triggered == true && block_state.r#facing == Facing::South { return 570; }
        if block_state.r#facing == Facing::Down && block_state.r#triggered == false { return 577; }
        if block_state.r#triggered == false && block_state.r#facing == Facing::South { return 571; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 566 {
            return Some(Dispenser {
                r#facing: Facing::North,
                r#triggered: true,
            });
        }
        if state_id == 569 {
            return Some(Dispenser {
                r#triggered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 574 {
            return Some(Dispenser {
                r#facing: Facing::Up,
                r#triggered: true,
            });
        }
        if state_id == 573 {
            return Some(Dispenser {
                r#triggered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 575 {
            return Some(Dispenser {
                r#facing: Facing::Up,
                r#triggered: false,
            });
        }
        if state_id == 572 {
            return Some(Dispenser {
                r#facing: Facing::West,
                r#triggered: true,
            });
        }
        if state_id == 568 {
            return Some(Dispenser {
                r#triggered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 567 {
            return Some(Dispenser {
                r#triggered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 576 {
            return Some(Dispenser {
                r#triggered: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 570 {
            return Some(Dispenser {
                r#triggered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 577 {
            return Some(Dispenser {
                r#facing: Facing::Down,
                r#triggered: false,
            });
        }
        if state_id == 571 {
            return Some(Dispenser {
                r#triggered: false,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

