use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CobblestoneSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for CobblestoneSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 13227; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 13228; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 13229; }
        if self.r#waterlogged == true && self.r#type == Type::Bottom { return 13226; }
        if self.r#waterlogged == true && self.r#type == Type::Top { return 13224; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 13225; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13227 {
            return Some(CobblestoneSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13228 {
            return Some(CobblestoneSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 13229 {
            return Some(CobblestoneSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13226 {
            return Some(CobblestoneSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13224 {
            return Some(CobblestoneSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13225 {
            return Some(CobblestoneSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

