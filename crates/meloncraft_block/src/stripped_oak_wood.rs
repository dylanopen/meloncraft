use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedOakWood {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedOakWood {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Z { return 227; }
        if self.r#axis == Axis::X { return 225; }
        if self.r#axis == Axis::Y { return 226; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 227 {
            return Some(StrippedOakWood {
                r#axis: Axis::Z,
            });
        }
        if state_id == 225 {
            return Some(StrippedOakWood {
                r#axis: Axis::X,
            });
        }
        if state_id == 226 {
            return Some(StrippedOakWood {
                r#axis: Axis::Y,
            });
        }
        return None;
    }
}

