use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateGoldOre {}

impl BlockState for DeepslateGoldOre {
    fn to_id(&self) -> i32 {
        return 130;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 130 {
            return Some(DeepslateGoldOre {});
        }
        return None;
    }
}
