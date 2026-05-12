use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperBulb {
    pub powered: bool,
    pub lit: bool,
}


impl BlockState for CopperBulb {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#lit == true { return 26861; }
        if block_state.r#powered == false && block_state.r#lit == false { return 26864; }
        if block_state.r#lit == false && block_state.r#powered == true { return 26863; }
        if block_state.r#lit == true && block_state.r#powered == false { return 26862; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26861 {
            return Some(CopperBulb {
                r#powered: true,
                r#lit: true,
            });
        }
        if state_id == 26864 {
            return Some(CopperBulb {
                r#powered: false,
                r#lit: false,
            });
        }
        if state_id == 26863 {
            return Some(CopperBulb {
                r#lit: false,
                r#powered: true,
            });
        }
        if state_id == 26862 {
            return Some(CopperBulb {
                r#lit: true,
                r#powered: false,
            });
        }
        return None;
    }
}

