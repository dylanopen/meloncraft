use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandstoneSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for SandstoneSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 13208; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 13207; }
        if self.r#waterlogged == false && self.r#type == Type::Double { return 13211; }
        if self.r#waterlogged == true && self.r#type == Type::Top { return 13206; }
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 13209; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 13210; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13208 {
            return Some(SandstoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 13207 {
            return Some(SandstoneSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13211 {
            return Some(SandstoneSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 13206 {
            return Some(SandstoneSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13209 {
            return Some(SandstoneSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13210 {
            return Some(SandstoneSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

