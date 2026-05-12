use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piston {
    pub r#facing: Facing,
    pub extended: bool,
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

impl BlockState for Piston {
    fn to_id(self) -> i32 {
        if block_state.r#extended == false && block_state.r#facing == Facing::Up { return 2067; }
        if block_state.r#facing == Facing::East && block_state.r#extended == true { return 2058; }
        if block_state.r#facing == Facing::Up && block_state.r#extended == true { return 2061; }
        if block_state.r#facing == Facing::East && block_state.r#extended == false { return 2064; }
        if block_state.r#facing == Facing::Down && block_state.r#extended == true { return 2062; }
        if block_state.r#extended == true && block_state.r#facing == Facing::South { return 2059; }
        if block_state.r#facing == Facing::Down && block_state.r#extended == false { return 2068; }
        if block_state.r#facing == Facing::North && block_state.r#extended == true { return 2057; }
        if block_state.r#facing == Facing::South && block_state.r#extended == false { return 2065; }
        if block_state.r#extended == false && block_state.r#facing == Facing::North { return 2063; }
        if block_state.r#extended == false && block_state.r#facing == Facing::West { return 2066; }
        if block_state.r#extended == true && block_state.r#facing == Facing::West { return 2060; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2067 {
            return Some(Piston {
                r#extended: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 2058 {
            return Some(Piston {
                r#facing: Facing::East,
                r#extended: true,
            });
        }
        if state_id == 2061 {
            return Some(Piston {
                r#facing: Facing::Up,
                r#extended: true,
            });
        }
        if state_id == 2064 {
            return Some(Piston {
                r#facing: Facing::East,
                r#extended: false,
            });
        }
        if state_id == 2062 {
            return Some(Piston {
                r#facing: Facing::Down,
                r#extended: true,
            });
        }
        if state_id == 2059 {
            return Some(Piston {
                r#extended: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2068 {
            return Some(Piston {
                r#facing: Facing::Down,
                r#extended: false,
            });
        }
        if state_id == 2057 {
            return Some(Piston {
                r#facing: Facing::North,
                r#extended: true,
            });
        }
        if state_id == 2065 {
            return Some(Piston {
                r#facing: Facing::South,
                r#extended: false,
            });
        }
        if state_id == 2063 {
            return Some(Piston {
                r#extended: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2066 {
            return Some(Piston {
                r#extended: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2060 {
            return Some(Piston {
                r#extended: true,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

