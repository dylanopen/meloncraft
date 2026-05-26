use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedBirchLog {
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedBirchLog {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::X {
            return 174;
        }
        if self.r#axis == Axis::Y {
            return 175;
        }
        if self.r#axis == Axis::Z {
            return 176;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 174 {
            return Some(StrippedBirchLog { r#axis: Axis::X });
        }
        if state_id == 175 {
            return Some(StrippedBirchLog { r#axis: Axis::Y });
        }
        if state_id == 176 {
            return Some(StrippedBirchLog { r#axis: Axis::Z });
        }
        return None;
    }
}
