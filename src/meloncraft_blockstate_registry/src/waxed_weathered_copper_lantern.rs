use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCopperLantern {
    pub waterlogged: bool,
    pub hanging: bool,
}


impl BlockState for WaxedWeatheredCopperLantern {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#hanging == false { return 20669; }
        if self.r#hanging == true && self.r#waterlogged == true { return 20667; }
        if self.r#hanging == true && self.r#waterlogged == false { return 20668; }
        if self.r#hanging == false && self.r#waterlogged == false { return 20670; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20669 {
            return Some(WaxedWeatheredCopperLantern {
                r#waterlogged: true,
                r#hanging: false,
            });
        }
        if state_id == 20667 {
            return Some(WaxedWeatheredCopperLantern {
                r#hanging: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20668 {
            return Some(WaxedWeatheredCopperLantern {
                r#hanging: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20670 {
            return Some(WaxedWeatheredCopperLantern {
                r#hanging: false,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

