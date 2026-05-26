use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperBulb {
    pub lit: bool,
    pub powered: bool,
}

impl BlockState for CopperBulb {
    fn to_id(&self) -> i32 {
        if self.r#lit == true && self.r#powered == true {
            return 26861;
        }
        if self.r#lit == true && self.r#powered == false {
            return 26862;
        }
        if self.r#lit == false && self.r#powered == false {
            return 26864;
        }
        if self.r#powered == true && self.r#lit == false {
            return 26863;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26861 {
            return Some(CopperBulb {
                r#lit: true,
                r#powered: true,
            });
        }
        if state_id == 26862 {
            return Some(CopperBulb {
                r#lit: true,
                r#powered: false,
            });
        }
        if state_id == 26864 {
            return Some(CopperBulb {
                r#lit: false,
                r#powered: false,
            });
        }
        if state_id == 26863 {
            return Some(CopperBulb {
                r#powered: true,
                r#lit: false,
            });
        }
        return None;
    }
}
