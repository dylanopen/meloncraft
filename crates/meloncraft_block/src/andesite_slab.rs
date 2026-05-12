use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AndesiteSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for AndesiteSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#type == Type::Bottom { return 16270; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 16272; }
        if self.r#type == Type::Top && self.r#waterlogged == true { return 16268; }
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 16271; }
        if self.r#waterlogged == false && self.r#type == Type::Double { return 16273; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 16269; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16270 {
            return Some(AndesiteSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 16272 {
            return Some(AndesiteSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 16268 {
            return Some(AndesiteSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 16271 {
            return Some(AndesiteSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 16273 {
            return Some(AndesiteSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 16269 {
            return Some(AndesiteSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

