use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandleCake {
    pub lit: bool,
}

impl BlockState for CandleCake {
    fn to_id(&self) -> i32 {
        if self.r#lit == false {
            return 23167;
        }
        if self.r#lit == true {
            return 23166;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23167 {
            return Some(CandleCake { r#lit: false });
        }
        if state_id == 23166 {
            return Some(CandleCake { r#lit: true });
        }
        return None;
    }
}
