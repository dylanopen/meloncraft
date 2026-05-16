use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoneBlock {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for BoneBlock {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Y { return 14647; }
        if self.r#axis == Axis::Z { return 14648; }
        if self.r#axis == Axis::X { return 14646; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14647 {
            return Some(BoneBlock {
                r#axis: Axis::Y,
            });
        }
        if state_id == 14648 {
            return Some(BoneBlock {
                r#axis: Axis::Z,
            });
        }
        if state_id == 14646 {
            return Some(BoneBlock {
                r#axis: Axis::X,
            });
        }
        return None;
    }
}

