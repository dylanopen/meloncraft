use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedBlackstonePressurePlate {
    pub powered: bool,
}


impl BlockState for PolishedBlackstonePressurePlate {
    fn to_id(&self) -> i32 {
        if self.r#powered == false { return 22542; }
        if self.r#powered == true { return 22541; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22542 {
            return Some(PolishedBlackstonePressurePlate {
                r#powered: false,
            });
        }
        if state_id == 22541 {
            return Some(PolishedBlackstonePressurePlate {
                r#powered: true,
            });
        }
        return None;
    }
}

