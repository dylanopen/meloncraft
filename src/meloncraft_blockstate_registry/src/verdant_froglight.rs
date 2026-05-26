use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerdantFroglight {
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for VerdantFroglight {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::X {
            return 29383;
        }
        if self.r#axis == Axis::Y {
            return 29384;
        }
        if self.r#axis == Axis::Z {
            return 29385;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29383 {
            return Some(VerdantFroglight { r#axis: Axis::X });
        }
        if state_id == 29384 {
            return Some(VerdantFroglight { r#axis: Axis::Y });
        }
        if state_id == 29385 {
            return Some(VerdantFroglight { r#axis: Axis::Z });
        }
        return None;
    }
}
