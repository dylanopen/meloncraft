use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for AcaciaSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Top && self.r#waterlogged == false { return 13153; }
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 13155; }
        if self.r#type == Type::Top && self.r#waterlogged == true { return 13152; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 13157; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 13156; }
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 13154; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13153 {
            return Some(AcaciaSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13155 {
            return Some(AcaciaSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13152 {
            return Some(AcaciaSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13157 {
            return Some(AcaciaSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13156 {
            return Some(AcaciaSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 13154 {
            return Some(AcaciaSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

