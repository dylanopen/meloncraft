use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedBambooBlock {
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedBambooBlock {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Y {
            return 199;
        }
        if self.r#axis == Axis::Z {
            return 200;
        }
        if self.r#axis == Axis::X {
            return 198;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 199 {
            return Some(StrippedBambooBlock { r#axis: Axis::Y });
        }
        if state_id == 200 {
            return Some(StrippedBambooBlock { r#axis: Axis::Z });
        }
        if state_id == 198 {
            return Some(StrippedBambooBlock { r#axis: Axis::X });
        }
        return None;
    }
}
