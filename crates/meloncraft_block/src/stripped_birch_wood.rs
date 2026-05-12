use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedBirchWood {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedBirchWood {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::X { return 231; }
        if self.r#axis == Axis::Y { return 232; }
        if self.r#axis == Axis::Z { return 233; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 231 {
            return Some(StrippedBirchWood {
                r#axis: Axis::X,
            });
        }
        if state_id == 232 {
            return Some(StrippedBirchWood {
                r#axis: Axis::Y,
            });
        }
        if state_id == 233 {
            return Some(StrippedBirchWood {
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

