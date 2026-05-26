use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleHangingMoss {
    pub tip: bool,
}

impl BlockState for PaleHangingMoss {
    fn to_id(&self) -> i32 {
        if self.r#tip == true {
            return 29664;
        }
        if self.r#tip == false {
            return 29665;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29664 {
            return Some(PaleHangingMoss { r#tip: true });
        }
        if state_id == 29665 {
            return Some(PaleHangingMoss { r#tip: false });
        }
        return None;
    }
}
