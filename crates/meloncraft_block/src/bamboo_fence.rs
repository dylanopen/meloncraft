use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooFence {
    pub south: bool,
    pub east: bool,
    pub waterlogged: bool,
    pub west: bool,
    pub north: bool,
}


impl BlockState for BambooFence {
    fn to_id(&self) -> i32 {
        if self.r#south == false && self.r#west == true && self.r#east == false && self.r#north == false && self.r#waterlogged == false { return 13856; }
        if self.r#east == false && self.r#north == false && self.r#south == true && self.r#west == false && self.r#waterlogged == false { return 13853; }
        if self.r#east == true && self.r#south == true && self.r#waterlogged == true && self.r#north == false && self.r#west == true { return 13834; }
        if self.r#waterlogged == false && self.r#south == true && self.r#east == true && self.r#north == true && self.r#west == true { return 13828; }
        if self.r#waterlogged == true && self.r#north == true && self.r#west == true && self.r#east == true && self.r#south == false { return 13830; }
        if self.r#east == true && self.r#south == false && self.r#waterlogged == false && self.r#west == false && self.r#north == false { return 13841; }
        if self.r#north == true && self.r#south == false && self.r#east == false && self.r#waterlogged == false && self.r#west == false { return 13849; }
        if self.r#waterlogged == true && self.r#west == false && self.r#north == true && self.r#south == true && self.r#east == true { return 13827; }
        if self.r#waterlogged == true && self.r#west == true && self.r#east == false && self.r#south == true && self.r#north == true { return 13842; }
        if self.r#north == true && self.r#waterlogged == false && self.r#west == false && self.r#south == true && self.r#east == false { return 13845; }
        if self.r#west == true && self.r#north == false && self.r#east == true && self.r#waterlogged == true && self.r#south == false { return 13838; }
        if self.r#west == false && self.r#waterlogged == true && self.r#east == false && self.r#north == true && self.r#south == false { return 13847; }
        if self.r#waterlogged == false && self.r#north == false && self.r#east == false && self.r#west == false && self.r#south == false { return 13857; }
        if self.r#waterlogged == true && self.r#west == true && self.r#south == true && self.r#north == true && self.r#east == true { return 13826; }
        if self.r#south == false && self.r#north == false && self.r#waterlogged == false && self.r#west == true && self.r#east == true { return 13840; }
        if self.r#north == true && self.r#east == true && self.r#waterlogged == false && self.r#south == true && self.r#west == false { return 13829; }
        if self.r#east == true && self.r#waterlogged == true && self.r#west == false && self.r#north == false && self.r#south == true { return 13835; }
        if self.r#waterlogged == false && self.r#west == true && self.r#south == true && self.r#east == false && self.r#north == false { return 13852; }
        if self.r#south == false && self.r#west == false && self.r#east == true && self.r#waterlogged == true && self.r#north == false { return 13839; }
        if self.r#north == true && self.r#east == true && self.r#waterlogged == false && self.r#west == false && self.r#south == false { return 13833; }
        if self.r#waterlogged == false && self.r#north == false && self.r#east == true && self.r#south == true && self.r#west == true { return 13836; }
        if self.r#west == false && self.r#east == false && self.r#north == false && self.r#south == false && self.r#waterlogged == true { return 13855; }
        if self.r#east == false && self.r#west == true && self.r#waterlogged == false && self.r#south == false && self.r#north == true { return 13848; }
        if self.r#east == false && self.r#west == true && self.r#north == false && self.r#south == true && self.r#waterlogged == true { return 13850; }
        if self.r#south == true && self.r#waterlogged == true && self.r#west == false && self.r#north == false && self.r#east == false { return 13851; }
        if self.r#east == false && self.r#waterlogged == true && self.r#north == true && self.r#west == true && self.r#south == false { return 13846; }
        if self.r#waterlogged == true && self.r#west == false && self.r#north == true && self.r#east == true && self.r#south == false { return 13831; }
        if self.r#south == true && self.r#waterlogged == false && self.r#east == true && self.r#north == false && self.r#west == false { return 13837; }
        if self.r#south == true && self.r#waterlogged == false && self.r#west == true && self.r#east == false && self.r#north == true { return 13844; }
        if self.r#east == false && self.r#waterlogged == true && self.r#west == false && self.r#north == true && self.r#south == true { return 13843; }
        if self.r#east == true && self.r#waterlogged == false && self.r#west == true && self.r#north == true && self.r#south == false { return 13832; }
        if self.r#waterlogged == true && self.r#east == false && self.r#south == false && self.r#north == false && self.r#west == true { return 13854; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13856 {
            return Some(BambooFence {
                r#south: false,
                r#west: true,
                r#east: false,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13853 {
            return Some(BambooFence {
                r#east: false,
                r#north: false,
                r#south: true,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13834 {
            return Some(BambooFence {
                r#east: true,
                r#south: true,
                r#waterlogged: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 13828 {
            return Some(BambooFence {
                r#waterlogged: false,
                r#south: true,
                r#east: true,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 13830 {
            return Some(BambooFence {
                r#waterlogged: true,
                r#north: true,
                r#west: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 13841 {
            return Some(BambooFence {
                r#east: true,
                r#south: false,
                r#waterlogged: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 13849 {
            return Some(BambooFence {
                r#north: true,
                r#south: false,
                r#east: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 13827 {
            return Some(BambooFence {
                r#waterlogged: true,
                r#west: false,
                r#north: true,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 13842 {
            return Some(BambooFence {
                r#waterlogged: true,
                r#west: true,
                r#east: false,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 13845 {
            return Some(BambooFence {
                r#north: true,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 13838 {
            return Some(BambooFence {
                r#west: true,
                r#north: false,
                r#east: true,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 13847 {
            return Some(BambooFence {
                r#west: false,
                r#waterlogged: true,
                r#east: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 13857 {
            return Some(BambooFence {
                r#waterlogged: false,
                r#north: false,
                r#east: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 13826 {
            return Some(BambooFence {
                r#waterlogged: true,
                r#west: true,
                r#south: true,
                r#north: true,
                r#east: true,
            });
        }
        if state_id == 13840 {
            return Some(BambooFence {
                r#south: false,
                r#north: false,
                r#waterlogged: false,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 13829 {
            return Some(BambooFence {
                r#north: true,
                r#east: true,
                r#waterlogged: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 13835 {
            return Some(BambooFence {
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 13852 {
            return Some(BambooFence {
                r#waterlogged: false,
                r#west: true,
                r#south: true,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 13839 {
            return Some(BambooFence {
                r#south: false,
                r#west: false,
                r#east: true,
                r#waterlogged: true,
                r#north: false,
            });
        }
        if state_id == 13833 {
            return Some(BambooFence {
                r#north: true,
                r#east: true,
                r#waterlogged: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 13836 {
            return Some(BambooFence {
                r#waterlogged: false,
                r#north: false,
                r#east: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 13855 {
            return Some(BambooFence {
                r#west: false,
                r#east: false,
                r#north: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13848 {
            return Some(BambooFence {
                r#east: false,
                r#west: true,
                r#waterlogged: false,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 13850 {
            return Some(BambooFence {
                r#east: false,
                r#west: true,
                r#north: false,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13851 {
            return Some(BambooFence {
                r#south: true,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
                r#east: false,
            });
        }
        if state_id == 13846 {
            return Some(BambooFence {
                r#east: false,
                r#waterlogged: true,
                r#north: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 13831 {
            return Some(BambooFence {
                r#waterlogged: true,
                r#west: false,
                r#north: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 13837 {
            return Some(BambooFence {
                r#south: true,
                r#waterlogged: false,
                r#east: true,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 13844 {
            return Some(BambooFence {
                r#south: true,
                r#waterlogged: false,
                r#west: true,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 13843 {
            return Some(BambooFence {
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 13832 {
            return Some(BambooFence {
                r#east: true,
                r#waterlogged: false,
                r#west: true,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 13854 {
            return Some(BambooFence {
                r#waterlogged: true,
                r#east: false,
                r#south: false,
                r#north: false,
                r#west: true,
            });
        }
        return None;
    }
}

