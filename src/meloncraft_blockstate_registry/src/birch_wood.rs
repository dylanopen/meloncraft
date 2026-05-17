use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchWood {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for BirchWood {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Z { return 209; }
        if self.r#axis == Axis::X { return 207; }
        if self.r#axis == Axis::Y { return 208; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 209 {
            return Some(BirchWood {
                r#axis: Axis::Z,
            });
        }
        if state_id == 207 {
            return Some(BirchWood {
                r#axis: Axis::X,
            });
        }
        if state_id == 208 {
            return Some(BirchWood {
                r#axis: Axis::Y,
            });
        }
        return None;
    }
}

