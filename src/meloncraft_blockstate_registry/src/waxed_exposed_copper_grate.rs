use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedCopperGrate {
    pub waterlogged: bool,
}

impl BlockState for WaxedExposedCopperGrate {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true {
            return 26855;
        }
        if self.r#waterlogged == false {
            return 26856;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26855 {
            return Some(WaxedExposedCopperGrate {
                r#waterlogged: true,
            });
        }
        if state_id == 26856 {
            return Some(WaxedExposedCopperGrate {
                r#waterlogged: false,
            });
        }
        return None;
    }
}
