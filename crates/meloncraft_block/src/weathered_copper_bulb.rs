use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCopperBulb {
    pub powered: bool,
    pub lit: bool,
}


impl BlockState for WeatheredCopperBulb {
    fn to_id(self) -> i32 {
        if block_state.r#powered == false && block_state.r#lit == true { return 26870; }
        if block_state.r#lit == true && block_state.r#powered == true { return 26869; }
        if block_state.r#lit == false && block_state.r#powered == false { return 26872; }
        if block_state.r#lit == false && block_state.r#powered == true { return 26871; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26870 {
            return Some(WeatheredCopperBulb {
                r#powered: false,
                r#lit: true,
            });
        }
        if state_id == 26869 {
            return Some(WeatheredCopperBulb {
                r#lit: true,
                r#powered: true,
            });
        }
        if state_id == 26872 {
            return Some(WeatheredCopperBulb {
                r#lit: false,
                r#powered: false,
            });
        }
        if state_id == 26871 {
            return Some(WeatheredCopperBulb {
                r#lit: false,
                r#powered: true,
            });
        }
        return None;
    }
}

