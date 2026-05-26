use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightBlueConcrete {}

impl BlockState for LightBlueConcrete {
    fn to_id(&self) -> i32 {
        return 14831;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14831 {
            return Some(LightBlueConcrete {});
        }
        return None;
    }
}
