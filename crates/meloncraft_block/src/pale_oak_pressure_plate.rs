use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakPressurePlate {
    pub powered: bool,
}


impl BlockState for PaleOakPressurePlate {
    fn to_id(&self) -> i32 {
        if self.r#powered == true { return 6674; }
        if self.r#powered == false { return 6675; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6674 {
            return Some(PaleOakPressurePlate {
                r#powered: true,
            });
        }
        if state_id == 6675 {
            return Some(PaleOakPressurePlate {
                r#powered: false,
            });
        }
        return None;
    }
}

