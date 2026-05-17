use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedSpruceWood {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedSpruceWood {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::X { return 228; }
        if self.r#axis == Axis::Y { return 229; }
        if self.r#axis == Axis::Z { return 230; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 228 {
            return Some(StrippedSpruceWood {
                r#axis: Axis::X,
            });
        }
        if state_id == 229 {
            return Some(StrippedSpruceWood {
                r#axis: Axis::Y,
            });
        }
        if state_id == 230 {
            return Some(StrippedSpruceWood {
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

