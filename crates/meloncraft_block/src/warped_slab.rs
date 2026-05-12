use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for WarpedSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 20838; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 20839; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 20843; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 20840; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 20841; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 20842; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20838 {
            return Some(WarpedSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 20839 {
            return Some(WarpedSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 20843 {
            return Some(WarpedSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 20840 {
            return Some(WarpedSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 20841 {
            return Some(WarpedSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 20842 {
            return Some(WarpedSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

