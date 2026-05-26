use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedCherryWood {
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedCherryWood {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Z {
            return 242;
        }
        if self.r#axis == Axis::X {
            return 240;
        }
        if self.r#axis == Axis::Y {
            return 241;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 242 {
            return Some(StrippedCherryWood { r#axis: Axis::Z });
        }
        if state_id == 240 {
            return Some(StrippedCherryWood { r#axis: Axis::X });
        }
        if state_id == 241 {
            return Some(StrippedCherryWood { r#axis: Axis::Y });
        }
        return None;
    }
}
