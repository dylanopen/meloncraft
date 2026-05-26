use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrickSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for BrickSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Top && self.r#waterlogged == false {
            return 13231;
        }
        if self.r#waterlogged == false && self.r#type == Type::Bottom {
            return 13233;
        }
        if self.r#type == Type::Top && self.r#waterlogged == true {
            return 13230;
        }
        if self.r#type == Type::Double && self.r#waterlogged == true {
            return 13234;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == true {
            return 13232;
        }
        if self.r#waterlogged == false && self.r#type == Type::Double {
            return 13235;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13231 {
            return Some(BrickSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13233 {
            return Some(BrickSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13230 {
            return Some(BrickSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13234 {
            return Some(BrickSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 13232 {
            return Some(BrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 13235 {
            return Some(BrickSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        return None;
    }
}
