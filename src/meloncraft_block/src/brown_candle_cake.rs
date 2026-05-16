use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrownCandleCake {
    pub lit: bool,
}


impl BlockState for BrownCandleCake {
    fn to_id(&self) -> i32 {
        if self.r#lit == true { return 23192; }
        if self.r#lit == false { return 23193; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23192 {
            return Some(BrownCandleCake {
                r#lit: true,
            });
        }
        if state_id == 23193 {
            return Some(BrownCandleCake {
                r#lit: false,
            });
        }
        return None;
    }
}

