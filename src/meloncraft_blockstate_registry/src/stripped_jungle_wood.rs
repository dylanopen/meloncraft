use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedJungleWood {
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedJungleWood {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::X {
            return 234;
        }
        if self.r#axis == Axis::Y {
            return 235;
        }
        if self.r#axis == Axis::Z {
            return 236;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 234 {
            return Some(StrippedJungleWood { r#axis: Axis::X });
        }
        if state_id == 235 {
            return Some(StrippedJungleWood { r#axis: Axis::Y });
        }
        if state_id == 236 {
            return Some(StrippedJungleWood { r#axis: Axis::Z });
        }
        return None;
    }
}
