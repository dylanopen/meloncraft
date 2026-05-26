use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedDarkOakWood {
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedDarkOakWood {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Y {
            return 244;
        }
        if self.r#axis == Axis::Z {
            return 245;
        }
        if self.r#axis == Axis::X {
            return 243;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 244 {
            return Some(StrippedDarkOakWood { r#axis: Axis::Y });
        }
        if state_id == 245 {
            return Some(StrippedDarkOakWood { r#axis: Axis::Z });
        }
        if state_id == 243 {
            return Some(StrippedDarkOakWood { r#axis: Axis::X });
        }
        return None;
    }
}
