use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mud {}

impl BlockState for Mud {
    fn to_id(&self) -> i32 {
        return 27720;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27720 {
            return Some(Mud {});
        }
        return None;
    }
}
