use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PaleOakSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Bottom && self.r#waterlogged == false { return 13173; }
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 13172; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 13171; }
        if self.r#waterlogged == true && self.r#type == Type::Double { return 13174; }
        if self.r#waterlogged == false && self.r#type == Type::Double { return 13175; }
        if self.r#type == Type::Top && self.r#waterlogged == true { return 13170; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13173 {
            return Some(PaleOakSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13172 {
            return Some(PaleOakSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 13171 {
            return Some(PaleOakSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13174 {
            return Some(PaleOakSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 13175 {
            return Some(PaleOakSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 13170 {
            return Some(PaleOakSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

