use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmoothSandstoneSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for SmoothSandstoneSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Double && self.r#waterlogged == false { return 16255; }
        if self.r#waterlogged == false && self.r#type == Type::Top { return 16251; }
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 16252; }
        if self.r#waterlogged == true && self.r#type == Type::Double { return 16254; }
        if self.r#type == Type::Bottom && self.r#waterlogged == false { return 16253; }
        if self.r#type == Type::Top && self.r#waterlogged == true { return 16250; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16255 {
            return Some(SmoothSandstoneSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 16251 {
            return Some(SmoothSandstoneSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 16252 {
            return Some(SmoothSandstoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 16254 {
            return Some(SmoothSandstoneSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 16253 {
            return Some(SmoothSandstoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 16250 {
            return Some(SmoothSandstoneSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

