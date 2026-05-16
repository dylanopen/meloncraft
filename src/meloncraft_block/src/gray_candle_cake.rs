use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayCandleCake {
    pub lit: bool,
}


impl BlockState for GrayCandleCake {
    fn to_id(&self) -> i32 {
        if self.r#lit == true { return 23182; }
        if self.r#lit == false { return 23183; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23182 {
            return Some(GrayCandleCake {
                r#lit: true,
            });
        }
        if state_id == 23183 {
            return Some(GrayCandleCake {
                r#lit: false,
            });
        }
        return None;
    }
}

