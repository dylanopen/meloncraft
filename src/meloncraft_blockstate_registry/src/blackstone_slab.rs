use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackstoneSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for BlackstoneSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#type == Type::Double {
            return 22038;
        }
        if self.r#waterlogged == false && self.r#type == Type::Bottom {
            return 22037;
        }
        if self.r#type == Type::Top && self.r#waterlogged == false {
            return 22035;
        }
        if self.r#type == Type::Double && self.r#waterlogged == false {
            return 22039;
        }
        if self.r#waterlogged == true && self.r#type == Type::Bottom {
            return 22036;
        }
        if self.r#waterlogged == true && self.r#type == Type::Top {
            return 22034;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22038 {
            return Some(BlackstoneSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 22037 {
            return Some(BlackstoneSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 22035 {
            return Some(BlackstoneSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 22039 {
            return Some(BlackstoneSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 22036 {
            return Some(BlackstoneSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 22034 {
            return Some(BlackstoneSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        return None;
    }
}
