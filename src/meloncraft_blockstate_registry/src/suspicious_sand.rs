use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuspiciousSand {
    pub dusted: i32,
}


impl BlockState for SuspiciousSand {
    fn to_id(&self) -> i32 {
        if self.r#dusted == 1 { return 120; }
        if self.r#dusted == 3 { return 122; }
        if self.r#dusted == 2 { return 121; }
        if self.r#dusted == 0 { return 119; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 120 {
            return Some(SuspiciousSand {
                r#dusted: 1,
            });
        }
        if state_id == 122 {
            return Some(SuspiciousSand {
                r#dusted: 3,
            });
        }
        if state_id == 121 {
            return Some(SuspiciousSand {
                r#dusted: 2,
            });
        }
        if state_id == 119 {
            return Some(SuspiciousSand {
                r#dusted: 0,
            });
        }
        return None;
    }
}

