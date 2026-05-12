use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraniteSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for GraniteSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Double && self.r#waterlogged == true { return 16266; }
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 16264; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 16263; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 16267; }
        if self.r#waterlogged == true && self.r#type == Type::Top { return 16262; }
        if self.r#type == Type::Bottom && self.r#waterlogged == false { return 16265; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16266 {
            return Some(GraniteSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 16264 {
            return Some(GraniteSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 16263 {
            return Some(GraniteSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 16267 {
            return Some(GraniteSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 16262 {
            return Some(GraniteSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 16265 {
            return Some(GraniteSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

