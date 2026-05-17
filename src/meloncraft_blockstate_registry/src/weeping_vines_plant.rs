use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeepingVinesPlant {
}


impl BlockState for WeepingVinesPlant {
    fn to_id(&self) -> i32 {
        return 20801;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20801 {
            return Some(WeepingVinesPlant {
            });
        }
        return None;
    }
}

