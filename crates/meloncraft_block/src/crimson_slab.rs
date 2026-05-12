use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for CrimsonSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 20832; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 20833; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 20834; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 20835; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 20837; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 20836; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20832 {
            return Some(CrimsonSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 20833 {
            return Some(CrimsonSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 20834 {
            return Some(CrimsonSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 20835 {
            return Some(CrimsonSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 20837 {
            return Some(CrimsonSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 20836 {
            return Some(CrimsonSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

