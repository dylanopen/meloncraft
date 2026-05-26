use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCopperChain {
    pub waterlogged: bool,
    pub r#axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for WaxedWeatheredCopperChain {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Z && self.r#waterlogged == false {
            return 8092;
        }
        if self.r#waterlogged == false && self.r#axis == Axis::Y {
            return 8090;
        }
        if self.r#axis == Axis::Y && self.r#waterlogged == true {
            return 8089;
        }
        if self.r#axis == Axis::X && self.r#waterlogged == true {
            return 8087;
        }
        if self.r#waterlogged == false && self.r#axis == Axis::X {
            return 8088;
        }
        if self.r#waterlogged == true && self.r#axis == Axis::Z {
            return 8091;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8092 {
            return Some(WaxedWeatheredCopperChain {
                r#axis: Axis::Z,
                r#waterlogged: false,
            });
        }
        if state_id == 8090 {
            return Some(WaxedWeatheredCopperChain {
                r#waterlogged: false,
                r#axis: Axis::Y,
            });
        }
        if state_id == 8089 {
            return Some(WaxedWeatheredCopperChain {
                r#axis: Axis::Y,
                r#waterlogged: true,
            });
        }
        if state_id == 8087 {
            return Some(WaxedWeatheredCopperChain {
                r#axis: Axis::X,
                r#waterlogged: true,
            });
        }
        if state_id == 8088 {
            return Some(WaxedWeatheredCopperChain {
                r#waterlogged: false,
                r#axis: Axis::X,
            });
        }
        if state_id == 8091 {
            return Some(WaxedWeatheredCopperChain {
                r#waterlogged: true,
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}
