use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCopperLantern {
    pub hanging: bool,
    pub waterlogged: bool,
}


impl BlockState for ExposedCopperLantern {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#hanging == false { return 20650; }
        if self.r#hanging == false && self.r#waterlogged == true { return 20649; }
        if self.r#hanging == true && self.r#waterlogged == true { return 20647; }
        if self.r#hanging == true && self.r#waterlogged == false { return 20648; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20650 {
            return Some(ExposedCopperLantern {
                r#waterlogged: false,
                r#hanging: false,
            });
        }
        if state_id == 20649 {
            return Some(ExposedCopperLantern {
                r#hanging: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20647 {
            return Some(ExposedCopperLantern {
                r#hanging: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20648 {
            return Some(ExposedCopperLantern {
                r#hanging: true,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

