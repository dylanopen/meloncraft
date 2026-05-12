use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCopperBulb {
    pub lit: bool,
    pub powered: bool,
}


impl BlockState for OxidizedCopperBulb {
    fn to_id(self) -> i32 {
        if block_state.r#lit == true && block_state.r#powered == false { return 26874; }
        if block_state.r#lit == false && block_state.r#powered == true { return 26875; }
        if block_state.r#lit == false && block_state.r#powered == false { return 26876; }
        if block_state.r#lit == true && block_state.r#powered == true { return 26873; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26874 {
            return Some(OxidizedCopperBulb {
                r#lit: true,
                r#powered: false,
            });
        }
        if state_id == 26875 {
            return Some(OxidizedCopperBulb {
                r#lit: false,
                r#powered: true,
            });
        }
        if state_id == 26876 {
            return Some(OxidizedCopperBulb {
                r#lit: false,
                r#powered: false,
            });
        }
        if state_id == 26873 {
            return Some(OxidizedCopperBulb {
                r#lit: true,
                r#powered: true,
            });
        }
        return None;
    }
}

