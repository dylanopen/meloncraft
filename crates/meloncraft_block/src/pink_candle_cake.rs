use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkCandleCake {
    pub lit: bool,
}


impl BlockState for PinkCandleCake {
    fn to_id(&self) -> i32 {
        if self.r#lit == true { return 23180; }
        if self.r#lit == false { return 23181; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23180 {
            return Some(PinkCandleCake {
                r#lit: true,
            });
        }
        if state_id == 23181 {
            return Some(PinkCandleCake {
                r#lit: false,
            });
        }
        return None;
    }
}

