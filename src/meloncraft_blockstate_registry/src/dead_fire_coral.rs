use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadFireCoral {
    pub waterlogged: bool,
}

impl BlockState for DeadFireCoral {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false {
            return 14952;
        }
        if self.r#waterlogged == true {
            return 14951;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14952 {
            return Some(DeadFireCoral {
                r#waterlogged: false,
            });
        }
        if state_id == 14951 {
            return Some(DeadFireCoral {
                r#waterlogged: true,
            });
        }
        return None;
    }
}
