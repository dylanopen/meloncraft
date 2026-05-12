use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hopper {
    pub r#facing: Facing,
    pub enabled: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    Down,
    North,
    South,
    West,
    East,
}

impl BlockState for Hopper {
    fn to_id(self) -> i32 {
        if block_state.r#enabled == true && block_state.r#facing == Facing::North { return 11112; }
        if block_state.r#facing == Facing::South && block_state.r#enabled == false { return 11118; }
        if block_state.r#enabled == true && block_state.r#facing == Facing::West { return 11114; }
        if block_state.r#facing == Facing::North && block_state.r#enabled == false { return 11117; }
        if block_state.r#enabled == true && block_state.r#facing == Facing::Down { return 11111; }
        if block_state.r#facing == Facing::Down && block_state.r#enabled == false { return 11116; }
        if block_state.r#enabled == true && block_state.r#facing == Facing::East { return 11115; }
        if block_state.r#enabled == false && block_state.r#facing == Facing::West { return 11119; }
        if block_state.r#enabled == false && block_state.r#facing == Facing::East { return 11120; }
        if block_state.r#enabled == true && block_state.r#facing == Facing::South { return 11113; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11112 {
            return Some(Hopper {
                r#enabled: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 11118 {
            return Some(Hopper {
                r#facing: Facing::South,
                r#enabled: false,
            });
        }
        if state_id == 11114 {
            return Some(Hopper {
                r#enabled: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 11117 {
            return Some(Hopper {
                r#facing: Facing::North,
                r#enabled: false,
            });
        }
        if state_id == 11111 {
            return Some(Hopper {
                r#enabled: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 11116 {
            return Some(Hopper {
                r#facing: Facing::Down,
                r#enabled: false,
            });
        }
        if state_id == 11115 {
            return Some(Hopper {
                r#enabled: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 11119 {
            return Some(Hopper {
                r#enabled: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 11120 {
            return Some(Hopper {
                r#enabled: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 11113 {
            return Some(Hopper {
                r#enabled: true,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

