use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Azalea {}

impl BlockState for Azalea {
    fn to_id(&self) -> i32 {
        return 27609;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27609 {
            return Some(Azalea {});
        }
        return None;
    }
}
