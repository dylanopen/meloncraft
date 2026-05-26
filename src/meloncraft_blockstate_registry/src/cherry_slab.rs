use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherrySlab {
    pub waterlogged: bool,
    pub r#type: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for CherrySlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Top && self.r#waterlogged == true {
            return 13158;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == true {
            return 13160;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == false {
            return 13161;
        }
        if self.r#waterlogged == true && self.r#type == Type::Double {
            return 13162;
        }
        if self.r#waterlogged == false && self.r#type == Type::Double {
            return 13163;
        }
        if self.r#type == Type::Top && self.r#waterlogged == false {
            return 13159;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13158 {
            return Some(CherrySlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13160 {
            return Some(CherrySlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 13161 {
            return Some(CherrySlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13162 {
            return Some(CherrySlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 13163 {
            return Some(CherrySlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 13159 {
            return Some(CherrySlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
