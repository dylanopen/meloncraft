use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deepslate {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for Deepslate {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Y { return 27722; }
        if self.r#axis == Axis::Z { return 27723; }
        if self.r#axis == Axis::X { return 27721; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27722 {
            return Some(Deepslate {
                r#axis: Axis::Y,
            });
        }
        if state_id == 27723 {
            return Some(Deepslate {
                r#axis: Axis::Z,
            });
        }
        if state_id == 27721 {
            return Some(Deepslate {
                r#axis: Axis::X,
            });
        }
        return None;
    }
}

