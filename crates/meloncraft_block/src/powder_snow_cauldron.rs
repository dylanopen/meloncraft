use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowderSnowCauldron {
    pub level: i32,
}


impl BlockState for PowderSnowCauldron {
    fn to_id(&self) -> i32 {
        if self.r#level == 3 { return 9266; }
        if self.r#level == 1 { return 9264; }
        if self.r#level == 2 { return 9265; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9266 {
            return Some(PowderSnowCauldron {
                r#level: 3,
            });
        }
        if state_id == 9264 {
            return Some(PowderSnowCauldron {
                r#level: 1,
            });
        }
        if state_id == 9265 {
            return Some(PowderSnowCauldron {
                r#level: 2,
            });
        }
        return None;
    }
}

