use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoneSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for StoneSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#type == Type::Top {
            return 13194;
        }
        if self.r#waterlogged == false && self.r#type == Type::Bottom {
            return 13197;
        }
        if self.r#type == Type::Top && self.r#waterlogged == false {
            return 13195;
        }
        if self.r#waterlogged == true && self.r#type == Type::Bottom {
            return 13196;
        }
        if self.r#waterlogged == true && self.r#type == Type::Double {
            return 13198;
        }
        if self.r#type == Type::Double && self.r#waterlogged == false {
            return 13199;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13194 {
            return Some(StoneSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13197 {
            return Some(StoneSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13195 {
            return Some(StoneSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13196 {
            return Some(StoneSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13198 {
            return Some(StoneSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 13199 {
            return Some(StoneSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
