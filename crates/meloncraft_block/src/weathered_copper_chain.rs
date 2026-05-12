use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCopperChain {
    pub waterlogged: bool,
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for WeatheredCopperChain {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::X && block_state.r#waterlogged == true { return 8063; }
        if block_state.r#waterlogged == false && block_state.r#axis == Axis::Y { return 8066; }
        if block_state.r#waterlogged == true && block_state.r#axis == Axis::Z { return 8067; }
        if block_state.r#axis == Axis::Z && block_state.r#waterlogged == false { return 8068; }
        if block_state.r#waterlogged == false && block_state.r#axis == Axis::X { return 8064; }
        if block_state.r#waterlogged == true && block_state.r#axis == Axis::Y { return 8065; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8063 {
            return Some(WeatheredCopperChain {
                r#axis: Axis::X,
                r#waterlogged: true,
            });
        }
        if state_id == 8066 {
            return Some(WeatheredCopperChain {
                r#waterlogged: false,
                r#axis: Axis::Y,
            });
        }
        if state_id == 8067 {
            return Some(WeatheredCopperChain {
                r#waterlogged: true,
                r#axis: Axis::Z,
            });
        }
        if state_id == 8068 {
            return Some(WeatheredCopperChain {
                r#axis: Axis::Z,
                r#waterlogged: false,
            });
        }
        if state_id == 8064 {
            return Some(WeatheredCopperChain {
                r#waterlogged: false,
                r#axis: Axis::X,
            });
        }
        if state_id == 8065 {
            return Some(WeatheredCopperChain {
                r#waterlogged: true,
                r#axis: Axis::Y,
            });
        }
        return None;
    }
}

