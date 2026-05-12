use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaFence {
    pub west: bool,
    pub north: bool,
    pub east: bool,
    pub south: bool,
    pub waterlogged: bool,
}


impl BlockState for AcaciaFence {
    fn to_id(&self) -> i32 {
        if self.r#east == true && self.r#south == false && self.r#west == true && self.r#north == false && self.r#waterlogged == false { return 13680; }
        if self.r#east == false && self.r#south == false && self.r#north == false && self.r#waterlogged == false && self.r#west == true { return 13696; }
        if self.r#waterlogged == true && self.r#north == false && self.r#west == true && self.r#east == false && self.r#south == true { return 13690; }
        if self.r#south == true && self.r#east == false && self.r#north == true && self.r#waterlogged == true && self.r#west == false { return 13683; }
        if self.r#south == false && self.r#west == true && self.r#east == false && self.r#north == false && self.r#waterlogged == true { return 13694; }
        if self.r#north == false && self.r#waterlogged == false && self.r#south == true && self.r#west == true && self.r#east == true { return 13676; }
        if self.r#south == false && self.r#waterlogged == true && self.r#east == false && self.r#west == true && self.r#north == true { return 13686; }
        if self.r#west == false && self.r#east == false && self.r#south == true && self.r#north == true && self.r#waterlogged == false { return 13685; }
        if self.r#east == false && self.r#waterlogged == false && self.r#west == false && self.r#south == false && self.r#north == true { return 13689; }
        if self.r#north == true && self.r#south == true && self.r#east == true && self.r#waterlogged == false && self.r#west == true { return 13668; }
        if self.r#south == false && self.r#waterlogged == true && self.r#east == true && self.r#west == true && self.r#north == true { return 13670; }
        if self.r#east == true && self.r#south == false && self.r#west == false && self.r#north == false && self.r#waterlogged == true { return 13679; }
        if self.r#north == true && self.r#south == false && self.r#waterlogged == false && self.r#east == false && self.r#west == true { return 13688; }
        if self.r#east == false && self.r#south == true && self.r#waterlogged == true && self.r#west == true && self.r#north == true { return 13682; }
        if self.r#north == false && self.r#east == false && self.r#waterlogged == false && self.r#west == false && self.r#south == true { return 13693; }
        if self.r#east == true && self.r#north == false && self.r#waterlogged == true && self.r#west == true && self.r#south == true { return 13674; }
        if self.r#north == false && self.r#west == false && self.r#east == false && self.r#south == false && self.r#waterlogged == true { return 13695; }
        if self.r#west == true && self.r#east == true && self.r#north == false && self.r#south == false && self.r#waterlogged == true { return 13678; }
        if self.r#east == true && self.r#south == true && self.r#north == true && self.r#waterlogged == false && self.r#west == false { return 13669; }
        if self.r#east == true && self.r#south == false && self.r#north == true && self.r#waterlogged == false && self.r#west == false { return 13673; }
        if self.r#east == true && self.r#waterlogged == false && self.r#west == false && self.r#north == false && self.r#south == false { return 13681; }
        if self.r#east == false && self.r#north == true && self.r#west == false && self.r#waterlogged == true && self.r#south == false { return 13687; }
        if self.r#west == false && self.r#north == false && self.r#east == false && self.r#south == false && self.r#waterlogged == false { return 13697; }
        if self.r#north == false && self.r#east == true && self.r#waterlogged == true && self.r#south == true && self.r#west == false { return 13675; }
        if self.r#north == false && self.r#south == true && self.r#waterlogged == false && self.r#east == true && self.r#west == false { return 13677; }
        if self.r#north == true && self.r#waterlogged == false && self.r#west == true && self.r#south == true && self.r#east == false { return 13684; }
        if self.r#west == false && self.r#waterlogged == true && self.r#north == true && self.r#east == true && self.r#south == true { return 13667; }
        if self.r#north == true && self.r#east == true && self.r#waterlogged == true && self.r#south == true && self.r#west == true { return 13666; }
        if self.r#waterlogged == false && self.r#west == true && self.r#south == true && self.r#east == false && self.r#north == false { return 13692; }
        if self.r#south == false && self.r#east == true && self.r#west == true && self.r#north == true && self.r#waterlogged == false { return 13672; }
        if self.r#south == true && self.r#east == false && self.r#north == false && self.r#waterlogged == true && self.r#west == false { return 13691; }
        if self.r#south == false && self.r#west == false && self.r#north == true && self.r#east == true && self.r#waterlogged == true { return 13671; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13680 {
            return Some(AcaciaFence {
                r#east: true,
                r#south: false,
                r#west: true,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13696 {
            return Some(AcaciaFence {
                r#east: false,
                r#south: false,
                r#north: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 13690 {
            return Some(AcaciaFence {
                r#waterlogged: true,
                r#north: false,
                r#west: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 13683 {
            return Some(AcaciaFence {
                r#south: true,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 13694 {
            return Some(AcaciaFence {
                r#south: false,
                r#west: true,
                r#east: false,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13676 {
            return Some(AcaciaFence {
                r#north: false,
                r#waterlogged: false,
                r#south: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 13686 {
            return Some(AcaciaFence {
                r#south: false,
                r#waterlogged: true,
                r#east: false,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 13685 {
            return Some(AcaciaFence {
                r#west: false,
                r#east: false,
                r#south: true,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13689 {
            return Some(AcaciaFence {
                r#east: false,
                r#waterlogged: false,
                r#west: false,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 13668 {
            return Some(AcaciaFence {
                r#north: true,
                r#south: true,
                r#east: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 13670 {
            return Some(AcaciaFence {
                r#south: false,
                r#waterlogged: true,
                r#east: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 13679 {
            return Some(AcaciaFence {
                r#east: true,
                r#south: false,
                r#west: false,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13688 {
            return Some(AcaciaFence {
                r#north: true,
                r#south: false,
                r#waterlogged: false,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 13682 {
            return Some(AcaciaFence {
                r#east: false,
                r#south: true,
                r#waterlogged: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 13693 {
            return Some(AcaciaFence {
                r#north: false,
                r#east: false,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 13674 {
            return Some(AcaciaFence {
                r#east: true,
                r#north: false,
                r#waterlogged: true,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 13695 {
            return Some(AcaciaFence {
                r#north: false,
                r#west: false,
                r#east: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13678 {
            return Some(AcaciaFence {
                r#west: true,
                r#east: true,
                r#north: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13669 {
            return Some(AcaciaFence {
                r#east: true,
                r#south: true,
                r#north: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 13673 {
            return Some(AcaciaFence {
                r#east: true,
                r#south: false,
                r#north: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 13681 {
            return Some(AcaciaFence {
                r#east: true,
                r#waterlogged: false,
                r#west: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 13687 {
            return Some(AcaciaFence {
                r#east: false,
                r#north: true,
                r#west: false,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 13697 {
            return Some(AcaciaFence {
                r#west: false,
                r#north: false,
                r#east: false,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13675 {
            return Some(AcaciaFence {
                r#north: false,
                r#east: true,
                r#waterlogged: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 13677 {
            return Some(AcaciaFence {
                r#north: false,
                r#south: true,
                r#waterlogged: false,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 13684 {
            return Some(AcaciaFence {
                r#north: true,
                r#waterlogged: false,
                r#west: true,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 13667 {
            return Some(AcaciaFence {
                r#west: false,
                r#waterlogged: true,
                r#north: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 13666 {
            return Some(AcaciaFence {
                r#north: true,
                r#east: true,
                r#waterlogged: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 13692 {
            return Some(AcaciaFence {
                r#waterlogged: false,
                r#west: true,
                r#south: true,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 13672 {
            return Some(AcaciaFence {
                r#south: false,
                r#east: true,
                r#west: true,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13691 {
            return Some(AcaciaFence {
                r#south: true,
                r#east: false,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 13671 {
            return Some(AcaciaFence {
                r#south: false,
                r#west: false,
                r#north: true,
                r#east: true,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

