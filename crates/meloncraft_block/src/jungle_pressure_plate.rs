use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JunglePressurePlate {
    pub powered: bool,
}


impl BlockState for JunglePressurePlate {
    fn to_id(&self) -> i32 {
        if self.r#powered == true { return 6666; }
        if self.r#powered == false { return 6667; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6666 {
            return Some(JunglePressurePlate {
                r#powered: true,
            });
        }
        if state_id == 6667 {
            return Some(JunglePressurePlate {
                r#powered: false,
            });
        }
        return None;
    }
}

