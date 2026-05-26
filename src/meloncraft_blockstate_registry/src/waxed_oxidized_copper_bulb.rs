use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCopperBulb {
    pub powered: bool,
    pub lit: bool,
}

impl BlockState for WaxedOxidizedCopperBulb {
    fn to_id(&self) -> i32 {
        if self.r#lit == true && self.r#powered == true {
            return 26889;
        }
        if self.r#powered == true && self.r#lit == false {
            return 26891;
        }
        if self.r#powered == false && self.r#lit == false {
            return 26892;
        }
        if self.r#lit == true && self.r#powered == false {
            return 26890;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26889 {
            return Some(WaxedOxidizedCopperBulb {
                r#lit: true,
                r#powered: true,
            });
        }
        if state_id == 26891 {
            return Some(WaxedOxidizedCopperBulb {
                r#powered: true,
                r#lit: false,
            });
        }
        if state_id == 26892 {
            return Some(WaxedOxidizedCopperBulb {
                r#powered: false,
                r#lit: false,
            });
        }
        if state_id == 26890 {
            return Some(WaxedOxidizedCopperBulb {
                r#lit: true,
                r#powered: false,
            });
        }
        return None;
    }
}
