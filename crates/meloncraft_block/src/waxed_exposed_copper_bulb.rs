use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedCopperBulb {
    pub lit: bool,
    pub powered: bool,
}


impl BlockState for WaxedExposedCopperBulb {
    fn to_id(self) -> i32 {
        if block_state.r#lit == false && block_state.r#powered == true { return 26883; }
        if block_state.r#lit == true && block_state.r#powered == false { return 26882; }
        if block_state.r#lit == true && block_state.r#powered == true { return 26881; }
        if block_state.r#lit == false && block_state.r#powered == false { return 26884; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26883 {
            return Some(WaxedExposedCopperBulb {
                r#lit: false,
                r#powered: true,
            });
        }
        if state_id == 26882 {
            return Some(WaxedExposedCopperBulb {
                r#lit: true,
                r#powered: false,
            });
        }
        if state_id == 26881 {
            return Some(WaxedExposedCopperBulb {
                r#lit: true,
                r#powered: true,
            });
        }
        if state_id == 26884 {
            return Some(WaxedExposedCopperBulb {
                r#lit: false,
                r#powered: false,
            });
        }
        return None;
    }
}

