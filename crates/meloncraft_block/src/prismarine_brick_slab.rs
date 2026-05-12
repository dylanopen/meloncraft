use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrismarineBrickSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PrismarineBrickSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 12683; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 12678; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 12679; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 12680; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 12682; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 12681; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12683 {
            return Some(PrismarineBrickSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 12678 {
            return Some(PrismarineBrickSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 12679 {
            return Some(PrismarineBrickSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 12680 {
            return Some(PrismarineBrickSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 12682 {
            return Some(PrismarineBrickSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 12681 {
            return Some(PrismarineBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

