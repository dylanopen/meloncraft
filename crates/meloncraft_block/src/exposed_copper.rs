use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCopper {
}


impl BlockState for ExposedCopper {
    fn to_id(&self) -> i32 {
        return 25108;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25108 {
            return Some(ExposedCopper {
            });
        }
        return None;
    }
}

