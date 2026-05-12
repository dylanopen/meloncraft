use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooMosaicSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for BambooMosaicSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 13190; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 13193; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 13188; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 13189; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 13191; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 13192; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13190 {
            return Some(BambooMosaicSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13193 {
            return Some(BambooMosaicSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13188 {
            return Some(BambooMosaicSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13189 {
            return Some(BambooMosaicSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 13191 {
            return Some(BambooMosaicSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13192 {
            return Some(BambooMosaicSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

