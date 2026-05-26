use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaConcrete {}

impl BlockState for MagentaConcrete {
    fn to_id(&self) -> i32 {
        return 14830;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14830 {
            return Some(MagentaConcrete {});
        }
        return None;
    }
}
