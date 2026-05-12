use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooMosaicSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for BambooMosaicSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Top && self.r#waterlogged == true { return 13188; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 13192; }
        if self.r#waterlogged == false && self.r#type == Type::Double { return 13193; }
        if self.r#waterlogged == true && self.r#type == Type::Bottom { return 13190; }
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 13191; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 13189; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13188 {
            return Some(BambooMosaicSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13192 {
            return Some(BambooMosaicSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 13193 {
            return Some(BambooMosaicSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 13190 {
            return Some(BambooMosaicSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13191 {
            return Some(BambooMosaicSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13189 {
            return Some(BambooMosaicSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

