use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TubeCoral {
    pub waterlogged: bool,
}


impl BlockState for TubeCoral {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false { return 14956; }
        if self.r#waterlogged == true { return 14955; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14956 {
            return Some(TubeCoral {
                r#waterlogged: false,
            });
        }
        if state_id == 14955 {
            return Some(TubeCoral {
                r#waterlogged: true,
            });
        }
        return None;
    }
}

