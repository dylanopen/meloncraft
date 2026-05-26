use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedAllium {}

impl BlockState for PottedAllium {
    fn to_id(&self) -> i32 {
        return 10443;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10443 {
            return Some(PottedAllium {});
        }
        return None;
    }
}
