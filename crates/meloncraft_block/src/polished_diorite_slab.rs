use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedDioriteSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PolishedDioriteSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 16236; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 16233; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 16234; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 16237; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 16232; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 16235; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16236 {
            return Some(PolishedDioriteSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 16233 {
            return Some(PolishedDioriteSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 16234 {
            return Some(PolishedDioriteSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 16237 {
            return Some(PolishedDioriteSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 16232 {
            return Some(PolishedDioriteSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 16235 {
            return Some(PolishedDioriteSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        return None;
    }
}

