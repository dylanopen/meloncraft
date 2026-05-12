use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCopperLantern {
    pub hanging: bool,
    pub waterlogged: bool,
}


impl BlockState for OxidizedCopperLantern {
    fn to_id(&self) -> i32 {
        if self.r#hanging == true && self.r#waterlogged == false { return 20656; }
        if self.r#hanging == true && self.r#waterlogged == true { return 20655; }
        if self.r#hanging == false && self.r#waterlogged == true { return 20657; }
        if self.r#hanging == false && self.r#waterlogged == false { return 20658; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20656 {
            return Some(OxidizedCopperLantern {
                r#hanging: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20655 {
            return Some(OxidizedCopperLantern {
                r#hanging: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20657 {
            return Some(OxidizedCopperLantern {
                r#hanging: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20658 {
            return Some(OxidizedCopperLantern {
                r#hanging: false,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

