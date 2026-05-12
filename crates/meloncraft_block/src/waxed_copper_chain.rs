use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCopperChain {
    pub waterlogged: bool,
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for WaxedCopperChain {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#axis == Axis::X { return 8076; }
        if block_state.r#axis == Axis::Y && block_state.r#waterlogged == true { return 8077; }
        if block_state.r#waterlogged == true && block_state.r#axis == Axis::Z { return 8079; }
        if block_state.r#axis == Axis::X && block_state.r#waterlogged == true { return 8075; }
        if block_state.r#axis == Axis::Y && block_state.r#waterlogged == false { return 8078; }
        if block_state.r#axis == Axis::Z && block_state.r#waterlogged == false { return 8080; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8076 {
            return Some(WaxedCopperChain {
                r#waterlogged: false,
                r#axis: Axis::X,
            });
        }
        if state_id == 8077 {
            return Some(WaxedCopperChain {
                r#axis: Axis::Y,
                r#waterlogged: true,
            });
        }
        if state_id == 8079 {
            return Some(WaxedCopperChain {
                r#waterlogged: true,
                r#axis: Axis::Z,
            });
        }
        if state_id == 8075 {
            return Some(WaxedCopperChain {
                r#axis: Axis::X,
                r#waterlogged: true,
            });
        }
        if state_id == 8078 {
            return Some(WaxedCopperChain {
                r#axis: Axis::Y,
                r#waterlogged: false,
            });
        }
        if state_id == 8080 {
            return Some(WaxedCopperChain {
                r#axis: Axis::Z,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

