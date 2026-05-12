use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryFence {
    pub west: bool,
    pub waterlogged: bool,
    pub north: bool,
    pub south: bool,
    pub east: bool,
}


impl BlockState for CherryFence {
    fn to_id(&self) -> i32 {
        if self.r#north == true && self.r#waterlogged == true && self.r#east == true && self.r#west == false && self.r#south == true { return 13699; }
        if self.r#north == false && self.r#east == true && self.r#waterlogged == true && self.r#west == false && self.r#south == false { return 13711; }
        if self.r#east == true && self.r#north == false && self.r#waterlogged == false && self.r#west == false && self.r#south == true { return 13709; }
        if self.r#north == true && self.r#south == true && self.r#waterlogged == false && self.r#east == false && self.r#west == false { return 13717; }
        if self.r#west == true && self.r#waterlogged == false && self.r#south == false && self.r#north == true && self.r#east == true { return 13704; }
        if self.r#south == true && self.r#waterlogged == true && self.r#east == true && self.r#north == false && self.r#west == true { return 13706; }
        if self.r#west == false && self.r#waterlogged == true && self.r#east == false && self.r#north == false && self.r#south == true { return 13723; }
        if self.r#north == true && self.r#waterlogged == false && self.r#east == true && self.r#west == false && self.r#south == true { return 13701; }
        if self.r#east == false && self.r#waterlogged == true && self.r#north == true && self.r#south == true && self.r#west == false { return 13715; }
        if self.r#east == true && self.r#south == false && self.r#west == true && self.r#north == false && self.r#waterlogged == true { return 13710; }
        if self.r#north == true && self.r#south == true && self.r#west == true && self.r#east == false && self.r#waterlogged == true { return 13714; }
        if self.r#south == true && self.r#waterlogged == false && self.r#east == true && self.r#north == true && self.r#west == true { return 13700; }
        if self.r#waterlogged == false && self.r#west == true && self.r#north == true && self.r#east == false && self.r#south == true { return 13716; }
        if self.r#south == false && self.r#north == false && self.r#east == false && self.r#west == false && self.r#waterlogged == false { return 13729; }
        if self.r#east == true && self.r#waterlogged == false && self.r#south == false && self.r#west == true && self.r#north == false { return 13712; }
        if self.r#north == false && self.r#waterlogged == false && self.r#west == true && self.r#east == true && self.r#south == true { return 13708; }
        if self.r#east == false && self.r#west == false && self.r#north == true && self.r#waterlogged == true && self.r#south == false { return 13719; }
        if self.r#east == true && self.r#south == false && self.r#north == true && self.r#waterlogged == true && self.r#west == false { return 13703; }
        if self.r#waterlogged == false && self.r#west == true && self.r#east == false && self.r#south == true && self.r#north == false { return 13724; }
        if self.r#waterlogged == false && self.r#west == false && self.r#north == true && self.r#south == false && self.r#east == true { return 13705; }
        if self.r#east == false && self.r#west == false && self.r#north == false && self.r#south == false && self.r#waterlogged == true { return 13727; }
        if self.r#north == true && self.r#waterlogged == true && self.r#east == true && self.r#south == false && self.r#west == true { return 13702; }
        if self.r#waterlogged == false && self.r#east == false && self.r#south == false && self.r#north == false && self.r#west == true { return 13728; }
        if self.r#north == true && self.r#east == true && self.r#south == true && self.r#waterlogged == true && self.r#west == true { return 13698; }
        if self.r#west == false && self.r#east == true && self.r#south == false && self.r#north == false && self.r#waterlogged == false { return 13713; }
        if self.r#waterlogged == true && self.r#south == true && self.r#north == false && self.r#east == true && self.r#west == false { return 13707; }
        if self.r#south == false && self.r#east == false && self.r#north == true && self.r#waterlogged == false && self.r#west == false { return 13721; }
        if self.r#north == false && self.r#waterlogged == true && self.r#west == true && self.r#east == false && self.r#south == true { return 13722; }
        if self.r#south == false && self.r#waterlogged == false && self.r#west == true && self.r#east == false && self.r#north == true { return 13720; }
        if self.r#west == true && self.r#south == false && self.r#east == false && self.r#north == true && self.r#waterlogged == true { return 13718; }
        if self.r#east == false && self.r#north == false && self.r#waterlogged == false && self.r#south == true && self.r#west == false { return 13725; }
        if self.r#east == false && self.r#north == false && self.r#west == true && self.r#waterlogged == true && self.r#south == false { return 13726; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13699 {
            return Some(CherryFence {
                r#north: true,
                r#waterlogged: true,
                r#east: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 13711 {
            return Some(CherryFence {
                r#north: false,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 13709 {
            return Some(CherryFence {
                r#east: true,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 13717 {
            return Some(CherryFence {
                r#north: true,
                r#south: true,
                r#waterlogged: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 13704 {
            return Some(CherryFence {
                r#west: true,
                r#waterlogged: false,
                r#south: false,
                r#north: true,
                r#east: true,
            });
        }
        if state_id == 13706 {
            return Some(CherryFence {
                r#south: true,
                r#waterlogged: true,
                r#east: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 13723 {
            return Some(CherryFence {
                r#west: false,
                r#waterlogged: true,
                r#east: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 13701 {
            return Some(CherryFence {
                r#north: true,
                r#waterlogged: false,
                r#east: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 13715 {
            return Some(CherryFence {
                r#east: false,
                r#waterlogged: true,
                r#north: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 13710 {
            return Some(CherryFence {
                r#east: true,
                r#south: false,
                r#west: true,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13714 {
            return Some(CherryFence {
                r#north: true,
                r#south: true,
                r#west: true,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13700 {
            return Some(CherryFence {
                r#south: true,
                r#waterlogged: false,
                r#east: true,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 13716 {
            return Some(CherryFence {
                r#waterlogged: false,
                r#west: true,
                r#north: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 13729 {
            return Some(CherryFence {
                r#south: false,
                r#north: false,
                r#east: false,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13712 {
            return Some(CherryFence {
                r#east: true,
                r#waterlogged: false,
                r#south: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 13708 {
            return Some(CherryFence {
                r#north: false,
                r#waterlogged: false,
                r#west: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 13719 {
            return Some(CherryFence {
                r#east: false,
                r#west: false,
                r#north: true,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 13703 {
            return Some(CherryFence {
                r#east: true,
                r#south: false,
                r#north: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 13724 {
            return Some(CherryFence {
                r#waterlogged: false,
                r#west: true,
                r#east: false,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 13705 {
            return Some(CherryFence {
                r#waterlogged: false,
                r#west: false,
                r#north: true,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 13727 {
            return Some(CherryFence {
                r#east: false,
                r#west: false,
                r#north: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13702 {
            return Some(CherryFence {
                r#north: true,
                r#waterlogged: true,
                r#east: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 13728 {
            return Some(CherryFence {
                r#waterlogged: false,
                r#east: false,
                r#south: false,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 13698 {
            return Some(CherryFence {
                r#north: true,
                r#east: true,
                r#south: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 13713 {
            return Some(CherryFence {
                r#west: false,
                r#east: true,
                r#south: false,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13707 {
            return Some(CherryFence {
                r#waterlogged: true,
                r#south: true,
                r#north: false,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 13721 {
            return Some(CherryFence {
                r#south: false,
                r#east: false,
                r#north: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 13722 {
            return Some(CherryFence {
                r#north: false,
                r#waterlogged: true,
                r#west: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 13720 {
            return Some(CherryFence {
                r#south: false,
                r#waterlogged: false,
                r#west: true,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 13718 {
            return Some(CherryFence {
                r#west: true,
                r#south: false,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13725 {
            return Some(CherryFence {
                r#east: false,
                r#north: false,
                r#waterlogged: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 13726 {
            return Some(CherryFence {
                r#east: false,
                r#north: false,
                r#west: true,
                r#waterlogged: true,
                r#south: false,
            });
        }
        return None;
    }
}

