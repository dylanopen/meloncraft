use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkPrismarineSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for DarkPrismarineSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 12688; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 12685; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 12684; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 12689; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 12687; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 12686; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12688 {
            return Some(DarkPrismarineSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 12685 {
            return Some(DarkPrismarineSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 12684 {
            return Some(DarkPrismarineSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 12689 {
            return Some(DarkPrismarineSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 12687 {
            return Some(DarkPrismarineSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 12686 {
            return Some(DarkPrismarineSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        return None;
    }
}

