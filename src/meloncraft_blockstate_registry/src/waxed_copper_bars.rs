use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCopperBars {
    pub waterlogged: bool,
    pub north: bool,
    pub east: bool,
    pub south: bool,
    pub west: bool,
}

impl BlockState for WaxedCopperBars {
    fn to_id(&self) -> i32 {
        if self.r#west == true
            && self.r#south == false
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#east == true
        {
            return 7923;
        }
        if self.r#south == true
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#east == false
            && self.r#west == false
        {
            return 7942;
        }
        if self.r#east == true
            && self.r#south == true
            && self.r#west == true
            && self.r#waterlogged == false
            && self.r#north == false
        {
            return 7927;
        }
        if self.r#waterlogged == true
            && self.r#north == true
            && self.r#south == false
            && self.r#west == true
            && self.r#east == true
        {
            return 7921;
        }
        if self.r#east == false
            && self.r#south == true
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#west == false
        {
            return 7936;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#west == false
            && self.r#south == false
            && self.r#waterlogged == true
        {
            return 7938;
        }
        if self.r#west == false
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#east == false
            && self.r#south == false
        {
            return 7946;
        }
        if self.r#east == false
            && self.r#north == false
            && self.r#south == false
            && self.r#west == true
            && self.r#waterlogged == true
        {
            return 7945;
        }
        if self.r#south == true
            && self.r#north == true
            && self.r#west == false
            && self.r#east == true
            && self.r#waterlogged == true
        {
            return 7918;
        }
        if self.r#south == false
            && self.r#north == true
            && self.r#east == false
            && self.r#west == true
            && self.r#waterlogged == false
        {
            return 7939;
        }
        if self.r#waterlogged == false
            && self.r#north == true
            && self.r#south == false
            && self.r#east == true
            && self.r#west == false
        {
            return 7924;
        }
        if self.r#east == true
            && self.r#west == true
            && self.r#north == true
            && self.r#south == true
            && self.r#waterlogged == false
        {
            return 7919;
        }
        if self.r#east == false
            && self.r#north == false
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#south == true
        {
            return 7944;
        }
        if self.r#east == true
            && self.r#waterlogged == false
            && self.r#south == false
            && self.r#north == false
            && self.r#west == false
        {
            return 7932;
        }
        if self.r#waterlogged == true
            && self.r#east == true
            && self.r#south == true
            && self.r#north == true
            && self.r#west == true
        {
            return 7917;
        }
        if self.r#east == true
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#south == true
            && self.r#west == false
        {
            return 7920;
        }
        if self.r#east == false
            && self.r#south == true
            && self.r#waterlogged == true
            && self.r#north == true
            && self.r#west == true
        {
            return 7933;
        }
        if self.r#south == false
            && self.r#waterlogged == false
            && self.r#east == true
            && self.r#north == false
            && self.r#west == true
        {
            return 7931;
        }
        if self.r#north == false
            && self.r#west == false
            && self.r#east == false
            && self.r#south == false
            && self.r#waterlogged == false
        {
            return 7948;
        }
        if self.r#north == false
            && self.r#south == false
            && self.r#west == false
            && self.r#east == true
            && self.r#waterlogged == true
        {
            return 7930;
        }
        if self.r#south == false
            && self.r#west == true
            && self.r#waterlogged == true
            && self.r#north == false
            && self.r#east == true
        {
            return 7929;
        }
        if self.r#south == true
            && self.r#waterlogged == false
            && self.r#north == false
            && self.r#east == false
            && self.r#west == true
        {
            return 7943;
        }
        if self.r#east == true
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#north == false
            && self.r#south == true
        {
            return 7928;
        }
        if self.r#north == true
            && self.r#south == true
            && self.r#east == false
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 7935;
        }
        if self.r#north == true
            && self.r#south == true
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#east == false
        {
            return 7934;
        }
        if self.r#west == true
            && self.r#north == false
            && self.r#east == true
            && self.r#south == true
            && self.r#waterlogged == true
        {
            return 7925;
        }
        if self.r#south == true
            && self.r#north == false
            && self.r#east == true
            && self.r#west == false
            && self.r#waterlogged == true
        {
            return 7926;
        }
        if self.r#waterlogged == true
            && self.r#south == false
            && self.r#west == true
            && self.r#east == false
            && self.r#north == true
        {
            return 7937;
        }
        if self.r#waterlogged == false
            && self.r#west == false
            && self.r#east == false
            && self.r#south == false
            && self.r#north == true
        {
            return 7940;
        }
        if self.r#waterlogged == true
            && self.r#north == false
            && self.r#south == true
            && self.r#west == true
            && self.r#east == false
        {
            return 7941;
        }
        if self.r#north == false
            && self.r#waterlogged == false
            && self.r#south == false
            && self.r#east == false
            && self.r#west == true
        {
            return 7947;
        }
        if self.r#waterlogged == true
            && self.r#south == false
            && self.r#east == true
            && self.r#north == true
            && self.r#west == false
        {
            return 7922;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7923 {
            return Some(WaxedCopperBars {
                r#west: true,
                r#south: false,
                r#north: true,
                r#waterlogged: false,
                r#east: true,
            });
        }
        if state_id == 7942 {
            return Some(WaxedCopperBars {
                r#south: true,
                r#north: false,
                r#waterlogged: true,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 7927 {
            return Some(WaxedCopperBars {
                r#east: true,
                r#south: true,
                r#west: true,
                r#waterlogged: false,
                r#north: false,
            });
        }
        if state_id == 7921 {
            return Some(WaxedCopperBars {
                r#waterlogged: true,
                r#north: true,
                r#south: false,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 7936 {
            return Some(WaxedCopperBars {
                r#east: false,
                r#south: true,
                r#north: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 7938 {
            return Some(WaxedCopperBars {
                r#east: false,
                r#north: true,
                r#west: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7946 {
            return Some(WaxedCopperBars {
                r#west: false,
                r#north: false,
                r#waterlogged: true,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 7945 {
            return Some(WaxedCopperBars {
                r#east: false,
                r#north: false,
                r#south: false,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7918 {
            return Some(WaxedCopperBars {
                r#south: true,
                r#north: true,
                r#west: false,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7939 {
            return Some(WaxedCopperBars {
                r#south: false,
                r#north: true,
                r#east: false,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7924 {
            return Some(WaxedCopperBars {
                r#waterlogged: false,
                r#north: true,
                r#south: false,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 7919 {
            return Some(WaxedCopperBars {
                r#east: true,
                r#west: true,
                r#north: true,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7944 {
            return Some(WaxedCopperBars {
                r#east: false,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 7932 {
            return Some(WaxedCopperBars {
                r#east: true,
                r#waterlogged: false,
                r#south: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 7917 {
            return Some(WaxedCopperBars {
                r#waterlogged: true,
                r#east: true,
                r#south: true,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 7920 {
            return Some(WaxedCopperBars {
                r#east: true,
                r#north: true,
                r#waterlogged: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 7933 {
            return Some(WaxedCopperBars {
                r#east: false,
                r#south: true,
                r#waterlogged: true,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 7931 {
            return Some(WaxedCopperBars {
                r#south: false,
                r#waterlogged: false,
                r#east: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 7948 {
            return Some(WaxedCopperBars {
                r#north: false,
                r#west: false,
                r#east: false,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7930 {
            return Some(WaxedCopperBars {
                r#north: false,
                r#south: false,
                r#west: false,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7929 {
            return Some(WaxedCopperBars {
                r#south: false,
                r#west: true,
                r#waterlogged: true,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 7943 {
            return Some(WaxedCopperBars {
                r#south: true,
                r#waterlogged: false,
                r#north: false,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 7928 {
            return Some(WaxedCopperBars {
                r#east: true,
                r#waterlogged: false,
                r#west: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 7935 {
            return Some(WaxedCopperBars {
                r#north: true,
                r#south: true,
                r#east: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 7934 {
            return Some(WaxedCopperBars {
                r#north: true,
                r#south: true,
                r#waterlogged: true,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 7925 {
            return Some(WaxedCopperBars {
                r#west: true,
                r#north: false,
                r#east: true,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7926 {
            return Some(WaxedCopperBars {
                r#south: true,
                r#north: false,
                r#east: true,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7937 {
            return Some(WaxedCopperBars {
                r#waterlogged: true,
                r#south: false,
                r#west: true,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 7940 {
            return Some(WaxedCopperBars {
                r#waterlogged: false,
                r#west: false,
                r#east: false,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 7941 {
            return Some(WaxedCopperBars {
                r#waterlogged: true,
                r#north: false,
                r#south: true,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 7947 {
            return Some(WaxedCopperBars {
                r#north: false,
                r#waterlogged: false,
                r#south: false,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 7922 {
            return Some(WaxedCopperBars {
                r#waterlogged: true,
                r#south: false,
                r#east: true,
                r#north: true,
                r#west: false,
            });
        }
        return None;
    }
}
