use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MudBrickSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for MudBrickSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 13245; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 13247; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 13244; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 13246; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 13242; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 13243; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13245 {
            return Some(MudBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13247 {
            return Some(MudBrickSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13244 {
            return Some(MudBrickSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13246 {
            return Some(MudBrickSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 13242 {
            return Some(MudBrickSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13243 {
            return Some(MudBrickSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

