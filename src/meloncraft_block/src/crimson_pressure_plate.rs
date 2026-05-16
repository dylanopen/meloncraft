use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonPressurePlate {
    pub powered: bool,
}


impl BlockState for CrimsonPressurePlate {
    fn to_id(&self) -> i32 {
        if self.r#powered == false { return 20845; }
        if self.r#powered == true { return 20844; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20845 {
            return Some(CrimsonPressurePlate {
                r#powered: false,
            });
        }
        if state_id == 20844 {
            return Some(CrimsonPressurePlate {
                r#powered: true,
            });
        }
        return None;
    }
}

