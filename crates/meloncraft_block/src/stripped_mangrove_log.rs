use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedMangroveLog {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedMangroveLog {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::Y { return 196; }
        if block_state.r#axis == Axis::Z { return 197; }
        if block_state.r#axis == Axis::X { return 195; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 196 {
            return Some(StrippedMangroveLog {
                r#axis: Axis::Y,
            });
        }
        if state_id == 197 {
            return Some(StrippedMangroveLog {
                r#axis: Axis::Z,
            });
        }
        if state_id == 195 {
            return Some(StrippedMangroveLog {
                r#axis: Axis::X,
            });
        }
        return None;
    }
}

