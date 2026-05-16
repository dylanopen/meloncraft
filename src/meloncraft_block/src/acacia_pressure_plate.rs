use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaPressurePlate {
    pub powered: bool,
}


impl BlockState for AcaciaPressurePlate {
    fn to_id(&self) -> i32 {
        if self.r#powered == true { return 6668; }
        if self.r#powered == false { return 6669; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6668 {
            return Some(AcaciaPressurePlate {
                r#powered: true,
            });
        }
        if state_id == 6669 {
            return Some(AcaciaPressurePlate {
                r#powered: false,
            });
        }
        return None;
    }
}

