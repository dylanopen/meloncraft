use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCutCopperSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for WaxedCutCopperSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Double && self.r#waterlogged == true { return 25819; }
        if self.r#waterlogged == false && self.r#type == Type::Double { return 25820; }
        if self.r#type == Type::Top && self.r#waterlogged == true { return 25815; }
        if self.r#type == Type::Bottom && self.r#waterlogged == false { return 25818; }
        if self.r#waterlogged == true && self.r#type == Type::Bottom { return 25817; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 25816; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25819 {
            return Some(WaxedCutCopperSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 25820 {
            return Some(WaxedCutCopperSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 25815 {
            return Some(WaxedCutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25818 {
            return Some(WaxedCutCopperSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25817 {
            return Some(WaxedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 25816 {
            return Some(WaxedCutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

