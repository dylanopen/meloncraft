use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for MangroveSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 13181; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 13176; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 13179; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 13178; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 13177; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 13180; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13181 {
            return Some(MangroveSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 13176 {
            return Some(MangroveSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13179 {
            return Some(MangroveSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13178 {
            return Some(MangroveSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13177 {
            return Some(MangroveSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13180 {
            return Some(MangroveSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

