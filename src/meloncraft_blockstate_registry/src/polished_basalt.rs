use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedBasalt {
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for PolishedBasalt {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Y {
            return 6803;
        }
        if self.r#axis == Axis::Z {
            return 6804;
        }
        if self.r#axis == Axis::X {
            return 6802;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6803 {
            return Some(PolishedBasalt { r#axis: Axis::Y });
        }
        if state_id == 6804 {
            return Some(PolishedBasalt { r#axis: Axis::Z });
        }
        if state_id == 6802 {
            return Some(PolishedBasalt { r#axis: Axis::X });
        }
        return None;
    }
}
