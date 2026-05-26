use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedCopperBars {
    pub east: bool,
    pub waterlogged: bool,
    pub west: bool,
    pub north: bool,
    pub south: bool,
}

impl BlockState for WaxedExposedCopperBars {
    fn to_id(&self) -> i32 {
        if self.r#south == true
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#east == true
            && self.r#west == true
        {
            return 7951;
        }
        if self.r#south == false
            && self.r#north == false
            && self.r#west == false
            && self.r#east == true
            && self.r#waterlogged == false
        {
            return 7964;
        }
        if self.r#north == false
            && self.r#south == true
            && self.r#east == true
            && self.r#waterlogged == true
            && self.r#west == false
        {
            return 7958;
        }
        if self.r#west == false
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#east == true
            && self.r#north == true
        {
            return 7954;
        }
        if self.r#waterlogged == true
            && self.r#south == true
            && self.r#north == true
            && self.r#east == true
            && self.r#west == true
        {
            return 7949;
        }
        if self.r#north == true
            && self.r#east == true
            && self.r#west == true
            && self.r#south == false
            && self.r#waterlogged == false
        {
            return 7955;
        }
        if self.r#north == false
            && self.r#west == true
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#south == true
        {
            return 7973;
        }
        if self.r#south == true
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#east == false
            && self.r#west == false
        {
            return 7974;
        }
        if self.r#west == true
            && self.r#north == false
            && self.r#waterlogged == false
            && self.r#south == false
            && self.r#east == false
        {
            return 7979;
        }
        if self.r#waterlogged == false
            && self.r#east == false
            && self.r#north == false
            && self.r#south == true
            && self.r#west == false
        {
            return 7976;
        }
        if self.r#east == true
            && self.r#west == false
            && self.r#waterlogged == false
            && self.r#north == true
            && self.r#south == true
        {
            return 7952;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#west == true
            && self.r#waterlogged == true
            && self.r#south == false
        {
            return 7969;
        }
        if self.r#east == true
            && self.r#south == true
            && self.r#west == true
            && self.r#north == false
            && self.r#waterlogged == true
        {
            return 7957;
        }
        if self.r#south == false
            && self.r#east == false
            && self.r#west == false
            && self.r#north == false
            && self.r#waterlogged == false
        {
            return 7980;
        }
        if self.r#east == true
            && self.r#north == true
            && self.r#west == false
            && self.r#waterlogged == false
            && self.r#south == false
        {
            return 7956;
        }
        if self.r#north == true
            && self.r#east == false
            && self.r#south == true
            && self.r#waterlogged == true
            && self.r#west == false
        {
            return 7966;
        }
        if self.r#south == true
            && self.r#east == true
            && self.r#north == true
            && self.r#west == false
            && self.r#waterlogged == true
        {
            return 7950;
        }
        if self.r#north == false
            && self.r#east == true
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#west == false
        {
            return 7960;
        }
        if self.r#east == false
            && self.r#waterlogged == true
            && self.r#north == true
            && self.r#west == true
            && self.r#south == true
        {
            return 7965;
        }
        if self.r#east == true
            && self.r#south == false
            && self.r#west == false
            && self.r#waterlogged == true
            && self.r#north == false
        {
            return 7962;
        }
        if self.r#south == false
            && self.r#north == true
            && self.r#east == false
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 7971;
        }
        if self.r#south == true
            && self.r#north == false
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#east == false
        {
            return 7975;
        }
        if self.r#east == false
            && self.r#west == false
            && self.r#north == true
            && self.r#south == true
            && self.r#waterlogged == false
        {
            return 7968;
        }
        if self.r#east == false
            && self.r#waterlogged == false
            && self.r#south == true
            && self.r#west == true
            && self.r#north == true
        {
            return 7967;
        }
        if self.r#east == true
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#north == false
            && self.r#west == true
        {
            return 7961;
        }
        if self.r#west == true
            && self.r#north == true
            && self.r#south == false
            && self.r#east == true
            && self.r#waterlogged == true
        {
            return 7953;
        }
        if self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#north == true
            && self.r#south == false
        {
            return 7970;
        }
        if self.r#west == true
            && self.r#waterlogged == false
            && self.r#north == false
            && self.r#south == true
            && self.r#east == true
        {
            return 7959;
        }
        if self.r#north == true
            && self.r#waterlogged == false
            && self.r#south == false
            && self.r#west == false
            && self.r#east == false
        {
            return 7972;
        }
        if self.r#north == false
            && self.r#east == false
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#west == false
        {
            return 7978;
        }
        if self.r#north == false
            && self.r#south == false
            && self.r#west == true
            && self.r#east == true
            && self.r#waterlogged == false
        {
            return 7963;
        }
        if self.r#east == false
            && self.r#north == false
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#west == true
        {
            return 7977;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7951 {
            return Some(WaxedExposedCopperBars {
                r#south: true,
                r#north: true,
                r#waterlogged: false,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 7964 {
            return Some(WaxedExposedCopperBars {
                r#south: false,
                r#north: false,
                r#west: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7958 {
            return Some(WaxedExposedCopperBars {
                r#north: false,
                r#south: true,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 7954 {
            return Some(WaxedExposedCopperBars {
                r#west: false,
                r#south: false,
                r#waterlogged: true,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 7949 {
            return Some(WaxedExposedCopperBars {
                r#waterlogged: true,
                r#south: true,
                r#north: true,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 7955 {
            return Some(WaxedExposedCopperBars {
                r#north: true,
                r#east: true,
                r#west: true,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7973 {
            return Some(WaxedExposedCopperBars {
                r#north: false,
                r#west: true,
                r#east: false,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 7974 {
            return Some(WaxedExposedCopperBars {
                r#south: true,
                r#north: false,
                r#waterlogged: true,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 7979 {
            return Some(WaxedExposedCopperBars {
                r#west: true,
                r#north: false,
                r#waterlogged: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 7976 {
            return Some(WaxedExposedCopperBars {
                r#waterlogged: false,
                r#east: false,
                r#north: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 7952 {
            return Some(WaxedExposedCopperBars {
                r#east: true,
                r#west: false,
                r#waterlogged: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 7969 {
            return Some(WaxedExposedCopperBars {
                r#east: false,
                r#north: true,
                r#west: true,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 7957 {
            return Some(WaxedExposedCopperBars {
                r#east: true,
                r#south: true,
                r#west: true,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7980 {
            return Some(WaxedExposedCopperBars {
                r#south: false,
                r#east: false,
                r#west: false,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7956 {
            return Some(WaxedExposedCopperBars {
                r#east: true,
                r#north: true,
                r#west: false,
                r#waterlogged: false,
                r#south: false,
            });
        }
        if state_id == 7966 {
            return Some(WaxedExposedCopperBars {
                r#north: true,
                r#east: false,
                r#south: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 7950 {
            return Some(WaxedExposedCopperBars {
                r#south: true,
                r#east: true,
                r#north: true,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7960 {
            return Some(WaxedExposedCopperBars {
                r#north: false,
                r#east: true,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 7965 {
            return Some(WaxedExposedCopperBars {
                r#east: false,
                r#waterlogged: true,
                r#north: true,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 7962 {
            return Some(WaxedExposedCopperBars {
                r#east: true,
                r#south: false,
                r#west: false,
                r#waterlogged: true,
                r#north: false,
            });
        }
        if state_id == 7971 {
            return Some(WaxedExposedCopperBars {
                r#south: false,
                r#north: true,
                r#east: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 7975 {
            return Some(WaxedExposedCopperBars {
                r#south: true,
                r#north: false,
                r#waterlogged: false,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 7968 {
            return Some(WaxedExposedCopperBars {
                r#east: false,
                r#west: false,
                r#north: true,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7967 {
            return Some(WaxedExposedCopperBars {
                r#east: false,
                r#waterlogged: false,
                r#south: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 7961 {
            return Some(WaxedExposedCopperBars {
                r#east: true,
                r#south: false,
                r#waterlogged: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 7953 {
            return Some(WaxedExposedCopperBars {
                r#west: true,
                r#north: true,
                r#south: false,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7970 {
            return Some(WaxedExposedCopperBars {
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 7959 {
            return Some(WaxedExposedCopperBars {
                r#west: true,
                r#waterlogged: false,
                r#north: false,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 7972 {
            return Some(WaxedExposedCopperBars {
                r#north: true,
                r#waterlogged: false,
                r#south: false,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 7978 {
            return Some(WaxedExposedCopperBars {
                r#north: false,
                r#east: false,
                r#south: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 7963 {
            return Some(WaxedExposedCopperBars {
                r#north: false,
                r#south: false,
                r#west: true,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7977 {
            return Some(WaxedExposedCopperBars {
                r#east: false,
                r#north: false,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        return None;
    }
}
