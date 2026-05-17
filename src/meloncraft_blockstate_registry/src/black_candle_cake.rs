use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackCandleCake {
    pub lit: bool,
}


impl BlockState for BlackCandleCake {
    fn to_id(&self) -> i32 {
        if self.r#lit == false { return 23199; }
        if self.r#lit == true { return 23198; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23199 {
            return Some(BlackCandleCake {
                r#lit: false,
            });
        }
        if state_id == 23198 {
            return Some(BlackCandleCake {
                r#lit: true,
            });
        }
        return None;
    }
}

