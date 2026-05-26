use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCopperBulb {
    pub powered: bool,
    pub lit: bool,
}

impl BlockState for OxidizedCopperBulb {
    fn to_id(&self) -> i32 {
        if self.r#lit == false && self.r#powered == true {
            return 26875;
        }
        if self.r#lit == false && self.r#powered == false {
            return 26876;
        }
        if self.r#lit == true && self.r#powered == true {
            return 26873;
        }
        if self.r#lit == true && self.r#powered == false {
            return 26874;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26875 {
            return Some(OxidizedCopperBulb {
                r#lit: false,
                r#powered: true,
            });
        }
        if state_id == 26876 {
            return Some(OxidizedCopperBulb {
                r#lit: false,
                r#powered: false,
            });
        }
        if state_id == 26873 {
            return Some(OxidizedCopperBulb {
                r#lit: true,
                r#powered: true,
            });
        }
        if state_id == 26874 {
            return Some(OxidizedCopperBulb {
                r#lit: true,
                r#powered: false,
            });
        }
        return None;
    }
}
