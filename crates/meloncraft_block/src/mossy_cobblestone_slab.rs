use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MossyCobblestoneSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for MossyCobblestoneSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 16238; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 16240; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 16239; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 16241; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 16243; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 16242; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
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
        if state_id == 16239 {
            return Some(MossyCobblestoneSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 16241 {
            return Some(MossyCobblestoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 16243 {
            return Some(MossyCobblestoneSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 16242 {
            return Some(MossyCobblestoneSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

