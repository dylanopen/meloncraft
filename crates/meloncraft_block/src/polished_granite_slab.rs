use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedGraniteSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PolishedGraniteSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 16217; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 16216; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 16214; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 16218; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 16219; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 16215; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16217 {
            return Some(PolishedGraniteSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 16216 {
            return Some(PolishedGraniteSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 16214 {
            return Some(PolishedGraniteSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 16218 {
            return Some(PolishedGraniteSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 16219 {
            return Some(PolishedGraniteSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 16215 {
            return Some(PolishedGraniteSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        return None;
    }
}

