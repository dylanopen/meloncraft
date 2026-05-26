use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for WarpedSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#type == Type::Double {
            return 20843;
        }
        if self.r#type == Type::Top && self.r#waterlogged == false {
            return 20839;
        }
        if self.r#waterlogged == true && self.r#type == Type::Bottom {
            return 20840;
        }
        if self.r#waterlogged == false && self.r#type == Type::Bottom {
            return 20841;
        }
        if self.r#waterlogged == true && self.r#type == Type::Double {
            return 20842;
        }
        if self.r#type == Type::Top && self.r#waterlogged == true {
            return 20838;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20843 {
            return Some(WarpedSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 20839 {
            return Some(WarpedSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 20840 {
            return Some(WarpedSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 20841 {
            return Some(WarpedSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 20842 {
            return Some(WarpedSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 20838 {
            return Some(WarpedSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
