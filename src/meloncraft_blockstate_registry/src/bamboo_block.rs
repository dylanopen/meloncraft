use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooBlock {
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for BambooBlock {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Y {
            return 169;
        }
        if self.r#axis == Axis::Z {
            return 170;
        }
        if self.r#axis == Axis::X {
            return 168;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 169 {
            return Some(BambooBlock { r#axis: Axis::Y });
        }
        if state_id == 170 {
            return Some(BambooBlock { r#axis: Axis::Z });
        }
        if state_id == 168 {
            return Some(BambooBlock { r#axis: Axis::X });
        }
        return None;
    }
}
