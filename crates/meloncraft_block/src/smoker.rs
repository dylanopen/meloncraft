use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Smoker {
    pub r#facing: Facing,
    pub lit: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for Smoker {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#lit == true { return 20554; }
        if block_state.r#lit == true && block_state.r#facing == Facing::East { return 20558; }
        if block_state.r#facing == Facing::North && block_state.r#lit == true { return 20552; }
        if block_state.r#facing == Facing::West && block_state.r#lit == true { return 20556; }
        if block_state.r#lit == false && block_state.r#facing == Facing::East { return 20559; }
        if block_state.r#facing == Facing::South && block_state.r#lit == false { return 20555; }
        if block_state.r#facing == Facing::West && block_state.r#lit == false { return 20557; }
        if block_state.r#facing == Facing::North && block_state.r#lit == false { return 20553; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20554 {
            return Some(Smoker {
                r#facing: Facing::South,
                r#lit: true,
            });
        }
        if state_id == 20558 {
            return Some(Smoker {
                r#lit: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 20552 {
            return Some(Smoker {
                r#facing: Facing::North,
                r#lit: true,
            });
        }
        if state_id == 20556 {
            return Some(Smoker {
                r#facing: Facing::West,
                r#lit: true,
            });
        }
        if state_id == 20559 {
            return Some(Smoker {
                r#lit: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 20555 {
            return Some(Smoker {
                r#facing: Facing::South,
                r#lit: false,
            });
        }
        if state_id == 20557 {
            return Some(Smoker {
                r#facing: Facing::West,
                r#lit: false,
            });
        }
        if state_id == 20553 {
            return Some(Smoker {
                r#facing: Facing::North,
                r#lit: false,
            });
        }
        return None;
    }
}

