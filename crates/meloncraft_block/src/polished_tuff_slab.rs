use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedTuffSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PolishedTuffSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 23662; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 23664; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 23667; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 23665; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 23663; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 23666; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23662 {
            return Some(PolishedTuffSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 23664 {
            return Some(PolishedTuffSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 23667 {
            return Some(PolishedTuffSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 23665 {
            return Some(PolishedTuffSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 23663 {
            return Some(PolishedTuffSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 23666 {
            return Some(PolishedTuffSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        return None;
    }
}

