use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AncientDebris {}

impl BlockState for AncientDebris {
    fn to_id(&self) -> i32 {
        return 21617;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21617 {
            return Some(AncientDebris {});
        }
        return None;
    }
}
