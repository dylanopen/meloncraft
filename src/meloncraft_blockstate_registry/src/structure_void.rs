use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructureVoid {}

impl BlockState for StructureVoid {
    fn to_id(&self) -> i32 {
        return 14649;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14649 {
            return Some(StructureVoid {});
        }
        return None;
    }
}
