use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cocoa {
    pub r#facing: Facing,
    pub age: i32,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for Cocoa {
    fn to_id(self) -> i32 {
        if block_state.r#age == 1 && block_state.r#facing == Facing::East { return 9287; }
        if block_state.r#age == 2 && block_state.r#facing == Facing::West { return 9290; }
        if block_state.r#facing == Facing::East && block_state.r#age == 2 { return 9291; }
        if block_state.r#age == 1 && block_state.r#facing == Facing::North { return 9284; }
        if block_state.r#facing == Facing::West && block_state.r#age == 1 { return 9286; }
        if block_state.r#facing == Facing::South && block_state.r#age == 1 { return 9285; }
        if block_state.r#age == 2 && block_state.r#facing == Facing::South { return 9289; }
        if block_state.r#age == 0 && block_state.r#facing == Facing::North { return 9280; }
        if block_state.r#age == 0 && block_state.r#facing == Facing::South { return 9281; }
        if block_state.r#facing == Facing::North && block_state.r#age == 2 { return 9288; }
        if block_state.r#facing == Facing::East && block_state.r#age == 0 { return 9283; }
        if block_state.r#age == 0 && block_state.r#facing == Facing::West { return 9282; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9287 {
            return Some(Cocoa {
                r#age: 1,
                r#facing: Facing::East,
            });
        }
        if state_id == 9290 {
            return Some(Cocoa {
                r#age: 2,
                r#facing: Facing::West,
            });
        }
        if state_id == 9291 {
            return Some(Cocoa {
                r#facing: Facing::East,
                r#age: 2,
            });
        }
        if state_id == 9284 {
            return Some(Cocoa {
                r#age: 1,
                r#facing: Facing::North,
            });
        }
        if state_id == 9286 {
            return Some(Cocoa {
                r#facing: Facing::West,
                r#age: 1,
            });
        }
        if state_id == 9285 {
            return Some(Cocoa {
                r#facing: Facing::South,
                r#age: 1,
            });
        }
        if state_id == 9289 {
            return Some(Cocoa {
                r#age: 2,
                r#facing: Facing::South,
            });
        }
        if state_id == 9280 {
            return Some(Cocoa {
                r#age: 0,
                r#facing: Facing::North,
            });
        }
        if state_id == 9281 {
            return Some(Cocoa {
                r#age: 0,
                r#facing: Facing::South,
            });
        }
        if state_id == 9288 {
            return Some(Cocoa {
                r#facing: Facing::North,
                r#age: 2,
            });
        }
        if state_id == 9283 {
            return Some(Cocoa {
                r#facing: Facing::East,
                r#age: 0,
            });
        }
        if state_id == 9282 {
            return Some(Cocoa {
                r#age: 0,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

