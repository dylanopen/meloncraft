use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedCopperLantern {
    pub waterlogged: bool,
    pub hanging: bool,
}


impl BlockState for WaxedExposedCopperLantern {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#hanging == true { return 20664; }
        if self.r#waterlogged == true && self.r#hanging == false { return 20665; }
        if self.r#waterlogged == true && self.r#hanging == true { return 20663; }
        if self.r#hanging == false && self.r#waterlogged == false { return 20666; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20664 {
            return Some(WaxedExposedCopperLantern {
                r#waterlogged: false,
                r#hanging: true,
            });
        }
        if state_id == 20665 {
            return Some(WaxedExposedCopperLantern {
                r#waterlogged: true,
                r#hanging: false,
            });
        }
        if state_id == 20663 {
            return Some(WaxedExposedCopperLantern {
                r#waterlogged: true,
                r#hanging: true,
            });
        }
        if state_id == 20666 {
            return Some(WaxedExposedCopperLantern {
                r#hanging: false,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

