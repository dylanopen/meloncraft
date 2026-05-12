use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedSpruceLog {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedSpruceLog {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::Y { return 172; }
        if block_state.r#axis == Axis::X { return 171; }
        if block_state.r#axis == Axis::Z { return 173; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 172 {
            return Some(StrippedSpruceLog {
                r#axis: Axis::Y,
            });
        }
        if state_id == 171 {
            return Some(StrippedSpruceLog {
                r#axis: Axis::X,
            });
        }
        if state_id == 173 {
            return Some(StrippedSpruceLog {
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

