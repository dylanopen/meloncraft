use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YellowCarpet {
}


impl BlockState for YellowCarpet {
    fn to_id(self) -> i32 {
        return 12698;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12698 {
            return Some(YellowCarpet {
            });
        }
        return None;
    }
}

