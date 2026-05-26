use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCopperGrate {
    pub waterlogged: bool,
}

impl BlockState for OxidizedCopperGrate {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true {
            return 26851;
        }
        if self.r#waterlogged == false {
            return 26852;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26851 {
            return Some(OxidizedCopperGrate {
                r#waterlogged: true,
            });
        }
        if state_id == 26852 {
            return Some(OxidizedCopperGrate {
                r#waterlogged: false,
            });
        }
        return None;
    }
}
