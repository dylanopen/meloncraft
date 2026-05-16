use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedCopperBulb {
    pub powered: bool,
    pub lit: bool,
}


impl BlockState for WaxedExposedCopperBulb {
    fn to_id(&self) -> i32 {
        if self.r#lit == true && self.r#powered == true { return 26881; }
        if self.r#lit == true && self.r#powered == false { return 26882; }
        if self.r#lit == false && self.r#powered == true { return 26883; }
        if self.r#powered == false && self.r#lit == false { return 26884; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26881 {
            return Some(WaxedExposedCopperBulb {
                r#lit: true,
                r#powered: true,
            });
        }
        if state_id == 26882 {
            return Some(WaxedExposedCopperBulb {
                r#lit: true,
                r#powered: false,
            });
        }
        if state_id == 26883 {
            return Some(WaxedExposedCopperBulb {
                r#lit: false,
                r#powered: true,
            });
        }
        if state_id == 26884 {
            return Some(WaxedExposedCopperBulb {
                r#powered: false,
                r#lit: false,
            });
        }
        return None;
    }
}

