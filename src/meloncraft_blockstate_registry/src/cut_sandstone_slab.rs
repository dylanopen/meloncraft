use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CutSandstoneSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for CutSandstoneSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Top && self.r#waterlogged == false {
            return 13213;
        }
        if self.r#waterlogged == true && self.r#type == Type::Top {
            return 13212;
        }
        if self.r#waterlogged == false && self.r#type == Type::Bottom {
            return 13215;
        }
        if self.r#waterlogged == false && self.r#type == Type::Double {
            return 13217;
        }
        if self.r#waterlogged == true && self.r#type == Type::Bottom {
            return 13214;
        }
        if self.r#type == Type::Double && self.r#waterlogged == true {
            return 13216;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13213 {
            return Some(CutSandstoneSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13212 {
            return Some(CutSandstoneSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13215 {
            return Some(CutSandstoneSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13217 {
            return Some(CutSandstoneSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 13214 {
            return Some(CutSandstoneSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13216 {
            return Some(CutSandstoneSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
