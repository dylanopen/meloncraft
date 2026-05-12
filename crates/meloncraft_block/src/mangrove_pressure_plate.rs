use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangrovePressurePlate {
    pub powered: bool,
}


impl BlockState for MangrovePressurePlate {
    fn to_id(&self) -> i32 {
        if self.r#powered == false { return 6677; }
        if self.r#powered == true { return 6676; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6677 {
            return Some(MangrovePressurePlate {
                r#powered: false,
            });
        }
        if state_id == 6676 {
            return Some(MangrovePressurePlate {
                r#powered: true,
            });
        }
        return None;
    }
}

