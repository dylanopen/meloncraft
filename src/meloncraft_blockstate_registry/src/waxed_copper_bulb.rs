use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCopperBulb {
    pub lit: bool,
    pub powered: bool,
}

impl BlockState for WaxedCopperBulb {
    fn to_id(&self) -> i32 {
        if self.r#powered == false && self.r#lit == false {
            return 26880;
        }
        if self.r#lit == true && self.r#powered == true {
            return 26877;
        }
        if self.r#lit == false && self.r#powered == true {
            return 26879;
        }
        if self.r#lit == true && self.r#powered == false {
            return 26878;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26880 {
            return Some(WaxedCopperBulb {
                r#powered: false,
                r#lit: false,
            });
        }
        if state_id == 26877 {
            return Some(WaxedCopperBulb {
                r#lit: true,
                r#powered: true,
            });
        }
        if state_id == 26879 {
            return Some(WaxedCopperBulb {
                r#lit: false,
                r#powered: true,
            });
        }
        if state_id == 26878 {
            return Some(WaxedCopperBulb {
                r#lit: true,
                r#powered: false,
            });
        }
        return None;
    }
}
