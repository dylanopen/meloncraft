use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCopperBulb {
    pub powered: bool,
    pub lit: bool,
}

impl BlockState for ExposedCopperBulb {
    fn to_id(&self) -> i32 {
        if self.r#lit == true && self.r#powered == true {
            return 26865;
        }
        if self.r#powered == false && self.r#lit == false {
            return 26868;
        }
        if self.r#lit == false && self.r#powered == true {
            return 26867;
        }
        if self.r#powered == false && self.r#lit == true {
            return 26866;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26865 {
            return Some(ExposedCopperBulb {
                r#lit: true,
                r#powered: true,
            });
        }
        if state_id == 26868 {
            return Some(ExposedCopperBulb {
                r#powered: false,
                r#lit: false,
            });
        }
        if state_id == 26867 {
            return Some(ExposedCopperBulb {
                r#lit: false,
                r#powered: true,
            });
        }
        if state_id == 26866 {
            return Some(ExposedCopperBulb {
                r#powered: false,
                r#lit: true,
            });
        }
        return None;
    }
}
