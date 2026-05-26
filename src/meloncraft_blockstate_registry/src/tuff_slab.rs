use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuffSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for TuffSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Top && self.r#waterlogged == true {
            return 23251;
        }
        if self.r#type == Type::Double && self.r#waterlogged == true {
            return 23255;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == true {
            return 23253;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == false {
            return 23254;
        }
        if self.r#type == Type::Double && self.r#waterlogged == false {
            return 23256;
        }
        if self.r#waterlogged == false && self.r#type == Type::Top {
            return 23252;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23251 {
            return Some(TuffSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 23255 {
            return Some(TuffSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 23253 {
            return Some(TuffSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 23254 {
            return Some(TuffSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 23256 {
            return Some(TuffSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 23252 {
            return Some(TuffSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        return None;
    }
}
