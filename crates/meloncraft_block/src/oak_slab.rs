use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for OakSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#type == Type::Top { return 13128; }
        if self.r#type == Type::Bottom && self.r#waterlogged == false { return 13131; }
        if self.r#waterlogged == false && self.r#type == Type::Top { return 13129; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 13132; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 13133; }
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 13130; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13128 {
            return Some(OakSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13131 {
            return Some(OakSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13129 {
            return Some(OakSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 13132 {
            return Some(OakSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 13133 {
            return Some(OakSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13130 {
            return Some(OakSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

