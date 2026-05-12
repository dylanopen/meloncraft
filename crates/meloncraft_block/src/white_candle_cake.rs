use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteCandleCake {
    pub lit: bool,
}


impl BlockState for WhiteCandleCake {
    fn to_id(&self) -> i32 {
        if self.r#lit == true { return 23168; }
        if self.r#lit == false { return 23169; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23168 {
            return Some(WhiteCandleCake {
                r#lit: true,
            });
        }
        if state_id == 23169 {
            return Some(WhiteCandleCake {
                r#lit: false,
            });
        }
        return None;
    }
}

