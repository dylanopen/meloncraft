use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateIronOre {
}


impl BlockState for DeepslateIronOre {
    fn to_id(&self) -> i32 {
        return 132;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 132 {
            return Some(DeepslateIronOre {
            });
        }
        return None;
    }
}

