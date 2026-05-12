use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for AcaciaSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 13154; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 13152; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 13156; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 13155; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 13153; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 13157; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13154 {
            return Some(AcaciaSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13152 {
            return Some(AcaciaSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13156 {
            return Some(AcaciaSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 13155 {
            return Some(AcaciaSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13153 {
            return Some(AcaciaSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 13157 {
            return Some(AcaciaSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

