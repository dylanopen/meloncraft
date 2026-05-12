use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightGrayConcrete {
}


impl BlockState for LightGrayConcrete {
    fn to_id(self) -> i32 {
        return 14836;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14836 {
            return Some(LightGrayConcrete {
            });
        }
        return None;
    }
}

