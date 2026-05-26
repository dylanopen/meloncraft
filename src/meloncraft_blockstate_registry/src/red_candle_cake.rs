use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedCandleCake {
    pub lit: bool,
}

impl BlockState for RedCandleCake {
    fn to_id(&self) -> i32 {
        if self.r#lit == false {
            return 23197;
        }
        if self.r#lit == true {
            return 23196;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23197 {
            return Some(RedCandleCake { r#lit: false });
        }
        if state_id == 23196 {
            return Some(RedCandleCake { r#lit: true });
        }
        return None;
    }
}
