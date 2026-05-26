use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCopperLantern {
    pub hanging: bool,
    pub waterlogged: bool,
}

impl BlockState for WeatheredCopperLantern {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#hanging == false {
            return 20654;
        }
        if self.r#hanging == true && self.r#waterlogged == true {
            return 20651;
        }
        if self.r#waterlogged == true && self.r#hanging == false {
            return 20653;
        }
        if self.r#waterlogged == false && self.r#hanging == true {
            return 20652;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20654 {
            return Some(WeatheredCopperLantern {
                r#waterlogged: false,
                r#hanging: false,
            });
        }
        if state_id == 20651 {
            return Some(WeatheredCopperLantern {
                r#hanging: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20653 {
            return Some(WeatheredCopperLantern {
                r#waterlogged: true,
                r#hanging: false,
            });
        }
        if state_id == 20652 {
            return Some(WeatheredCopperLantern {
                r#waterlogged: false,
                r#hanging: true,
            });
        }
        return None;
    }
}
