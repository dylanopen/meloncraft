use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCopperLantern {
    pub waterlogged: bool,
    pub hanging: bool,
}


impl BlockState for WaxedCopperLantern {
    fn to_id(&self) -> i32 {
        if self.r#hanging == true && self.r#waterlogged == false { return 20660; }
        if self.r#hanging == false && self.r#waterlogged == false { return 20662; }
        if self.r#hanging == true && self.r#waterlogged == true { return 20659; }
        if self.r#waterlogged == true && self.r#hanging == false { return 20661; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20660 {
            return Some(WaxedCopperLantern {
                r#hanging: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20662 {
            return Some(WaxedCopperLantern {
                r#hanging: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20659 {
            return Some(WaxedCopperLantern {
                r#hanging: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20661 {
            return Some(WaxedCopperLantern {
                r#waterlogged: true,
                r#hanging: false,
            });
        }
        return None;
    }
}

