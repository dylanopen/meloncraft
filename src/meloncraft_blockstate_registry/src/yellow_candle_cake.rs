use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YellowCandleCake {
    pub lit: bool,
}

impl BlockState for YellowCandleCake {
    fn to_id(&self) -> i32 {
        if self.r#lit == true {
            return 23176;
        }
        if self.r#lit == false {
            return 23177;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23176 {
            return Some(YellowCandleCake { r#lit: true });
        }
        if state_id == 23177 {
            return Some(YellowCandleCake { r#lit: false });
        }
        return None;
    }
}
