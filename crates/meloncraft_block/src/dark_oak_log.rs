use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakLog {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for DarkOakLog {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::X { return 154; }
        if block_state.r#axis == Axis::Y { return 155; }
        if block_state.r#axis == Axis::Z { return 156; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 154 {
            return Some(DarkOakLog {
                r#axis: Axis::X,
            });
        }
        if state_id == 155 {
            return Some(DarkOakLog {
                r#axis: Axis::Y,
            });
        }
        if state_id == 156 {
            return Some(DarkOakLog {
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

