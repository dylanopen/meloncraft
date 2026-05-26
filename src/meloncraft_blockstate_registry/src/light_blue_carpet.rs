use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightBlueCarpet {}

impl BlockState for LightBlueCarpet {
    fn to_id(&self) -> i32 {
        return 12697;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12697 {
            return Some(LightBlueCarpet {});
        }
        return None;
    }
}
