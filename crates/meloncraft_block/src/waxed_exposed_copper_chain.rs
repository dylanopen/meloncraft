use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedCopperChain {
    pub r#axis: Axis,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for WaxedExposedCopperChain {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::Z && block_state.r#waterlogged == false { return 8086; }
        if block_state.r#axis == Axis::X && block_state.r#waterlogged == true { return 8081; }
        if block_state.r#waterlogged == false && block_state.r#axis == Axis::Y { return 8084; }
        if block_state.r#waterlogged == false && block_state.r#axis == Axis::X { return 8082; }
        if block_state.r#waterlogged == true && block_state.r#axis == Axis::Y { return 8083; }
        if block_state.r#waterlogged == true && block_state.r#axis == Axis::Z { return 8085; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8086 {
            return Some(WaxedExposedCopperChain {
                r#axis: Axis::Z,
                r#waterlogged: false,
            });
        }
        if state_id == 8081 {
            return Some(WaxedExposedCopperChain {
                r#axis: Axis::X,
                r#waterlogged: true,
            });
        }
        if state_id == 8084 {
            return Some(WaxedExposedCopperChain {
                r#waterlogged: false,
                r#axis: Axis::Y,
            });
        }
        if state_id == 8082 {
            return Some(WaxedExposedCopperChain {
                r#waterlogged: false,
                r#axis: Axis::X,
            });
        }
        if state_id == 8083 {
            return Some(WaxedExposedCopperChain {
                r#waterlogged: true,
                r#axis: Axis::Y,
            });
        }
        if state_id == 8085 {
            return Some(WaxedExposedCopperChain {
                r#waterlogged: true,
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

