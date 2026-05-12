use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for JungleSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 13146; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 13150; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 13151; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 13149; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 13147; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 13148; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13146 {
            return Some(JungleSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13150 {
            return Some(JungleSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 13151 {
            return Some(JungleSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 13149 {
            return Some(JungleSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13147 {
            return Some(JungleSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13148 {
            return Some(JungleSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

