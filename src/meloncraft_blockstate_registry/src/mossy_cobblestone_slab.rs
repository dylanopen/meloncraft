use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MossyCobblestoneSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for MossyCobblestoneSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#type == Type::Top {
            return 16239;
        }
        if self.r#waterlogged == true && self.r#type == Type::Top {
            return 16238;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == true {
            return 16240;
        }
        if self.r#waterlogged == true && self.r#type == Type::Double {
            return 16242;
        }
        if self.r#waterlogged == false && self.r#type == Type::Double {
            return 16243;
        }
        if self.r#waterlogged == false && self.r#type == Type::Bottom {
            return 16241;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16239 {
            return Some(MossyCobblestoneSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 16238 {
            return Some(MossyCobblestoneSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 16240 {
            return Some(MossyCobblestoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 16242 {
            return Some(MossyCobblestoneSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 16243 {
            return Some(MossyCobblestoneSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 16241 {
            return Some(MossyCobblestoneSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        return None;
    }
}
