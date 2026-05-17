use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedMangroveWood {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedMangroveWood {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Y { return 250; }
        if self.r#axis == Axis::Z { return 251; }
        if self.r#axis == Axis::X { return 249; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 250 {
            return Some(StrippedMangroveWood {
                r#axis: Axis::Y,
            });
        }
        if state_id == 251 {
            return Some(StrippedMangroveWood {
                r#axis: Axis::Z,
            });
        }
        if state_id == 249 {
            return Some(StrippedMangroveWood {
                r#axis: Axis::X,
            });
        }
        return None;
    }
}

