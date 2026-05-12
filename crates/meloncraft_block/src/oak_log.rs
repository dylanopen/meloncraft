use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakLog {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for OakLog {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::X { return 136; }
        if block_state.r#axis == Axis::Y { return 137; }
        if block_state.r#axis == Axis::Z { return 138; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 136 {
            return Some(OakLog {
                r#axis: Axis::X,
            });
        }
        if state_id == 137 {
            return Some(OakLog {
                r#axis: Axis::Y,
            });
        }
        if state_id == 138 {
            return Some(OakLog {
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

