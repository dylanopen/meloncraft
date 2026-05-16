use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedDioriteSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PolishedDioriteSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Bottom && self.r#waterlogged == false { return 16235; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 16237; }
        if self.r#waterlogged == true && self.r#type == Type::Top { return 16232; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 16236; }
        if self.r#waterlogged == true && self.r#type == Type::Bottom { return 16234; }
        if self.r#waterlogged == false && self.r#type == Type::Top { return 16233; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16235 {
            return Some(PolishedDioriteSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 16237 {
            return Some(PolishedDioriteSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 16232 {
            return Some(PolishedDioriteSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 16236 {
            return Some(PolishedDioriteSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 16234 {
            return Some(PolishedDioriteSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 16233 {
            return Some(PolishedDioriteSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        return None;
    }
}

