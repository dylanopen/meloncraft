use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCopperGrate {
    pub waterlogged: bool,
}

impl BlockState for WaxedCopperGrate {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true {
            return 26853;
        }
        if self.r#waterlogged == false {
            return 26854;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26853 {
            return Some(WaxedCopperGrate {
                r#waterlogged: true,
            });
        }
        if state_id == 26854 {
            return Some(WaxedCopperGrate {
                r#waterlogged: false,
            });
        }
        return None;
    }
}
