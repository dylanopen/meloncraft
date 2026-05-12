use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedFence {
    pub west: bool,
    pub waterlogged: bool,
    pub east: bool,
    pub south: bool,
    pub north: bool,
}


impl BlockState for WarpedFence {
    fn to_id(&self) -> i32 {
        if self.r#south == false && self.r#west == true && self.r#east == true && self.r#waterlogged == false && self.r#north == false { return 20894; }
        if self.r#north == true && self.r#waterlogged == false && self.r#east == true && self.r#south == true && self.r#west == true { return 20882; }
        if self.r#waterlogged == true && self.r#west == false && self.r#north == true && self.r#south == false && self.r#east == true { return 20885; }
        if self.r#waterlogged == true && self.r#north == true && self.r#west == false && self.r#east == false && self.r#south == false { return 20901; }
        if self.r#south == false && self.r#waterlogged == true && self.r#north == true && self.r#east == false && self.r#west == true { return 20900; }
        if self.r#north == false && self.r#east == true && self.r#waterlogged == false && self.r#south == false && self.r#west == false { return 20895; }
        if self.r#waterlogged == true && self.r#west == true && self.r#north == true && self.r#east == true && self.r#south == false { return 20884; }
        if self.r#north == true && self.r#waterlogged == true && self.r#west == false && self.r#south == true && self.r#east == true { return 20881; }
        if self.r#west == false && self.r#south == true && self.r#waterlogged == false && self.r#north == true && self.r#east == true { return 20883; }
        if self.r#north == true && self.r#south == false && self.r#waterlogged == false && self.r#west == true && self.r#east == true { return 20886; }
        if self.r#north == false && self.r#south == true && self.r#west == true && self.r#east == true && self.r#waterlogged == false { return 20890; }
        if self.r#east == false && self.r#south == true && self.r#waterlogged == true && self.r#west == true && self.r#north == false { return 20904; }
        if self.r#waterlogged == true && self.r#east == true && self.r#west == true && self.r#south == false && self.r#north == false { return 20892; }
        if self.r#north == false && self.r#waterlogged == true && self.r#west == true && self.r#east == false && self.r#south == false { return 20908; }
        if self.r#waterlogged == false && self.r#south == false && self.r#west == false && self.r#north == true && self.r#east == false { return 20903; }
        if self.r#west == true && self.r#north == false && self.r#south == true && self.r#waterlogged == true && self.r#east == true { return 20888; }
        if self.r#north == false && self.r#south == true && self.r#east == false && self.r#waterlogged == false && self.r#west == true { return 20906; }
        if self.r#east == false && self.r#north == false && self.r#west == true && self.r#south == false && self.r#waterlogged == false { return 20910; }
        if self.r#south == false && self.r#west == false && self.r#north == false && self.r#waterlogged == false && self.r#east == false { return 20911; }
        if self.r#waterlogged == false && self.r#east == false && self.r#south == true && self.r#west == false && self.r#north == true { return 20899; }
        if self.r#north == false && self.r#west == false && self.r#south == true && self.r#east == false && self.r#waterlogged == true { return 20905; }
        if self.r#north == false && self.r#west == false && self.r#east == true && self.r#south == true && self.r#waterlogged == true { return 20889; }
        if self.r#east == true && self.r#south == true && self.r#north == true && self.r#west == true && self.r#waterlogged == true { return 20880; }
        if self.r#south == false && self.r#west == false && self.r#north == true && self.r#east == true && self.r#waterlogged == false { return 20887; }
        if self.r#north == false && self.r#south == true && self.r#west == false && self.r#east == true && self.r#waterlogged == false { return 20891; }
        if self.r#west == false && self.r#north == false && self.r#east == false && self.r#south == false && self.r#waterlogged == true { return 20909; }
        if self.r#waterlogged == false && self.r#south == true && self.r#west == true && self.r#east == false && self.r#north == true { return 20898; }
        if self.r#waterlogged == false && self.r#north == true && self.r#west == true && self.r#south == false && self.r#east == false { return 20902; }
        if self.r#waterlogged == true && self.r#south == false && self.r#west == false && self.r#north == false && self.r#east == true { return 20893; }
        if self.r#south == true && self.r#west == false && self.r#north == false && self.r#waterlogged == false && self.r#east == false { return 20907; }
        if self.r#north == true && self.r#west == true && self.r#east == false && self.r#south == true && self.r#waterlogged == true { return 20896; }
        if self.r#waterlogged == true && self.r#south == true && self.r#east == false && self.r#north == true && self.r#west == false { return 20897; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20894 {
            return Some(WarpedFence {
                r#south: false,
                r#west: true,
                r#east: true,
                r#waterlogged: false,
                r#north: false,
            });
        }
        if state_id == 20882 {
            return Some(WarpedFence {
                r#north: true,
                r#waterlogged: false,
                r#east: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 20885 {
            return Some(WarpedFence {
                r#waterlogged: true,
                r#west: false,
                r#north: true,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 20901 {
            return Some(WarpedFence {
                r#waterlogged: true,
                r#north: true,
                r#west: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 20900 {
            return Some(WarpedFence {
                r#south: false,
                r#waterlogged: true,
                r#north: true,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 20895 {
            return Some(WarpedFence {
                r#north: false,
                r#east: true,
                r#waterlogged: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 20884 {
            return Some(WarpedFence {
                r#waterlogged: true,
                r#west: true,
                r#north: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 20881 {
            return Some(WarpedFence {
                r#north: true,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 20883 {
            return Some(WarpedFence {
                r#west: false,
                r#south: true,
                r#waterlogged: false,
                r#north: true,
                r#east: true,
            });
        }
        if state_id == 20886 {
            return Some(WarpedFence {
                r#north: true,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 20890 {
            return Some(WarpedFence {
                r#north: false,
                r#south: true,
                r#west: true,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20904 {
            return Some(WarpedFence {
                r#east: false,
                r#south: true,
                r#waterlogged: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 20892 {
            return Some(WarpedFence {
                r#waterlogged: true,
                r#east: true,
                r#west: true,
                r#south: false,
                r#north: false,
            });
        }
        if state_id == 20908 {
            return Some(WarpedFence {
                r#north: false,
                r#waterlogged: true,
                r#west: true,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 20903 {
            return Some(WarpedFence {
                r#waterlogged: false,
                r#south: false,
                r#west: false,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 20888 {
            return Some(WarpedFence {
                r#west: true,
                r#north: false,
                r#south: true,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 20906 {
            return Some(WarpedFence {
                r#north: false,
                r#south: true,
                r#east: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 20910 {
            return Some(WarpedFence {
                r#east: false,
                r#north: false,
                r#west: true,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20911 {
            return Some(WarpedFence {
                r#south: false,
                r#west: false,
                r#north: false,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 20899 {
            return Some(WarpedFence {
                r#waterlogged: false,
                r#east: false,
                r#south: true,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 20905 {
            return Some(WarpedFence {
                r#north: false,
                r#west: false,
                r#south: true,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20889 {
            return Some(WarpedFence {
                r#north: false,
                r#west: false,
                r#east: true,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20880 {
            return Some(WarpedFence {
                r#east: true,
                r#south: true,
                r#north: true,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20887 {
            return Some(WarpedFence {
                r#south: false,
                r#west: false,
                r#north: true,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20891 {
            return Some(WarpedFence {
                r#north: false,
                r#south: true,
                r#west: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20909 {
            return Some(WarpedFence {
                r#west: false,
                r#north: false,
                r#east: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20898 {
            return Some(WarpedFence {
                r#waterlogged: false,
                r#south: true,
                r#west: true,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 20902 {
            return Some(WarpedFence {
                r#waterlogged: false,
                r#north: true,
                r#west: true,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 20893 {
            return Some(WarpedFence {
                r#waterlogged: true,
                r#south: false,
                r#west: false,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 20907 {
            return Some(WarpedFence {
                r#south: true,
                r#west: false,
                r#north: false,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 20896 {
            return Some(WarpedFence {
                r#north: true,
                r#west: true,
                r#east: false,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20897 {
            return Some(WarpedFence {
                r#waterlogged: true,
                r#south: true,
                r#east: false,
                r#north: true,
                r#west: false,
            });
        }
        return None;
    }
}

