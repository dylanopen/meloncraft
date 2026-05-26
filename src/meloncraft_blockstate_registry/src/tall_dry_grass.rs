use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TallDryGrass {}

impl BlockState for TallDryGrass {
    fn to_id(&self) -> i32 {
        return 2053;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2053 {
            return Some(TallDryGrass {});
        }
        return None;
    }
}
