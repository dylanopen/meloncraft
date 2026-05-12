use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Observer {
    pub r#facing: Facing,
    pub powered: bool,
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

impl BlockState for Observer {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#facing == Facing::North { return 14650; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false { return 14653; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false { return 14655; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West { return 14657; }
        if block_state.r#facing == Facing::Up && block_state.r#powered == true { return 14658; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West { return 14656; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true { return 14654; }
        if block_state.r#facing == Facing::Down && block_state.r#powered == false { return 14661; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North { return 14651; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East { return 14652; }
        if block_state.r#powered == false && block_state.r#facing == Facing::Up { return 14659; }
        if block_state.r#powered == true && block_state.r#facing == Facing::Down { return 14660; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14650 {
            return Some(Observer {
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 14653 {
            return Some(Observer {
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 14655 {
            return Some(Observer {
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 14657 {
            return Some(Observer {
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 14658 {
            return Some(Observer {
                r#facing: Facing::Up,
                r#powered: true,
            });
        }
        if state_id == 14656 {
            return Some(Observer {
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 14654 {
            return Some(Observer {
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 14661 {
            return Some(Observer {
                r#facing: Facing::Down,
                r#powered: false,
            });
        }
        if state_id == 14651 {
            return Some(Observer {
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 14652 {
            return Some(Observer {
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 14659 {
            return Some(Observer {
                r#powered: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 14660 {
            return Some(Observer {
                r#powered: true,
                r#facing: Facing::Down,
            });
        }
        return None;
    }
}

