use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightBlueCandleCake {
    pub lit: bool,
}


impl BlockState for LightBlueCandleCake {
    fn to_id(&self) -> i32 {
        if self.r#lit == false { return 23175; }
        if self.r#lit == true { return 23174; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23175 {
            return Some(LightBlueCandleCake {
                r#lit: false,
            });
        }
        if state_id == 23174 {
            return Some(LightBlueCandleCake {
                r#lit: true,
            });
        }
        return None;
    }
}

