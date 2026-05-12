use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IronChain {
    pub waterlogged: bool,
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for IronChain {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::X && block_state.r#waterlogged == false { return 8046; }
        if block_state.r#waterlogged == false && block_state.r#axis == Axis::Y { return 8048; }
        if block_state.r#axis == Axis::Z && block_state.r#waterlogged == true { return 8049; }
        if block_state.r#waterlogged == true && block_state.r#axis == Axis::X { return 8045; }
        if block_state.r#waterlogged == false && block_state.r#axis == Axis::Z { return 8050; }
        if block_state.r#waterlogged == true && block_state.r#axis == Axis::Y { return 8047; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8046 {
            return Some(IronChain {
                r#axis: Axis::X,
                r#waterlogged: false,
            });
        }
        if state_id == 8048 {
            return Some(IronChain {
                r#waterlogged: false,
                r#axis: Axis::Y,
            });
        }
        if state_id == 8049 {
            return Some(IronChain {
                r#axis: Axis::Z,
                r#waterlogged: true,
            });
        }
        if state_id == 8045 {
            return Some(IronChain {
                r#waterlogged: true,
                r#axis: Axis::X,
            });
        }
        if state_id == 8050 {
            return Some(IronChain {
                r#waterlogged: false,
                r#axis: Axis::Z,
            });
        }
        if state_id == 8047 {
            return Some(IronChain {
                r#waterlogged: true,
                r#axis: Axis::Y,
            });
        }
        return None;
    }
}

