use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OchreFroglight {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for OchreFroglight {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::X { return 29380; }
        if self.r#axis == Axis::Y { return 29381; }
        if self.r#axis == Axis::Z { return 29382; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29380 {
            return Some(OchreFroglight {
                r#axis: Axis::X,
            });
        }
        if state_id == 29381 {
            return Some(OchreFroglight {
                r#axis: Axis::Y,
            });
        }
        if state_id == 29382 {
            return Some(OchreFroglight {
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

