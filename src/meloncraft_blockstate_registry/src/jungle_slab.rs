use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for JungleSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Double && self.r#waterlogged == false {
            return 13151;
        }
        if self.r#type == Type::Top && self.r#waterlogged == true {
            return 13146;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == true {
            return 13148;
        }
        if self.r#type == Type::Top && self.r#waterlogged == false {
            return 13147;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == false {
            return 13149;
        }
        if self.r#type == Type::Double && self.r#waterlogged == true {
            return 13150;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13151 {
            return Some(JungleSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13146 {
            return Some(JungleSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13148 {
            return Some(JungleSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 13147 {
            return Some(JungleSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13149 {
            return Some(JungleSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13150 {
            return Some(JungleSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
