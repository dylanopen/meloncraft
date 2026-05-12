use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloweringAzalea {
}


impl BlockState for FloweringAzalea {
    fn to_id(self) -> i32 {
        return 27610;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27610 {
            return Some(FloweringAzalea {
            });
        }
        return None;
    }
}

