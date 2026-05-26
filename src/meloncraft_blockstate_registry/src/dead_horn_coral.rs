use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadHornCoral {
    pub waterlogged: bool,
}

impl BlockState for DeadHornCoral {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true {
            return 14953;
        }
        if self.r#waterlogged == false {
            return 14954;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14953 {
            return Some(DeadHornCoral {
                r#waterlogged: true,
            });
        }
        if state_id == 14954 {
            return Some(DeadHornCoral {
                r#waterlogged: false,
            });
        }
        return None;
    }
}
