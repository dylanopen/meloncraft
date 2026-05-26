use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CobbledDeepslateSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for CobbledDeepslateSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Bottom && self.r#waterlogged == true {
            return 27807;
        }
        if self.r#waterlogged == true && self.r#type == Type::Double {
            return 27809;
        }
        if self.r#type == Type::Top && self.r#waterlogged == false {
            return 27806;
        }
        if self.r#waterlogged == false && self.r#type == Type::Bottom {
            return 27808;
        }
        if self.r#waterlogged == false && self.r#type == Type::Double {
            return 27810;
        }
        if self.r#waterlogged == true && self.r#type == Type::Top {
            return 27805;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27807 {
            return Some(CobbledDeepslateSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 27809 {
            return Some(CobbledDeepslateSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 27806 {
            return Some(CobbledDeepslateSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 27808 {
            return Some(CobbledDeepslateSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 27810 {
            return Some(CobbledDeepslateSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 27805 {
            return Some(CobbledDeepslateSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        return None;
    }
}
