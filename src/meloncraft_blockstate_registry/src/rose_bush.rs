use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoseBush {
    pub r#half: Half,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

impl BlockState for RoseBush {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Upper {
            return 12717;
        }
        if self.r#half == Half::Lower {
            return 12718;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12717 {
            return Some(RoseBush {
                r#half: Half::Upper,
            });
        }
        if state_id == 12718 {
            return Some(RoseBush {
                r#half: Half::Lower,
            });
        }
        return None;
    }
}
