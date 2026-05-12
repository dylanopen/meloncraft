use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrostedIce {
    pub age: i32,
}


impl BlockState for FrostedIce {
    fn to_id(self) -> i32 {
        if block_state.r#age == 2 { return 14641; }
        if block_state.r#age == 0 { return 14639; }
        if block_state.r#age == 1 { return 14640; }
        if block_state.r#age == 3 { return 14642; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14641 {
            return Some(FrostedIce {
                r#age: 2,
            });
        }
        if state_id == 14639 {
            return Some(FrostedIce {
                r#age: 0,
            });
        }
        if state_id == 14640 {
            return Some(FrostedIce {
                r#age: 1,
            });
        }
        if state_id == 14642 {
            return Some(FrostedIce {
                r#age: 3,
            });
        }
        return None;
    }
}

