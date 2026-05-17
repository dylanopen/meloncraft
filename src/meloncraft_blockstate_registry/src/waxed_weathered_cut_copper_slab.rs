use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCutCopperSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for WaxedWeatheredCutCopperSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Bottom && self.r#waterlogged == false { return 25806; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 25808; }
        if self.r#type == Type::Top && self.r#waterlogged == true { return 25803; }
        if self.r#waterlogged == true && self.r#type == Type::Double { return 25807; }
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 25805; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 25804; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25806 {
            return Some(WaxedWeatheredCutCopperSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25808 {
            return Some(WaxedWeatheredCutCopperSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 25803 {
            return Some(WaxedWeatheredCutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25807 {
            return Some(WaxedWeatheredCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 25805 {
            return Some(WaxedWeatheredCutCopperSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25804 {
            return Some(WaxedWeatheredCutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

