use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuartzPillar {
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for QuartzPillar {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Y {
            return 11124;
        }
        if self.r#axis == Axis::Z {
            return 11125;
        }
        if self.r#axis == Axis::X {
            return 11123;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11124 {
            return Some(QuartzPillar { r#axis: Axis::Y });
        }
        if state_id == 11125 {
            return Some(QuartzPillar { r#axis: Axis::Z });
        }
        if state_id == 11123 {
            return Some(QuartzPillar { r#axis: Axis::X });
        }
        return None;
    }
}
