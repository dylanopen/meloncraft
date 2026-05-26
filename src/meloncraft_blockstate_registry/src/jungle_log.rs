use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleLog {
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for JungleLog {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Z {
            return 147;
        }
        if self.r#axis == Axis::X {
            return 145;
        }
        if self.r#axis == Axis::Y {
            return 146;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 147 {
            return Some(JungleLog { r#axis: Axis::Z });
        }
        if state_id == 145 {
            return Some(JungleLog { r#axis: Axis::X });
        }
        if state_id == 146 {
            return Some(JungleLog { r#axis: Axis::Y });
        }
        return None;
    }
}
