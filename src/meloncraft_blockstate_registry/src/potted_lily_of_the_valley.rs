use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedLilyOfTheValley {}

impl BlockState for PottedLilyOfTheValley {
    fn to_id(&self) -> i32 {
        return 10451;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10451 {
            return Some(PottedLilyOfTheValley {});
        }
        return None;
    }
}
