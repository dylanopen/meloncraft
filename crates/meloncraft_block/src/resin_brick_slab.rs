use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResinBrickSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for ResinBrickSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 8804; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 8803; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 8805; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 8806; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 8807; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 8802; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8804 {
            return Some(ResinBrickSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 8803 {
            return Some(ResinBrickSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 8805 {
            return Some(ResinBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 8806 {
            return Some(ResinBrickSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 8807 {
            return Some(ResinBrickSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 8802 {
            return Some(ResinBrickSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        return None;
    }
}

