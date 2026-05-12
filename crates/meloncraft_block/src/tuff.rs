use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tuff {
}


impl BlockState for Tuff {
    fn to_id(&self) -> i32 {
        return 23250;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23250 {
            return Some(Tuff {
            });
        }
        return None;
    }
}

