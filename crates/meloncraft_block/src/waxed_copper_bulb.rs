use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCopperBulb {
    pub lit: bool,
    pub powered: bool,
}


impl BlockState for WaxedCopperBulb {
    fn to_id(self) -> i32 {
        if block_state.r#lit == false && block_state.r#powered == false { return 26880; }
        if block_state.r#lit == true && block_state.r#powered == true { return 26877; }
        if block_state.r#lit == true && block_state.r#powered == false { return 26878; }
        if block_state.r#lit == false && block_state.r#powered == true { return 26879; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26880 {
            return Some(WaxedCopperBulb {
                r#lit: false,
                r#powered: false,
            });
        }
        if state_id == 26877 {
            return Some(WaxedCopperBulb {
                r#lit: true,
                r#powered: true,
            });
        }
        if state_id == 26878 {
            return Some(WaxedCopperBulb {
                r#lit: true,
                r#powered: false,
            });
        }
        if state_id == 26879 {
            return Some(WaxedCopperBulb {
                r#lit: false,
                r#powered: true,
            });
        }
        return None;
    }
}

