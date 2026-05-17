use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonFence {
    pub south: bool,
    pub waterlogged: bool,
    pub east: bool,
    pub west: bool,
    pub north: bool,
}


impl BlockState for CrimsonFence {
    fn to_id(&self) -> i32 {
        if self.r#east == true && self.r#south == false && self.r#north == true && self.r#west == true && self.r#waterlogged == true { return 20852; }
        if self.r#east == true && self.r#north == true && self.r#south == false && self.r#west == true && self.r#waterlogged == false { return 20854; }
        if self.r#waterlogged == false && self.r#north == true && self.r#west == true && self.r#south == false && self.r#east == false { return 20870; }
        if self.r#east == false && self.r#north == false && self.r#waterlogged == true && self.r#south == false && self.r#west == false { return 20877; }
        if self.r#west == true && self.r#east == false && self.r#north == false && self.r#waterlogged == false && self.r#south == true { return 20874; }
        if self.r#north == true && self.r#west == false && self.r#east == true && self.r#waterlogged == true && self.r#south == false { return 20853; }
        if self.r#waterlogged == false && self.r#north == false && self.r#south == false && self.r#west == false && self.r#east == false { return 20879; }
        if self.r#south == true && self.r#east == false && self.r#waterlogged == true && self.r#west == true && self.r#north == false { return 20872; }
        if self.r#south == true && self.r#east == true && self.r#north == false && self.r#waterlogged == true && self.r#west == true { return 20856; }
        if self.r#west == true && self.r#north == true && self.r#south == true && self.r#east == false && self.r#waterlogged == true { return 20864; }
        if self.r#west == false && self.r#north == false && self.r#waterlogged == true && self.r#east == false && self.r#south == true { return 20873; }
        if self.r#south == false && self.r#north == false && self.r#waterlogged == true && self.r#east == false && self.r#west == true { return 20876; }
        if self.r#west == false && self.r#south == true && self.r#waterlogged == false && self.r#north == true && self.r#east == true { return 20851; }
        if self.r#east == true && self.r#waterlogged == false && self.r#west == false && self.r#north == false && self.r#south == false { return 20863; }
        if self.r#east == false && self.r#waterlogged == false && self.r#west == false && self.r#south == true && self.r#north == true { return 20867; }
        if self.r#north == true && self.r#west == true && self.r#waterlogged == true && self.r#east == true && self.r#south == true { return 20848; }
        if self.r#north == true && self.r#west == false && self.r#waterlogged == false && self.r#east == true && self.r#south == false { return 20855; }
        if self.r#north == false && self.r#west == false && self.r#east == true && self.r#waterlogged == false && self.r#south == true { return 20859; }
        if self.r#east == false && self.r#west == true && self.r#waterlogged == false && self.r#north == true && self.r#south == true { return 20866; }
        if self.r#east == true && self.r#north == true && self.r#south == true && self.r#west == false && self.r#waterlogged == true { return 20849; }
        if self.r#south == false && self.r#west == false && self.r#north == false && self.r#waterlogged == true && self.r#east == true { return 20861; }
        if self.r#east == true && self.r#north == false && self.r#waterlogged == true && self.r#west == false && self.r#south == true { return 20857; }
        if self.r#east == true && self.r#south == true && self.r#waterlogged == false && self.r#west == true && self.r#north == true { return 20850; }
        if self.r#east == false && self.r#north == true && self.r#south == false && self.r#waterlogged == true && self.r#west == true { return 20868; }
        if self.r#north == false && self.r#west == true && self.r#south == false && self.r#east == true && self.r#waterlogged == true { return 20860; }
        if self.r#waterlogged == true && self.r#west == false && self.r#east == false && self.r#north == true && self.r#south == false { return 20869; }
        if self.r#waterlogged == false && self.r#west == true && self.r#north == false && self.r#south == true && self.r#east == true { return 20858; }
        if self.r#waterlogged == false && self.r#west == false && self.r#east == false && self.r#south == false && self.r#north == true { return 20871; }
        if self.r#waterlogged == false && self.r#north == false && self.r#west == true && self.r#east == false && self.r#south == false { return 20878; }
        if self.r#east == true && self.r#north == false && self.r#waterlogged == false && self.r#west == true && self.r#south == false { return 20862; }
        if self.r#waterlogged == true && self.r#west == false && self.r#south == true && self.r#east == false && self.r#north == true { return 20865; }
        if self.r#waterlogged == false && self.r#south == true && self.r#west == false && self.r#east == false && self.r#north == false { return 20875; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20852 {
            return Some(CrimsonFence {
                r#east: true,
                r#south: false,
                r#north: true,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20854 {
            return Some(CrimsonFence {
                r#east: true,
                r#north: true,
                r#south: false,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20870 {
            return Some(CrimsonFence {
                r#waterlogged: false,
                r#north: true,
                r#west: true,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 20877 {
            return Some(CrimsonFence {
                r#east: false,
                r#north: false,
                r#waterlogged: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 20874 {
            return Some(CrimsonFence {
                r#west: true,
                r#east: false,
                r#north: false,
                r#waterlogged: false,
                r#south: true,
            });
        }
        if state_id == 20853 {
            return Some(CrimsonFence {
                r#north: true,
                r#west: false,
                r#east: true,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 20879 {
            return Some(CrimsonFence {
                r#waterlogged: false,
                r#north: false,
                r#south: false,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 20872 {
            return Some(CrimsonFence {
                r#south: true,
                r#east: false,
                r#waterlogged: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 20856 {
            return Some(CrimsonFence {
                r#south: true,
                r#east: true,
                r#north: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 20864 {
            return Some(CrimsonFence {
                r#west: true,
                r#north: true,
                r#south: true,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20873 {
            return Some(CrimsonFence {
                r#west: false,
                r#north: false,
                r#waterlogged: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 20876 {
            return Some(CrimsonFence {
                r#south: false,
                r#north: false,
                r#waterlogged: true,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 20851 {
            return Some(CrimsonFence {
                r#west: false,
                r#south: true,
                r#waterlogged: false,
                r#north: true,
                r#east: true,
            });
        }
        if state_id == 20863 {
            return Some(CrimsonFence {
                r#east: true,
                r#waterlogged: false,
                r#west: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 20867 {
            return Some(CrimsonFence {
                r#east: false,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 20848 {
            return Some(CrimsonFence {
                r#north: true,
                r#west: true,
                r#waterlogged: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 20855 {
            return Some(CrimsonFence {
                r#north: true,
                r#west: false,
                r#waterlogged: false,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 20859 {
            return Some(CrimsonFence {
                r#north: false,
                r#west: false,
                r#east: true,
                r#waterlogged: false,
                r#south: true,
            });
        }
        if state_id == 20866 {
            return Some(CrimsonFence {
                r#east: false,
                r#west: true,
                r#waterlogged: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 20849 {
            return Some(CrimsonFence {
                r#east: true,
                r#north: true,
                r#south: true,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20861 {
            return Some(CrimsonFence {
                r#south: false,
                r#west: false,
                r#north: false,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 20857 {
            return Some(CrimsonFence {
                r#east: true,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 20850 {
            return Some(CrimsonFence {
                r#east: true,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 20868 {
            return Some(CrimsonFence {
                r#east: false,
                r#north: true,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 20860 {
            return Some(CrimsonFence {
                r#north: false,
                r#west: true,
                r#south: false,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20869 {
            return Some(CrimsonFence {
                r#waterlogged: true,
                r#west: false,
                r#east: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 20858 {
            return Some(CrimsonFence {
                r#waterlogged: false,
                r#west: true,
                r#north: false,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 20871 {
            return Some(CrimsonFence {
                r#waterlogged: false,
                r#west: false,
                r#east: false,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 20878 {
            return Some(CrimsonFence {
                r#waterlogged: false,
                r#north: false,
                r#west: true,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 20862 {
            return Some(CrimsonFence {
                r#east: true,
                r#north: false,
                r#waterlogged: false,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 20865 {
            return Some(CrimsonFence {
                r#waterlogged: true,
                r#west: false,
                r#south: true,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 20875 {
            return Some(CrimsonFence {
                r#waterlogged: false,
                r#south: true,
                r#west: false,
                r#east: false,
                r#north: false,
            });
        }
        return None;
    }
}

