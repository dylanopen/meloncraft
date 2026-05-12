use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaLog {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for AcaciaLog {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::X { return 148; }
        if self.r#axis == Axis::Z { return 150; }
        if self.r#axis == Axis::Y { return 149; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 148 {
            return Some(AcaciaLog {
                r#axis: Axis::X,
            });
        }
        if state_id == 150 {
            return Some(AcaciaLog {
                r#axis: Axis::Z,
            });
        }
        if state_id == 149 {
            return Some(AcaciaLog {
                r#axis: Axis::Y,
            });
        }
        return None;
    }
}

