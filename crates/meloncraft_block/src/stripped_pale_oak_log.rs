use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedPaleOakLog {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedPaleOakLog {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::X { return 189; }
        if block_state.r#axis == Axis::Z { return 191; }
        if block_state.r#axis == Axis::Y { return 190; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 189 {
            return Some(StrippedPaleOakLog {
                r#axis: Axis::X,
            });
        }
        if state_id == 191 {
            return Some(StrippedPaleOakLog {
                r#axis: Axis::Z,
            });
        }
        if state_id == 190 {
            return Some(StrippedPaleOakLog {
                r#axis: Axis::Y,
            });
        }
        return None;
    }
}

