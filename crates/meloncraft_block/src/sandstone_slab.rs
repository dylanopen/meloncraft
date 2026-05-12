use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandstoneSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for SandstoneSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 13210; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 13208; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 13206; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 13209; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 13207; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 13211; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13210 {
            return Some(SandstoneSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 13208 {
            return Some(SandstoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 13206 {
            return Some(SandstoneSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13209 {
            return Some(SandstoneSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
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
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

