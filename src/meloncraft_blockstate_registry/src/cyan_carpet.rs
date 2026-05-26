use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyanCarpet {}

impl BlockState for CyanCarpet {
    fn to_id(&self) -> i32 {
        return 12703;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12703 {
            return Some(CyanCarpet {});
        }
        return None;
    }
}
