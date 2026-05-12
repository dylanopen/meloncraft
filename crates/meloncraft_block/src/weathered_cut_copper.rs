use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCutCopper {
}


impl BlockState for WeatheredCutCopper {
    fn to_id(&self) -> i32 {
        return 25114;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25114 {
            return Some(WeatheredCutCopper {
            });
        }
        return None;
    }
}

