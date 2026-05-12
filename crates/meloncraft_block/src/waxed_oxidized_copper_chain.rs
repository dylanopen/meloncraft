use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCopperChain {
    pub waterlogged: bool,
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for WaxedOxidizedCopperChain {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::Z && block_state.r#waterlogged == false { return 8098; }
        if block_state.r#waterlogged == true && block_state.r#axis == Axis::Y { return 8095; }
        if block_state.r#axis == Axis::Y && block_state.r#waterlogged == false { return 8096; }
        if block_state.r#axis == Axis::Z && block_state.r#waterlogged == true { return 8097; }
        if block_state.r#axis == Axis::X && block_state.r#waterlogged == false { return 8094; }
        if block_state.r#waterlogged == true && block_state.r#axis == Axis::X { return 8093; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8098 {
            return Some(WaxedOxidizedCopperChain {
                r#axis: Axis::Z,
                r#waterlogged: false,
            });
        }
        if state_id == 8095 {
            return Some(WaxedOxidizedCopperChain {
                r#waterlogged: true,
                r#axis: Axis::Y,
            });
        }
        if state_id == 8096 {
            return Some(WaxedOxidizedCopperChain {
                r#axis: Axis::Y,
                r#waterlogged: false,
            });
        }
        if state_id == 8097 {
            return Some(WaxedOxidizedCopperChain {
                r#axis: Axis::Z,
                r#waterlogged: true,
            });
        }
        if state_id == 8094 {
            return Some(WaxedOxidizedCopperChain {
                r#axis: Axis::X,
                r#waterlogged: false,
            });
        }
        if state_id == 8093 {
            return Some(WaxedOxidizedCopperChain {
                r#waterlogged: true,
                r#axis: Axis::X,
            });
        }
        return None;
    }
}

