use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedDeepslateSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PolishedDeepslateSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#type == Type::Bottom {
            return 28218;
        }
        if self.r#waterlogged == true && self.r#type == Type::Top {
            return 28216;
        }
        if self.r#type == Type::Double && self.r#waterlogged == false {
            return 28221;
        }
        if self.r#waterlogged == false && self.r#type == Type::Bottom {
            return 28219;
        }
        if self.r#waterlogged == true && self.r#type == Type::Double {
            return 28220;
        }
        if self.r#type == Type::Top && self.r#waterlogged == false {
            return 28217;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 28218 {
            return Some(PolishedDeepslateSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 28216 {
            return Some(PolishedDeepslateSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 28221 {
            return Some(PolishedDeepslateSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 28219 {
            return Some(PolishedDeepslateSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 28220 {
            return Some(PolishedDeepslateSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 28217 {
            return Some(PolishedDeepslateSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
