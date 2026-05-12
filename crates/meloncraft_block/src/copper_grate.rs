use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperGrate {
    pub waterlogged: bool,
}


impl BlockState for CopperGrate {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true { return 26845; }
        if self.r#waterlogged == false { return 26846; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26845 {
            return Some(CopperGrate {
                r#waterlogged: true,
            });
        }
        if state_id == 26846 {
            return Some(CopperGrate {
                r#waterlogged: false,
            });
        }
        return None;
    }
}

