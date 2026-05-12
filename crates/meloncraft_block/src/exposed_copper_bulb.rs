use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCopperBulb {
    pub lit: bool,
    pub powered: bool,
}


impl BlockState for ExposedCopperBulb {
    fn to_id(self) -> i32 {
        if block_state.r#powered == false && block_state.r#lit == true { return 26866; }
        if block_state.r#lit == false && block_state.r#powered == true { return 26867; }
        if block_state.r#powered == false && block_state.r#lit == false { return 26868; }
        if block_state.r#powered == true && block_state.r#lit == true { return 26865; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26866 {
            return Some(ExposedCopperBulb {
                r#powered: false,
                r#lit: true,
            });
        }
        if state_id == 26867 {
            return Some(ExposedCopperBulb {
                r#lit: false,
                r#powered: true,
            });
        }
        if state_id == 26868 {
            return Some(ExposedCopperBulb {
                r#powered: false,
                r#lit: false,
            });
        }
        if state_id == 26865 {
            return Some(ExposedCopperBulb {
                r#powered: true,
                r#lit: true,
            });
        }
        return None;
    }
}

