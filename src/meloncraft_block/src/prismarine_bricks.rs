use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrismarineBricks {
}


impl BlockState for PrismarineBricks {
    fn to_id(&self) -> i32 {
        return 12430;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12430 {
            return Some(PrismarineBricks {
            });
        }
        return None;
    }
}

