use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightBlueCandleCake {
    pub lit: bool,
}


impl BlockState for LightBlueCandleCake {
    fn to_id(self) -> i32 {
        if block_state.r#lit == true { return 23174; }
        if block_state.r#lit == false { return 23175; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23174 {
            return Some(LightBlueCandleCake {
                r#lit: true,
            });
        }
        if state_id == 23175 {
            return Some(LightBlueCandleCake {
                r#lit: false,
            });
        }
        return None;
    }
}

