use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FireflyBush {
}


impl BlockState for FireflyBush {
    fn to_id(self) -> i32 {
        return 29670;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29670 {
            return Some(FireflyBush {
            });
        }
        return None;
    }
}

