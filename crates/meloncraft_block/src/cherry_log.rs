use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryLog {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for CherryLog {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::X { return 151; }
        if block_state.r#axis == Axis::Z { return 153; }
        if block_state.r#axis == Axis::Y { return 152; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 151 {
            return Some(CherryLog {
                r#axis: Axis::X,
            });
        }
        if state_id == 153 {
            return Some(CherryLog {
                r#axis: Axis::Z,
            });
        }
        if state_id == 152 {
            return Some(CherryLog {
                r#axis: Axis::Y,
            });
        }
        return None;
    }
}

