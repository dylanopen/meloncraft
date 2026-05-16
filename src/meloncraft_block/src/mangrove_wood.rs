use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveWood {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for MangroveWood {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Y { return 223; }
        if self.r#axis == Axis::X { return 222; }
        if self.r#axis == Axis::Z { return 224; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 223 {
            return Some(MangroveWood {
                r#axis: Axis::Y,
            });
        }
        if state_id == 222 {
            return Some(MangroveWood {
                r#axis: Axis::X,
            });
        }
        if state_id == 224 {
            return Some(MangroveWood {
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

