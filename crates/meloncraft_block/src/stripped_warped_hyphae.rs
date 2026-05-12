use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedWarpedHyphae {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedWarpedHyphae {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::Y { return 20753; }
        if block_state.r#axis == Axis::Z { return 20754; }
        if block_state.r#axis == Axis::X { return 20752; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20753 {
            return Some(StrippedWarpedHyphae {
                r#axis: Axis::Y,
            });
        }
        if state_id == 20754 {
            return Some(StrippedWarpedHyphae {
                r#axis: Axis::Z,
            });
        }
        if state_id == 20752 {
            return Some(StrippedWarpedHyphae {
                r#axis: Axis::X,
            });
        }
        return None;
    }
}

