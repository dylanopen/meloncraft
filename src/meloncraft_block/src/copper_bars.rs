use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperBars {
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub waterlogged: bool,
    pub west: bool,
}


impl BlockState for CopperBars {
    fn to_id(&self) -> i32 {
        if self.r#east == true && self.r#north == false && self.r#waterlogged == true && self.r#west == true && self.r#south == false { return 7801; }
        if self.r#south == false && self.r#east == false && self.r#north == true && self.r#waterlogged == true && self.r#west == true { return 7809; }
        if self.r#west == true && self.r#north == true && self.r#waterlogged == false && self.r#east == true && self.r#south == false { return 7795; }
        if self.r#west == false && self.r#waterlogged == true && self.r#south == true && self.r#east == false && self.r#north == true { return 7806; }
        if self.r#east == false && self.r#waterlogged == false && self.r#west == true && self.r#north == true && self.r#south == false { return 7811; }
        if self.r#east == true && self.r#north == false && self.r#waterlogged == false && self.r#west == false && self.r#south == true { return 7800; }
        if self.r#north == true && self.r#south == false && self.r#waterlogged == false && self.r#east == false && self.r#west == false { return 7812; }
        if self.r#west == true && self.r#south == true && self.r#east == false && self.r#waterlogged == true && self.r#north == false { return 7813; }
        if self.r#north == false && self.r#south == true && self.r#west == false && self.r#east == false && self.r#waterlogged == true { return 7814; }
        if self.r#south == true && self.r#waterlogged == false && self.r#north == false && self.r#east == true && self.r#west == true { return 7799; }
        if self.r#east == true && self.r#south == false && self.r#north == true && self.r#waterlogged == true && self.r#west == true { return 7793; }
        if self.r#south == true && self.r#waterlogged == false && self.r#east == true && self.r#north == true && self.r#west == true { return 7791; }
        if self.r#north == true && self.r#waterlogged == true && self.r#west == false && self.r#south == true && self.r#east == true { return 7790; }
        if self.r#north == true && self.r#waterlogged == true && self.r#south == true && self.r#east == false && self.r#west == true { return 7805; }
        if self.r#west == true && self.r#north == true && self.r#east == false && self.r#south == true && self.r#waterlogged == false { return 7807; }
        if self.r#north == true && self.r#east == false && self.r#waterlogged == true && self.r#west == false && self.r#south == false { return 7810; }
        if self.r#north == false && self.r#waterlogged == false && self.r#south == false && self.r#east == true && self.r#west == true { return 7803; }
        if self.r#south == true && self.r#north == true && self.r#waterlogged == false && self.r#east == false && self.r#west == false { return 7808; }
        if self.r#east == false && self.r#waterlogged == false && self.r#west == true && self.r#south == true && self.r#north == false { return 7815; }
        if self.r#south == false && self.r#waterlogged == false && self.r#east == true && self.r#west == false && self.r#north == false { return 7804; }
        if self.r#west == false && self.r#waterlogged == false && self.r#south == true && self.r#east == true && self.r#north == true { return 7792; }
        if self.r#north == true && self.r#waterlogged == true && self.r#west == false && self.r#east == true && self.r#south == false { return 7794; }
        if self.r#east == true && self.r#south == false && self.r#west == false && self.r#waterlogged == false && self.r#north == true { return 7796; }
        if self.r#waterlogged == false && self.r#north == false && self.r#west == false && self.r#east == false && self.r#south == true { return 7816; }
        if self.r#south == false && self.r#east == false && self.r#waterlogged == false && self.r#west == false && self.r#north == false { return 7820; }
        if self.r#south == true && self.r#east == true && self.r#north == true && self.r#waterlogged == true && self.r#west == true { return 7789; }
        if self.r#north == false && self.r#south == false && self.r#west == true && self.r#east == false && self.r#waterlogged == true { return 7817; }
        if self.r#east == true && self.r#waterlogged == true && self.r#south == true && self.r#west == true && self.r#north == false { return 7797; }
        if self.r#east == false && self.r#waterlogged == true && self.r#west == false && self.r#north == false && self.r#south == false { return 7818; }
        if self.r#north == false && self.r#waterlogged == true && self.r#east == true && self.r#south == true && self.r#west == false { return 7798; }
        if self.r#north == false && self.r#east == true && self.r#waterlogged == true && self.r#west == false && self.r#south == false { return 7802; }
        if self.r#waterlogged == false && self.r#north == false && self.r#west == true && self.r#east == false && self.r#south == false { return 7819; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7801 {
            return Some(CopperBars {
                r#east: true,
                r#north: false,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 7809 {
            return Some(CopperBars {
                r#south: false,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 7795 {
            return Some(CopperBars {
                r#west: true,
                r#north: true,
                r#waterlogged: false,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 7806 {
            return Some(CopperBars {
                r#west: false,
                r#waterlogged: true,
                r#south: true,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 7811 {
            return Some(CopperBars {
                r#east: false,
                r#waterlogged: false,
                r#west: true,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 7800 {
            return Some(CopperBars {
                r#east: true,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 7812 {
            return Some(CopperBars {
                r#north: true,
                r#south: false,
                r#waterlogged: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 7813 {
            return Some(CopperBars {
                r#west: true,
                r#south: true,
                r#east: false,
                r#waterlogged: true,
                r#north: false,
            });
        }
        if state_id == 7814 {
            return Some(CopperBars {
                r#north: false,
                r#south: true,
                r#west: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7799 {
            return Some(CopperBars {
                r#south: true,
                r#waterlogged: false,
                r#north: false,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 7793 {
            return Some(CopperBars {
                r#east: true,
                r#south: false,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 7791 {
            return Some(CopperBars {
                r#south: true,
                r#waterlogged: false,
                r#east: true,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 7790 {
            return Some(CopperBars {
                r#north: true,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 7805 {
            return Some(CopperBars {
                r#north: true,
                r#waterlogged: true,
                r#south: true,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 7807 {
            return Some(CopperBars {
                r#west: true,
                r#north: true,
                r#east: false,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7810 {
            return Some(CopperBars {
                r#north: true,
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 7803 {
            return Some(CopperBars {
                r#north: false,
                r#waterlogged: false,
                r#south: false,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 7808 {
            return Some(CopperBars {
                r#south: true,
                r#north: true,
                r#waterlogged: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 7815 {
            return Some(CopperBars {
                r#east: false,
                r#waterlogged: false,
                r#west: true,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 7804 {
            return Some(CopperBars {
                r#south: false,
                r#waterlogged: false,
                r#east: true,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 7792 {
            return Some(CopperBars {
                r#west: false,
                r#waterlogged: false,
                r#south: true,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 7794 {
            return Some(CopperBars {
                r#north: true,
                r#waterlogged: true,
                r#west: false,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 7796 {
            return Some(CopperBars {
                r#east: true,
                r#south: false,
                r#west: false,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 7816 {
            return Some(CopperBars {
                r#waterlogged: false,
                r#north: false,
                r#west: false,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 7820 {
            return Some(CopperBars {
                r#south: false,
                r#east: false,
                r#waterlogged: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 7789 {
            return Some(CopperBars {
                r#south: true,
                r#east: true,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 7817 {
            return Some(CopperBars {
                r#north: false,
                r#south: false,
                r#west: true,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7797 {
            return Some(CopperBars {
                r#east: true,
                r#waterlogged: true,
                r#south: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 7818 {
            return Some(CopperBars {
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 7798 {
            return Some(CopperBars {
                r#north: false,
                r#waterlogged: true,
                r#east: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 7802 {
            return Some(CopperBars {
                r#north: false,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 7819 {
            return Some(CopperBars {
                r#waterlogged: false,
                r#north: false,
                r#west: true,
                r#east: false,
                r#south: false,
            });
        }
        return None;
    }
}

