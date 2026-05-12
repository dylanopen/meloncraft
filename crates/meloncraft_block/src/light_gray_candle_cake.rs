use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightGrayCandleCake {
    pub lit: bool,
}


impl BlockState for LightGrayCandleCake {
    fn to_id(&self) -> i32 {
        if self.r#lit == false { return 23185; }
        if self.r#lit == true { return 23184; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23185 {
            return Some(LightGrayCandleCake {
                r#lit: false,
            });
        }
        if state_id == 23184 {
            return Some(LightGrayCandleCake {
                r#lit: true,
            });
        }
        return None;
    }
}

