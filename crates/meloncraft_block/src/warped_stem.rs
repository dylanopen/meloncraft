use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedStem {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for WarpedStem {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::X { return 20743; }
        if block_state.r#axis == Axis::Y { return 20744; }
        if block_state.r#axis == Axis::Z { return 20745; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20743 {
            return Some(WarpedStem {
                r#axis: Axis::X,
            });
        }
        if state_id == 20744 {
            return Some(WarpedStem {
                r#axis: Axis::Y,
            });
        }
        if state_id == 20745 {
            return Some(WarpedStem {
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

