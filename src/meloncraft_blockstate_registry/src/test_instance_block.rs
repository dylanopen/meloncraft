use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestInstanceBlock {}

impl BlockState for TestInstanceBlock {
    fn to_id(&self) -> i32 {
        return 21540;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21540 {
            return Some(TestInstanceBlock {});
        }
        return None;
    }
}
