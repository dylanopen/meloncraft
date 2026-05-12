use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCopperChain {
    pub r#axis: Axis,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for WaxedWeatheredCopperChain {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::Z && block_state.r#waterlogged == false { return 8092; }
        if block_state.r#axis == Axis::X && block_state.r#waterlogged == false { return 8088; }
        if block_state.r#axis == Axis::Y && block_state.r#waterlogged == false { return 8090; }
        if block_state.r#axis == Axis::Y && block_state.r#waterlogged == true { return 8089; }
        if block_state.r#waterlogged == true && block_state.r#axis == Axis::X { return 8087; }
        if block_state.r#axis == Axis::Z && block_state.r#waterlogged == true { return 8091; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8092 {
            return Some(WaxedWeatheredCopperChain {
                r#axis: Axis::Z,
                r#waterlogged: false,
            });
        }
        if state_id == 8088 {
            return Some(WaxedWeatheredCopperChain {
                r#axis: Axis::X,
                r#waterlogged: false,
            });
        }
        if state_id == 8090 {
            return Some(WaxedWeatheredCopperChain {
                r#axis: Axis::Y,
                r#waterlogged: false,
            });
        }
        if state_id == 8089 {
            return Some(WaxedWeatheredCopperChain {
                r#axis: Axis::Y,
                r#waterlogged: true,
            });
        }
        if state_id == 8087 {
            return Some(WaxedWeatheredCopperChain {
                r#waterlogged: true,
                r#axis: Axis::X,
            });
        }
        if state_id == 8091 {
            return Some(WaxedWeatheredCopperChain {
                r#axis: Axis::Z,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

