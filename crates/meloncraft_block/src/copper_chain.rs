use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperChain {
    pub r#axis: Axis,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for CopperChain {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::X && block_state.r#waterlogged == false { return 8052; }
        if block_state.r#axis == Axis::Y && block_state.r#waterlogged == false { return 8054; }
        if block_state.r#axis == Axis::Z && block_state.r#waterlogged == false { return 8056; }
        if block_state.r#axis == Axis::Y && block_state.r#waterlogged == true { return 8053; }
        if block_state.r#waterlogged == true && block_state.r#axis == Axis::Z { return 8055; }
        if block_state.r#waterlogged == true && block_state.r#axis == Axis::X { return 8051; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8052 {
            return Some(CopperChain {
                r#axis: Axis::X,
                r#waterlogged: false,
            });
        }
        if state_id == 8054 {
            return Some(CopperChain {
                r#axis: Axis::Y,
                r#waterlogged: false,
            });
        }
        if state_id == 8056 {
            return Some(CopperChain {
                r#axis: Axis::Z,
                r#waterlogged: false,
            });
        }
        if state_id == 8053 {
            return Some(CopperChain {
                r#axis: Axis::Y,
                r#waterlogged: true,
            });
        }
        if state_id == 8055 {
            return Some(CopperChain {
                r#waterlogged: true,
                r#axis: Axis::Z,
            });
        }
        if state_id == 8051 {
            return Some(CopperChain {
                r#waterlogged: true,
                r#axis: Axis::X,
            });
        }
        return None;
    }
}

