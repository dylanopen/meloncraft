use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCopperChain {
    pub waterlogged: bool,
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for ExposedCopperChain {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#axis == Axis::Z {
            return 8062;
        }
        if self.r#axis == Axis::X && self.r#waterlogged == false {
            return 8058;
        }
        if self.r#axis == Axis::Y && self.r#waterlogged == true {
            return 8059;
        }
        if self.r#waterlogged == true && self.r#axis == Axis::Z {
            return 8061;
        }
        if self.r#waterlogged == true && self.r#axis == Axis::X {
            return 8057;
        }
        if self.r#axis == Axis::Y && self.r#waterlogged == false {
            return 8060;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8062 {
            return Some(ExposedCopperChain {
                r#waterlogged: false,
                r#axis: Axis::Z,
            });
        }
        if state_id == 8058 {
            return Some(ExposedCopperChain {
                r#axis: Axis::X,
                r#waterlogged: false,
            });
        }
        if state_id == 8059 {
            return Some(ExposedCopperChain {
                r#axis: Axis::Y,
                r#waterlogged: true,
            });
        }
        if state_id == 8061 {
            return Some(ExposedCopperChain {
                r#waterlogged: true,
                r#axis: Axis::Z,
            });
        }
        if state_id == 8057 {
            return Some(ExposedCopperChain {
                r#waterlogged: true,
                r#axis: Axis::X,
            });
        }
        if state_id == 8060 {
            return Some(ExposedCopperChain {
                r#axis: Axis::Y,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
