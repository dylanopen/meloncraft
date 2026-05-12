use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CobblestoneSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for CobblestoneSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 13227; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 13229; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 13224; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 13228; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 13225; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 13226; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13227 {
            return Some(CobblestoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13229 {
            return Some(CobblestoneSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13224 {
            return Some(CobblestoneSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13228 {
            return Some(CobblestoneSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 13225 {
            return Some(CobblestoneSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13226 {
            return Some(CobblestoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

