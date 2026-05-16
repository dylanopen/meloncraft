use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaterCauldron {
    pub level: i32,
}


impl BlockState for WaterCauldron {
    fn to_id(&self) -> i32 {
        if self.r#level == 1 { return 9260; }
        if self.r#level == 2 { return 9261; }
        if self.r#level == 3 { return 9262; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9260 {
            return Some(WaterCauldron {
                r#level: 1,
            });
        }
        if state_id == 9261 {
            return Some(WaterCauldron {
                r#level: 2,
            });
        }
        if state_id == 9262 {
            return Some(WaterCauldron {
                r#level: 3,
            });
        }
        return None;
    }
}

