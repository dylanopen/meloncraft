use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCutCopperSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for ExposedCutCopperSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Double && self.r#waterlogged == false {
            return 25462;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == false {
            return 25460;
        }
        if self.r#waterlogged == false && self.r#type == Type::Top {
            return 25458;
        }
        if self.r#waterlogged == true && self.r#type == Type::Top {
            return 25457;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == true {
            return 25459;
        }
        if self.r#type == Type::Double && self.r#waterlogged == true {
            return 25461;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25462 {
            return Some(ExposedCutCopperSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 25460 {
            return Some(ExposedCutCopperSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25458 {
            return Some(ExposedCutCopperSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 25457 {
            return Some(ExposedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 25459 {
            return Some(ExposedCutCopperSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25461 {
            return Some(ExposedCutCopperSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
