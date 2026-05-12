use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowerPot {
}


impl BlockState for FlowerPot {
    fn to_id(self) -> i32 {
        return 10428;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10428 {
            return Some(FlowerPot {
            });
        }
        return None;
    }
}

