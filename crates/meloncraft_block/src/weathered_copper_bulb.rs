use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCopperBulb {
    pub powered: bool,
    pub lit: bool,
}


impl BlockState for WeatheredCopperBulb {
    fn to_id(&self) -> i32 {
        if self.r#powered == false && self.r#lit == false { return 26872; }
        if self.r#lit == false && self.r#powered == true { return 26871; }
        if self.r#lit == true && self.r#powered == false { return 26870; }
        if self.r#powered == true && self.r#lit == true { return 26869; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26872 {
            return Some(WeatheredCopperBulb {
                r#powered: false,
                r#lit: false,
            });
        }
        if state_id == 26871 {
            return Some(WeatheredCopperBulb {
                r#lit: false,
                r#powered: true,
            });
        }
        if state_id == 26870 {
            return Some(WeatheredCopperBulb {
                r#lit: true,
                r#powered: false,
            });
        }
        if state_id == 26869 {
            return Some(WeatheredCopperBulb {
                r#powered: true,
                r#lit: true,
            });
        }
        return None;
    }
}

