use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Beetroots {
    pub age: i32,
}


impl BlockState for Beetroots {
    fn to_id(self) -> i32 {
        if block_state.r#age == 0 { return 14609; }
        if block_state.r#age == 3 { return 14612; }
        if block_state.r#age == 2 { return 14611; }
        if block_state.r#age == 1 { return 14610; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14609 {
            return Some(Beetroots {
                r#age: 0,
            });
        }
        if state_id == 14612 {
            return Some(Beetroots {
                r#age: 3,
            });
        }
        if state_id == 14611 {
            return Some(Beetroots {
                r#age: 2,
            });
        }
        if state_id == 14610 {
            return Some(Beetroots {
                r#age: 1,
            });
        }
        return None;
    }
}

