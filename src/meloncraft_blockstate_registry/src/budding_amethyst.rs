use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuddingAmethyst {}

impl BlockState for BuddingAmethyst {
    fn to_id(&self) -> i32 {
        return 23201;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23201 {
            return Some(BuddingAmethyst {});
        }
        return None;
    }
}
