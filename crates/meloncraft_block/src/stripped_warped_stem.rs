use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedWarpedStem {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedWarpedStem {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::X { return 20746; }
        if self.r#axis == Axis::Z { return 20748; }
        if self.r#axis == Axis::Y { return 20747; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20746 {
            return Some(StrippedWarpedStem {
                r#axis: Axis::X,
            });
        }
        if state_id == 20748 {
            return Some(StrippedWarpedStem {
                r#axis: Axis::Z,
            });
        }
        if state_id == 20747 {
            return Some(StrippedWarpedStem {
                r#axis: Axis::Y,
            });
        }
        return None;
    }
}

