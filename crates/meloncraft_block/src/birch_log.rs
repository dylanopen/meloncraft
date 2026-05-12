use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchLog {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for BirchLog {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::X { return 142; }
        if block_state.r#axis == Axis::Z { return 144; }
        if block_state.r#axis == Axis::Y { return 143; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 142 {
            return Some(BirchLog {
                r#axis: Axis::X,
            });
        }
        if state_id == 144 {
            return Some(BirchLog {
                r#axis: Axis::Z,
            });
        }
        if state_id == 143 {
            return Some(BirchLog {
                r#axis: Axis::Y,
            });
        }
        return None;
    }
}

