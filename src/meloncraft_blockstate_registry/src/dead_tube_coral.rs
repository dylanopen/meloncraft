use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadTubeCoral {
    pub waterlogged: bool,
}

impl BlockState for DeadTubeCoral {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true {
            return 14945;
        }
        if self.r#waterlogged == false {
            return 14946;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14945 {
            return Some(DeadTubeCoral {
                r#waterlogged: true,
            });
        }
        if state_id == 14946 {
            return Some(DeadTubeCoral {
                r#waterlogged: false,
            });
        }
        return None;
    }
}
