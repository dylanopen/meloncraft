use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrismarineSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PrismarineSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 12676; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 12677; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 12672; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 12673; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 12674; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 12675; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12676 {
            return Some(PrismarineSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 12677 {
            return Some(PrismarineSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 12672 {
            return Some(PrismarineSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 12673 {
            return Some(PrismarineSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 12674 {
            return Some(PrismarineSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 12675 {
            return Some(PrismarineSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

