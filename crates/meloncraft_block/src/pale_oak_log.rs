use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakLog {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for PaleOakLog {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::X { return 157; }
        if self.r#axis == Axis::Z { return 159; }
        if self.r#axis == Axis::Y { return 158; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 157 {
            return Some(PaleOakLog {
                r#axis: Axis::X,
            });
        }
        if state_id == 159 {
            return Some(PaleOakLog {
                r#axis: Axis::Z,
            });
        }
        if state_id == 158 {
            return Some(PaleOakLog {
                r#axis: Axis::Y,
            });
        }
        return None;
    }
}

