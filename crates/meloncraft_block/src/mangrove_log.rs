use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveLog {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for MangroveLog {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Y { return 161; }
        if self.r#axis == Axis::Z { return 162; }
        if self.r#axis == Axis::X { return 160; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 161 {
            return Some(MangroveLog {
                r#axis: Axis::Y,
            });
        }
        if state_id == 162 {
            return Some(MangroveLog {
                r#axis: Axis::Z,
            });
        }
        if state_id == 160 {
            return Some(MangroveLog {
                r#axis: Axis::X,
            });
        }
        return None;
    }
}

