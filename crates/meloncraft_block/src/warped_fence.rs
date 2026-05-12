use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedFence {
    pub south: bool,
    pub waterlogged: bool,
    pub north: bool,
    pub east: bool,
    pub west: bool,
}


impl BlockState for WarpedFence {
    fn to_id(self) -> i32 {
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#north == true && block_state.r#west == false { return 20881; }
        if block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#south == false && block_state.r#west == false { return 20895; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#west == true { return 20890; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#west == false && block_state.r#east == true && block_state.r#waterlogged == false { return 20887; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#east == true { return 20894; }
        if block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == true && block_state.r#east == false { return 20904; }
        if block_state.r#west == true && block_state.r#east == false && block_state.r#south == false && block_state.r#north == false && block_state.r#waterlogged == true { return 20908; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#west == false && block_state.r#north == true && block_state.r#waterlogged == true { return 20897; }
        if block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#west == false && block_state.r#east == true { return 20885; }
        if block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#south == false && block_state.r#east == false && block_state.r#west == true { return 20900; }
        if block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == true && block_state.r#south == true { return 20899; }
        if block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#east == false && block_state.r#south == true { return 20896; }
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#west == false && block_state.r#south == false { return 20893; }
        if block_state.r#west == true && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#south == false { return 20910; }
        if block_state.r#north == false && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#south == true { return 20907; }
        if block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#east == false && block_state.r#north == true { return 20902; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#south == false { return 20884; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#north == true { return 20903; }
        if block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#south == true && block_state.r#west == false { return 20905; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#south == true && block_state.r#west == true && block_state.r#waterlogged == true { return 20888; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#north == false { return 20906; }
        if block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#east == true && block_state.r#north == false && block_state.r#west == false { return 20891; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == true { return 20882; }
        if block_state.r#south == false && block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#north == false { return 20892; }
        if block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#east == false && block_state.r#north == true { return 20898; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#west == false { return 20901; }
        if block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#north == true && block_state.r#east == true && block_state.r#west == true { return 20880; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#west == false { return 20889; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#south == false && block_state.r#north == true { return 20886; }
        if block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#north == false && block_state.r#west == false { return 20911; }
        if block_state.r#north == false && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#east == false { return 20909; }
        if block_state.r#north == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#east == true { return 20883; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20881 {
            return Some(WarpedFence {
                r#east: true,
                r#waterlogged: true,
                r#south: true,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 20895 {
            return Some(WarpedFence {
                r#north: false,
                r#waterlogged: false,
                r#east: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 20890 {
            return Some(WarpedFence {
                r#east: true,
                r#north: false,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 20887 {
            return Some(WarpedFence {
                r#north: true,
                r#south: false,
                r#west: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20894 {
            return Some(WarpedFence {
                r#north: false,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 20904 {
            return Some(WarpedFence {
                r#north: false,
                r#waterlogged: true,
                r#west: true,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 20908 {
            return Some(WarpedFence {
                r#west: true,
                r#east: false,
                r#south: false,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20897 {
            return Some(WarpedFence {
                r#east: false,
                r#south: true,
                r#west: false,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20885 {
            return Some(WarpedFence {
                r#south: false,
                r#waterlogged: true,
                r#north: true,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 20900 {
            return Some(WarpedFence {
                r#waterlogged: true,
                r#north: true,
                r#south: false,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 20899 {
            return Some(WarpedFence {
                r#east: false,
                r#waterlogged: false,
                r#west: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 20896 {
            return Some(WarpedFence {
                r#north: true,
                r#waterlogged: true,
                r#west: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 20893 {
            return Some(WarpedFence {
                r#east: true,
                r#waterlogged: true,
                r#north: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 20910 {
            return Some(WarpedFence {
                r#west: true,
                r#east: false,
                r#waterlogged: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 20907 {
            return Some(WarpedFence {
                r#north: false,
                r#west: false,
                r#waterlogged: false,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 20902 {
            return Some(WarpedFence {
                r#south: false,
                r#waterlogged: false,
                r#west: true,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 20884 {
            return Some(WarpedFence {
                r#west: true,
                r#east: true,
                r#north: true,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 20903 {
            return Some(WarpedFence {
                r#east: false,
                r#south: false,
                r#west: false,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 20905 {
            return Some(WarpedFence {
                r#north: false,
                r#waterlogged: true,
                r#east: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 20888 {
            return Some(WarpedFence {
                r#east: true,
                r#north: false,
                r#south: true,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20906 {
            return Some(WarpedFence {
                r#east: false,
                r#west: true,
                r#south: true,
                r#waterlogged: false,
                r#north: false,
            });
        }
        if state_id == 20891 {
            return Some(WarpedFence {
                r#waterlogged: false,
                r#south: true,
                r#east: true,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 20882 {
            return Some(WarpedFence {
                r#south: true,
                r#north: true,
                r#east: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 20892 {
            return Some(WarpedFence {
                r#south: false,
                r#west: true,
                r#waterlogged: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 20898 {
            return Some(WarpedFence {
                r#west: true,
                r#waterlogged: false,
                r#south: true,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 20901 {
            return Some(WarpedFence {
                r#south: false,
                r#east: false,
                r#waterlogged: true,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 20880 {
            return Some(WarpedFence {
                r#waterlogged: true,
                r#south: true,
                r#north: true,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 20889 {
            return Some(WarpedFence {
                r#north: false,
                r#east: true,
                r#waterlogged: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 20886 {
            return Some(WarpedFence {
                r#east: true,
                r#waterlogged: false,
                r#west: true,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 20911 {
            return Some(WarpedFence {
                r#east: false,
                r#waterlogged: false,
                r#south: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 20909 {
            return Some(WarpedFence {
                r#north: false,
                r#west: false,
                r#waterlogged: true,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 20883 {
            return Some(WarpedFence {
                r#north: true,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
                r#east: true,
            });
        }
        return None;
    }
}

