use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetherBrickSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for NetherBrickSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 13248; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 13251; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 13252; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 13250; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 13253; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 13249; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13248 {
            return Some(NetherBrickSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13251 {
            return Some(NetherBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13252 {
            return Some(NetherBrickSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 13250 {
            return Some(NetherBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 13253 {
            return Some(NetherBrickSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13249 {
            return Some(NetherBrickSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        return None;
    }
}

