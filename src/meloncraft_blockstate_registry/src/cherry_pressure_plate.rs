use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryPressurePlate {
    pub powered: bool,
}

impl BlockState for CherryPressurePlate {
    fn to_id(&self) -> i32 {
        if self.r#powered == false {
            return 6671;
        }
        if self.r#powered == true {
            return 6670;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6671 {
            return Some(CherryPressurePlate { r#powered: false });
        }
        if state_id == 6670 {
            return Some(CherryPressurePlate { r#powered: true });
        }
        return None;
    }
}
