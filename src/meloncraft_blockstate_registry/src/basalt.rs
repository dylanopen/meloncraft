use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Basalt {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for Basalt {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Y { return 6800; }
        if self.r#axis == Axis::Z { return 6801; }
        if self.r#axis == Axis::X { return 6799; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6800 {
            return Some(Basalt {
                r#axis: Axis::Y,
            });
        }
        if state_id == 6801 {
            return Some(Basalt {
                r#axis: Axis::Z,
            });
        }
        if state_id == 6799 {
            return Some(Basalt {
                r#axis: Axis::X,
            });
        }
        return None;
    }
}

