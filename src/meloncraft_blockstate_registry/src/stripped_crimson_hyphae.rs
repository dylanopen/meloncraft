use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedCrimsonHyphae {
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedCrimsonHyphae {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Z {
            return 20771;
        }
        if self.r#axis == Axis::X {
            return 20769;
        }
        if self.r#axis == Axis::Y {
            return 20770;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20771 {
            return Some(StrippedCrimsonHyphae { r#axis: Axis::Z });
        }
        if state_id == 20769 {
            return Some(StrippedCrimsonHyphae { r#axis: Axis::X });
        }
        if state_id == 20770 {
            return Some(StrippedCrimsonHyphae { r#axis: Axis::Y });
        }
        return None;
    }
}
