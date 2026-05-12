use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for CrimsonSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Bottom && self.r#waterlogged == false { return 20835; }
        if self.r#waterlogged == true && self.r#type == Type::Top { return 20832; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 20833; }
        if self.r#waterlogged == true && self.r#type == Type::Bottom { return 20834; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 20836; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 20837; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20835 {
            return Some(CrimsonSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 20832 {
            return Some(CrimsonSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 20833 {
            return Some(CrimsonSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 20834 {
            return Some(CrimsonSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 20836 {
            return Some(CrimsonSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 20837 {
            return Some(CrimsonSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

