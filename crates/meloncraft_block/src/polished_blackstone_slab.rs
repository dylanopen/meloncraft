use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedBlackstoneSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PolishedBlackstoneSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 22540; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 22538; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 22535; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 22536; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 22537; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 22539; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22540 {
            return Some(PolishedBlackstoneSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 22538 {
            return Some(PolishedBlackstoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 22535 {
            return Some(PolishedBlackstoneSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 22536 {
            return Some(PolishedBlackstoneSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 22537 {
            return Some(PolishedBlackstoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 22539 {
            return Some(PolishedBlackstoneSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

