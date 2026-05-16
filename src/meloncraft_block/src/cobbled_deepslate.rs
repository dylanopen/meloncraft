use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CobbledDeepslate {
}


impl BlockState for CobbledDeepslate {
    fn to_id(&self) -> i32 {
        return 27724;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27724 {
            return Some(CobbledDeepslate {
            });
        }
        return None;
    }
}

