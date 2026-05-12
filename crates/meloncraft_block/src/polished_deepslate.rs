use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedDeepslate {
}


impl BlockState for PolishedDeepslate {
    fn to_id(self) -> i32 {
        return 28135;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 28135 {
            return Some(PolishedDeepslate {
            });
        }
        return None;
    }
}

