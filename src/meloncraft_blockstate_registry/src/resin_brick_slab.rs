use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResinBrickSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for ResinBrickSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Double && self.r#waterlogged == true {
            return 8806;
        }
        if self.r#type == Type::Top && self.r#waterlogged == false {
            return 8803;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == false {
            return 8805;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == true {
            return 8804;
        }
        if self.r#waterlogged == true && self.r#type == Type::Top {
            return 8802;
        }
        if self.r#type == Type::Double && self.r#waterlogged == false {
            return 8807;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8806 {
            return Some(ResinBrickSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 8803 {
            return Some(ResinBrickSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 8805 {
            return Some(ResinBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 8804 {
            return Some(ResinBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 8802 {
            return Some(ResinBrickSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 8807 {
            return Some(ResinBrickSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
