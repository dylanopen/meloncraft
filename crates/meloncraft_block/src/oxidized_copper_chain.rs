use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCopperChain {
    pub waterlogged: bool,
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for OxidizedCopperChain {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::X && block_state.r#waterlogged == true { return 8069; }
        if block_state.r#axis == Axis::Z && block_state.r#waterlogged == false { return 8074; }
        if block_state.r#waterlogged == true && block_state.r#axis == Axis::Y { return 8071; }
        if block_state.r#axis == Axis::X && block_state.r#waterlogged == false { return 8070; }
        if block_state.r#axis == Axis::Y && block_state.r#waterlogged == false { return 8072; }
        if block_state.r#waterlogged == true && block_state.r#axis == Axis::Z { return 8073; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8069 {
            return Some(OxidizedCopperChain {
                r#axis: Axis::X,
                r#waterlogged: true,
            });
        }
        if state_id == 8074 {
            return Some(OxidizedCopperChain {
                r#axis: Axis::Z,
                r#waterlogged: false,
            });
        }
        if state_id == 8071 {
            return Some(OxidizedCopperChain {
                r#waterlogged: true,
                r#axis: Axis::Y,
            });
        }
        if state_id == 8070 {
            return Some(OxidizedCopperChain {
                r#axis: Axis::X,
                r#waterlogged: false,
            });
        }
        if state_id == 8072 {
            return Some(OxidizedCopperChain {
                r#axis: Axis::Y,
                r#waterlogged: false,
            });
        }
        if state_id == 8073 {
            return Some(OxidizedCopperChain {
                r#waterlogged: true,
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

