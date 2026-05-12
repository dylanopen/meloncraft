use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCopperBulb {
    pub powered: bool,
    pub lit: bool,
}


impl BlockState for WaxedWeatheredCopperBulb {
    fn to_id(&self) -> i32 {
        if self.r#powered == true && self.r#lit == true { return 26885; }
        if self.r#lit == true && self.r#powered == false { return 26886; }
        if self.r#lit == false && self.r#powered == true { return 26887; }
        if self.r#lit == false && self.r#powered == false { return 26888; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26885 {
            return Some(WaxedWeatheredCopperBulb {
                r#powered: true,
                r#lit: true,
            });
        }
        if state_id == 26886 {
            return Some(WaxedWeatheredCopperBulb {
                r#lit: true,
                r#powered: false,
            });
        }
        if state_id == 26887 {
            return Some(WaxedWeatheredCopperBulb {
                r#lit: false,
                r#powered: true,
            });
        }
        if state_id == 26888 {
            return Some(WaxedWeatheredCopperBulb {
                r#lit: false,
                r#powered: false,
            });
        }
        return None;
    }
}

