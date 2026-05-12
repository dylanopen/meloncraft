use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LilyOfTheValley {
}


impl BlockState for LilyOfTheValley {
    fn to_id(&self) -> i32 {
        return 2134;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2134 {
            return Some(LilyOfTheValley {
            });
        }
        return None;
    }
}

