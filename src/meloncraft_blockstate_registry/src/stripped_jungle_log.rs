use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedJungleLog {
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedJungleLog {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::X {
            return 177;
        }
        if self.r#axis == Axis::Z {
            return 179;
        }
        if self.r#axis == Axis::Y {
            return 178;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 177 {
            return Some(StrippedJungleLog { r#axis: Axis::X });
        }
        if state_id == 179 {
            return Some(StrippedJungleLog { r#axis: Axis::Z });
        }
        if state_id == 178 {
            return Some(StrippedJungleLog { r#axis: Axis::Y });
        }
        return None;
    }
}
