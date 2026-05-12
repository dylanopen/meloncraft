use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCopperGrate {
    pub waterlogged: bool,
}


impl BlockState for ExposedCopperGrate {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false { return 26848; }
        if self.r#waterlogged == true { return 26847; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26848 {
            return Some(ExposedCopperGrate {
                r#waterlogged: false,
            });
        }
        if state_id == 26847 {
            return Some(ExposedCopperGrate {
                r#waterlogged: true,
            });
        }
        return None;
    }
}

