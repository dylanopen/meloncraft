use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedAndesiteSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PolishedAndesiteSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#type == Type::Top {
            return 16281;
        }
        if self.r#waterlogged == true && self.r#type == Type::Bottom {
            return 16282;
        }
        if self.r#waterlogged == false && self.r#type == Type::Bottom {
            return 16283;
        }
        if self.r#type == Type::Double && self.r#waterlogged == false {
            return 16285;
        }
        if self.r#waterlogged == true && self.r#type == Type::Double {
            return 16284;
        }
        if self.r#waterlogged == true && self.r#type == Type::Top {
            return 16280;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16281 {
            return Some(PolishedAndesiteSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 16282 {
            return Some(PolishedAndesiteSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 16283 {
            return Some(PolishedAndesiteSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 16285 {
            return Some(PolishedAndesiteSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 16284 {
            return Some(PolishedAndesiteSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 16280 {
            return Some(PolishedAndesiteSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        return None;
    }
}
