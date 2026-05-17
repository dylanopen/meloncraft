use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrismarineSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PrismarineSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#type == Type::Top { return 12673; }
        if self.r#waterlogged == true && self.r#type == Type::Bottom { return 12674; }
        if self.r#type == Type::Top && self.r#waterlogged == true { return 12672; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 12676; }
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 12675; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 12677; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12673 {
            return Some(PrismarineSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 12674 {
            return Some(PrismarineSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 12672 {
            return Some(PrismarineSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 12676 {
            return Some(PrismarineSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 12675 {
            return Some(PrismarineSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 12677 {
            return Some(PrismarineSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

