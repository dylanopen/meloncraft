use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Farmland {
    pub moisture: i32,
}


impl BlockState for Farmland {
    fn to_id(self) -> i32 {
        if block_state.r#moisture == 3 { return 5121; }
        if block_state.r#moisture == 5 { return 5123; }
        if block_state.r#moisture == 2 { return 5120; }
        if block_state.r#moisture == 0 { return 5118; }
        if block_state.r#moisture == 7 { return 5125; }
        if block_state.r#moisture == 4 { return 5122; }
        if block_state.r#moisture == 1 { return 5119; }
        if block_state.r#moisture == 6 { return 5124; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5121 {
            return Some(Farmland {
                r#moisture: 3,
            });
        }
        if state_id == 5123 {
            return Some(Farmland {
                r#moisture: 5,
            });
        }
        if state_id == 5120 {
            return Some(Farmland {
                r#moisture: 2,
            });
        }
        if state_id == 5118 {
            return Some(Farmland {
                r#moisture: 0,
            });
        }
        if state_id == 5125 {
            return Some(Farmland {
                r#moisture: 7,
            });
        }
        if state_id == 5122 {
            return Some(Farmland {
                r#moisture: 4,
            });
        }
        if state_id == 5119 {
            return Some(Farmland {
                r#moisture: 1,
            });
        }
        if state_id == 5124 {
            return Some(Farmland {
                r#moisture: 6,
            });
        }
        return None;
    }
}

