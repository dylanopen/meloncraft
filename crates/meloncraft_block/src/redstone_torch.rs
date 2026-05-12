use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedstoneTorch {
    pub lit: bool,
}


impl BlockState for RedstoneTorch {
    fn to_id(&self) -> i32 {
        if self.r#lit == false { return 6685; }
        if self.r#lit == true { return 6684; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6685 {
            return Some(RedstoneTorch {
                r#lit: false,
            });
        }
        if state_id == 6684 {
            return Some(RedstoneTorch {
                r#lit: true,
            });
        }
        return None;
    }
}

