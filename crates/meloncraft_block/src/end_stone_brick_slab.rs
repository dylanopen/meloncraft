use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndStoneBrickSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for EndStoneBrickSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 16244; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 16247; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 16248; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 16249; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 16245; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 16246; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16244 {
            return Some(EndStoneBrickSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 16247 {
            return Some(EndStoneBrickSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 16248 {
            return Some(EndStoneBrickSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 16249 {
            return Some(EndStoneBrickSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 16245 {
            return Some(EndStoneBrickSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 16246 {
            return Some(EndStoneBrickSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        return None;
    }
}

