use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedBlackstoneSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PolishedBlackstoneSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Bottom && self.r#waterlogged == true {
            return 22537;
        }
        if self.r#waterlogged == false && self.r#type == Type::Bottom {
            return 22538;
        }
        if self.r#type == Type::Double && self.r#waterlogged == true {
            return 22539;
        }
        if self.r#waterlogged == false && self.r#type == Type::Double {
            return 22540;
        }
        if self.r#type == Type::Top && self.r#waterlogged == false {
            return 22536;
        }
        if self.r#type == Type::Top && self.r#waterlogged == true {
            return 22535;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22537 {
            return Some(PolishedBlackstoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 22538 {
            return Some(PolishedBlackstoneSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 22539 {
            return Some(PolishedBlackstoneSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 22540 {
            return Some(PolishedBlackstoneSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 22536 {
            return Some(PolishedBlackstoneSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 22535 {
            return Some(PolishedBlackstoneSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
