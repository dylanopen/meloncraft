use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DioriteSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for DioriteSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#type == Type::Double { return 16291; }
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 16288; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 16287; }
        if self.r#type == Type::Top && self.r#waterlogged == true { return 16286; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 16290; }
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 16289; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16291 {
            return Some(DioriteSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 16288 {
            return Some(DioriteSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 16287 {
            return Some(DioriteSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 16286 {
            return Some(DioriteSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 16290 {
            return Some(DioriteSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 16289 {
            return Some(DioriteSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        return None;
    }
}

