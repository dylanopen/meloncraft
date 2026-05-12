use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCopperBulb {
    pub powered: bool,
    pub lit: bool,
}


impl BlockState for WaxedOxidizedCopperBulb {
    fn to_id(self) -> i32 {
        if block_state.r#powered == false && block_state.r#lit == true { return 26890; }
        if block_state.r#powered == true && block_state.r#lit == true { return 26889; }
        if block_state.r#powered == true && block_state.r#lit == false { return 26891; }
        if block_state.r#lit == false && block_state.r#powered == false { return 26892; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26890 {
            return Some(WaxedOxidizedCopperBulb {
                r#powered: false,
                r#lit: true,
            });
        }
        if state_id == 26889 {
            return Some(WaxedOxidizedCopperBulb {
                r#powered: true,
                r#lit: true,
            });
        }
        if state_id == 26891 {
            return Some(WaxedOxidizedCopperBulb {
                r#powered: true,
                r#lit: false,
            });
        }
        if state_id == 26892 {
            return Some(WaxedOxidizedCopperBulb {
                r#lit: false,
                r#powered: false,
            });
        }
        return None;
    }
}

