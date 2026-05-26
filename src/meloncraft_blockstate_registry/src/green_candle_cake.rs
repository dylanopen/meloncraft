use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreenCandleCake {
    pub lit: bool,
}

impl BlockState for GreenCandleCake {
    fn to_id(&self) -> i32 {
        if self.r#lit == true {
            return 23194;
        }
        if self.r#lit == false {
            return 23195;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23194 {
            return Some(GreenCandleCake { r#lit: true });
        }
        if state_id == 23195 {
            return Some(GreenCandleCake { r#lit: false });
        }
        return None;
    }
}
