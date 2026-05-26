use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonStem {
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for CrimsonStem {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::X {
            return 20760;
        }
        if self.r#axis == Axis::Z {
            return 20762;
        }
        if self.r#axis == Axis::Y {
            return 20761;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20760 {
            return Some(CrimsonStem { r#axis: Axis::X });
        }
        if state_id == 20762 {
            return Some(CrimsonStem { r#axis: Axis::Z });
        }
        if state_id == 20761 {
            return Some(CrimsonStem { r#axis: Axis::Y });
        }
        return None;
    }
}
