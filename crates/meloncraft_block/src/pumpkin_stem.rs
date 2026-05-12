use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PumpkinStem {
    pub age: i32,
}


impl BlockState for PumpkinStem {
    fn to_id(self) -> i32 {
        if block_state.r#age == 0 { return 8141; }
        if block_state.r#age == 6 { return 8147; }
        if block_state.r#age == 7 { return 8148; }
        if block_state.r#age == 3 { return 8144; }
        if block_state.r#age == 5 { return 8146; }
        if block_state.r#age == 2 { return 8143; }
        if block_state.r#age == 4 { return 8145; }
        if block_state.r#age == 1 { return 8142; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8141 {
            return Some(PumpkinStem {
                r#age: 0,
            });
        }
        if state_id == 8147 {
            return Some(PumpkinStem {
                r#age: 6,
            });
        }
        if state_id == 8148 {
            return Some(PumpkinStem {
                r#age: 7,
            });
        }
        if state_id == 8144 {
            return Some(PumpkinStem {
                r#age: 3,
            });
        }
        if state_id == 8146 {
            return Some(PumpkinStem {
                r#age: 5,
            });
        }
        if state_id == 8143 {
            return Some(PumpkinStem {
                r#age: 2,
            });
        }
        if state_id == 8145 {
            return Some(PumpkinStem {
                r#age: 4,
            });
        }
        if state_id == 8142 {
            return Some(PumpkinStem {
                r#age: 1,
            });
        }
        return None;
    }
}

