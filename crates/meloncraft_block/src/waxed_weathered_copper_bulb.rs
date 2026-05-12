use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCopperBulb {
    pub lit: bool,
    pub powered: bool,
}


impl BlockState for WaxedWeatheredCopperBulb {
    fn to_id(self) -> i32 {
        if block_state.r#lit == false && block_state.r#powered == true { return 26887; }
        if block_state.r#lit == true && block_state.r#powered == true { return 26885; }
        if block_state.r#lit == true && block_state.r#powered == false { return 26886; }
        if block_state.r#powered == false && block_state.r#lit == false { return 26888; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26887 {
            return Some(WaxedWeatheredCopperBulb {
                r#lit: false,
                r#powered: true,
            });
        }
        if state_id == 26885 {
            return Some(WaxedWeatheredCopperBulb {
                r#lit: true,
                r#powered: true,
            });
        }
        if state_id == 26886 {
            return Some(WaxedWeatheredCopperBulb {
                r#lit: true,
                r#powered: false,
            });
        }
        if state_id == 26888 {
            return Some(WaxedWeatheredCopperBulb {
                r#powered: false,
                r#lit: false,
            });
        }
        return None;
    }
}

