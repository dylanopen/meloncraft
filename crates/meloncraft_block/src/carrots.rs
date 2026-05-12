use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Carrots {
    pub age: i32,
}


impl BlockState for Carrots {
    fn to_id(self) -> i32 {
        if block_state.r#age == 5 { return 10462; }
        if block_state.r#age == 0 { return 10457; }
        if block_state.r#age == 7 { return 10464; }
        if block_state.r#age == 1 { return 10458; }
        if block_state.r#age == 3 { return 10460; }
        if block_state.r#age == 2 { return 10459; }
        if block_state.r#age == 6 { return 10463; }
        if block_state.r#age == 4 { return 10461; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10462 {
            return Some(Carrots {
                r#age: 5,
            });
        }
        if state_id == 10457 {
            return Some(Carrots {
                r#age: 0,
            });
        }
        if state_id == 10464 {
            return Some(Carrots {
                r#age: 7,
            });
        }
        if state_id == 10458 {
            return Some(Carrots {
                r#age: 1,
            });
        }
        if state_id == 10460 {
            return Some(Carrots {
                r#age: 3,
            });
        }
        if state_id == 10459 {
            return Some(Carrots {
                r#age: 2,
            });
        }
        if state_id == 10463 {
            return Some(Carrots {
                r#age: 6,
            });
        }
        if state_id == 10461 {
            return Some(Carrots {
                r#age: 4,
            });
        }
        return None;
    }
}

