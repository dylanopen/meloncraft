use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedOakLog {
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedOakLog {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Y {
            return 193;
        }
        if self.r#axis == Axis::X {
            return 192;
        }
        if self.r#axis == Axis::Z {
            return 194;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 193 {
            return Some(StrippedOakLog { r#axis: Axis::Y });
        }
        if state_id == 192 {
            return Some(StrippedOakLog { r#axis: Axis::X });
        }
        if state_id == 194 {
            return Some(StrippedOakLog { r#axis: Axis::Z });
        }
        return None;
    }
}
