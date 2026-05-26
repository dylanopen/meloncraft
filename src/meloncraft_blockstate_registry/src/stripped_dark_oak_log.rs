use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedDarkOakLog {
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedDarkOakLog {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::X {
            return 186;
        }
        if self.r#axis == Axis::Z {
            return 188;
        }
        if self.r#axis == Axis::Y {
            return 187;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 186 {
            return Some(StrippedDarkOakLog { r#axis: Axis::X });
        }
        if state_id == 188 {
            return Some(StrippedDarkOakLog { r#axis: Axis::Z });
        }
        if state_id == 187 {
            return Some(StrippedDarkOakLog { r#axis: Axis::Y });
        }
        return None;
    }
}
