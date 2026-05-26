use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCopperGrate {
    pub waterlogged: bool,
}

impl BlockState for WeatheredCopperGrate {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true {
            return 26849;
        }
        if self.r#waterlogged == false {
            return 26850;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26849 {
            return Some(WeatheredCopperGrate {
                r#waterlogged: true,
            });
        }
        if state_id == 26850 {
            return Some(WeatheredCopperGrate {
                r#waterlogged: false,
            });
        }
        return None;
    }
}
