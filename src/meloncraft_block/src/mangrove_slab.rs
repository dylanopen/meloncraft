use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for MangroveSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 13179; }
        if self.r#waterlogged == true && self.r#type == Type::Bottom { return 13178; }
        if self.r#waterlogged == true && self.r#type == Type::Top { return 13176; }
        if self.r#waterlogged == false && self.r#type == Type::Double { return 13181; }
        if self.r#waterlogged == false && self.r#type == Type::Top { return 13177; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 13180; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13179 {
            return Some(MangroveSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13178 {
            return Some(MangroveSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13176 {
            return Some(MangroveSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13181 {
            return Some(MangroveSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 13177 {
            return Some(MangroveSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 13180 {
            return Some(MangroveSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

