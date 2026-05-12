use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooPressurePlate {
    pub powered: bool,
}


impl BlockState for BambooPressurePlate {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true { return 6678; }
        if block_state.r#powered == false { return 6679; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6678 {
            return Some(BambooPressurePlate {
                r#powered: true,
            });
        }
        if state_id == 6679 {
            return Some(BambooPressurePlate {
                r#powered: false,
            });
        }
        return None;
    }
}

