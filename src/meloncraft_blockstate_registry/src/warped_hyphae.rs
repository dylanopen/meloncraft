use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedHyphae {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for WarpedHyphae {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::X { return 20749; }
        if self.r#axis == Axis::Y { return 20750; }
        if self.r#axis == Axis::Z { return 20751; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20749 {
            return Some(WarpedHyphae {
                r#axis: Axis::X,
            });
        }
        if state_id == 20750 {
            return Some(WarpedHyphae {
                r#axis: Axis::Y,
            });
        }
        if state_id == 20751 {
            return Some(WarpedHyphae {
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

