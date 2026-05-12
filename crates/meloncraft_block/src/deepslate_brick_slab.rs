use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateBrickSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for DeepslateBrickSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 29039; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 29042; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 29038; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 29043; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 29041; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 29040; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29039 {
            return Some(DeepslateBrickSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 29042 {
            return Some(DeepslateBrickSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 29038 {
            return Some(DeepslateBrickSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 29043 {
            return Some(DeepslateBrickSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 29041 {
            return Some(DeepslateBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 29040 {
            return Some(DeepslateBrickSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        return None;
    }
}

