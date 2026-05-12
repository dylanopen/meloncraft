use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MudBrickSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for MudBrickSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#type == Type::Top { return 13242; }
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 13245; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 13243; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 13246; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 13247; }
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 13244; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13242 {
            return Some(MudBrickSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13245 {
            return Some(MudBrickSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13243 {
            return Some(MudBrickSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13246 {
            return Some(MudBrickSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 13247 {
            return Some(MudBrickSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13244 {
            return Some(MudBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

