use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaveVinesPlant {
    pub berries: bool,
}


impl BlockState for CaveVinesPlant {
    fn to_id(&self) -> i32 {
        if self.r#berries == false { return 27607; }
        if self.r#berries == true { return 27606; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27607 {
            return Some(CaveVinesPlant {
                r#berries: false,
            });
        }
        if state_id == 27606 {
            return Some(CaveVinesPlant {
                r#berries: true,
            });
        }
        return None;
    }
}

