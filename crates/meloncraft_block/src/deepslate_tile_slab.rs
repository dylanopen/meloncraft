use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateTileSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for DeepslateTileSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Double && self.r#waterlogged == true { return 28631; }
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 28629; }
        if self.r#waterlogged == false && self.r#type == Type::Double { return 28632; }
        if self.r#type == Type::Top && self.r#waterlogged == true { return 28627; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 28628; }
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 28630; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 28631 {
            return Some(DeepslateTileSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 28629 {
            return Some(DeepslateTileSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 28632 {
            return Some(DeepslateTileSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 28627 {
            return Some(DeepslateTileSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 28628 {
            return Some(DeepslateTileSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 28630 {
            return Some(DeepslateTileSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        return None;
    }
}

