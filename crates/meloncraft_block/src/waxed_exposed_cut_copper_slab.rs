use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedCutCopperSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for WaxedExposedCutCopperSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 25812; }
        if self.r#waterlogged == true && self.r#type == Type::Top { return 25809; }
        if self.r#waterlogged == false && self.r#type == Type::Double { return 25814; }
        if self.r#waterlogged == true && self.r#type == Type::Double { return 25813; }
        if self.r#waterlogged == true && self.r#type == Type::Bottom { return 25811; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 25810; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25812 {
            return Some(WaxedExposedCutCopperSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 25809 {
            return Some(WaxedExposedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 25814 {
            return Some(WaxedExposedCutCopperSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 25813 {
            return Some(WaxedExposedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 25811 {
            return Some(WaxedExposedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 25810 {
            return Some(WaxedExposedCutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

