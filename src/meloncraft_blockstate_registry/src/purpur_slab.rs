use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpurSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PurpurSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#type == Type::Top { return 13272; }
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 13275; }
        if self.r#waterlogged == true && self.r#type == Type::Bottom { return 13274; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 13277; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 13273; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 13276; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13272 {
            return Some(PurpurSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13275 {
            return Some(PurpurSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13274 {
            return Some(PurpurSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13277 {
            return Some(PurpurSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13273 {
            return Some(PurpurSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13276 {
            return Some(PurpurSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

