use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedAcaciaWood {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedAcaciaWood {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::X { return 237; }
        if self.r#axis == Axis::Z { return 239; }
        if self.r#axis == Axis::Y { return 238; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 237 {
            return Some(StrippedAcaciaWood {
                r#axis: Axis::X,
            });
        }
        if state_id == 239 {
            return Some(StrippedAcaciaWood {
                r#axis: Axis::Z,
            });
        }
        if state_id == 238 {
            return Some(StrippedAcaciaWood {
                r#axis: Axis::Y,
            });
        }
        return None;
    }
}

