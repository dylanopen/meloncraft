use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for BirchSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 13145; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 13141; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 13140; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 13143; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 13142; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 13144; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13145 {
            return Some(BirchSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13141 {
            return Some(BirchSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13140 {
            return Some(BirchSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13143 {
            return Some(BirchSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13142 {
            return Some(BirchSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13144 {
            return Some(BirchSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

