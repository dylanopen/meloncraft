use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakWood {
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for DarkOakWood {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Z {
            return 221;
        }
        if self.r#axis == Axis::X {
            return 219;
        }
        if self.r#axis == Axis::Y {
            return 220;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 221 {
            return Some(DarkOakWood { r#axis: Axis::Z });
        }
        if state_id == 219 {
            return Some(DarkOakWood { r#axis: Axis::X });
        }
        if state_id == 220 {
            return Some(DarkOakWood { r#axis: Axis::Y });
        }
        return None;
    }
}
