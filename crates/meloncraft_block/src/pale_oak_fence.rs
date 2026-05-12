use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakFence {
    pub south: bool,
    pub east: bool,
    pub waterlogged: bool,
    pub west: bool,
    pub north: bool,
}


impl BlockState for PaleOakFence {
    fn to_id(self) -> i32 {
        if block_state.r#west == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#north == true { return 13768; }
        if block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#north == true && block_state.r#south == true && block_state.r#west == false { return 13765; }
        if block_state.r#east == false && block_state.r#west == false && block_state.r#south == true && block_state.r#north == false && block_state.r#waterlogged == false { return 13789; }
        if block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#north == true && block_state.r#west == true && block_state.r#south == true { return 13780; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#south == true && block_state.r#west == true && block_state.r#waterlogged == false { return 13764; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#south == true && block_state.r#north == false { return 13773; }
        if block_state.r#south == true && block_state.r#west == true && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#east == true { return 13772; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == true { return 13782; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#west == true { return 13790; }
        if block_state.r#west == false && block_state.r#south == false && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#east == true { return 13767; }
        if block_state.r#south == false && block_state.r#north == false && block_state.r#west == true && block_state.r#east == false && block_state.r#waterlogged == false { return 13792; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == true { return 13762; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#north == false && block_state.r#east == true && block_state.r#waterlogged == true { return 13771; }
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#south == false && block_state.r#west == true { return 13766; }
        if block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#north == false && block_state.r#west == false { return 13793; }
        if block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#west == false && block_state.r#south == false { return 13769; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#west == false && block_state.r#north == true && block_state.r#waterlogged == true { return 13783; }
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#west == false && block_state.r#south == false { return 13775; }
        if block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#west == true && block_state.r#south == false && block_state.r#north == true { return 13784; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == true { return 13785; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#north == true && block_state.r#east == true && block_state.r#waterlogged == true { return 13763; }
        if block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#north == true && block_state.r#east == false { return 13778; }
        if block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#west == true && block_state.r#east == true { return 13776; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == false { return 13777; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == true { return 13787; }
        if block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#east == false && block_state.r#north == true { return 13779; }
        if block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#east == false && block_state.r#west == false && block_state.r#north == false { return 13791; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == true { return 13786; }
        if block_state.r#east == false && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#south == true { return 13781; }
        if block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#west == true && block_state.r#north == false { return 13788; }
        if block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#south == true && block_state.r#west == true { return 13770; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#north == false { return 13774; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13768 {
            return Some(PaleOakFence {
                r#west: true,
                r#south: false,
                r#waterlogged: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 13765 {
            return Some(PaleOakFence {
                r#waterlogged: false,
                r#east: true,
                r#north: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 13789 {
            return Some(PaleOakFence {
                r#east: false,
                r#west: false,
                r#south: true,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13780 {
            return Some(PaleOakFence {
                r#waterlogged: false,
                r#east: false,
                r#north: true,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 13764 {
            return Some(PaleOakFence {
                r#north: true,
                r#east: true,
                r#south: true,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13773 {
            return Some(PaleOakFence {
                r#east: true,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 13772 {
            return Some(PaleOakFence {
                r#south: true,
                r#west: true,
                r#north: false,
                r#waterlogged: false,
                r#east: true,
            });
        }
        if state_id == 13782 {
            return Some(PaleOakFence {
                r#south: false,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 13790 {
            return Some(PaleOakFence {
                r#north: false,
                r#east: false,
                r#waterlogged: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 13767 {
            return Some(PaleOakFence {
                r#west: false,
                r#south: false,
                r#north: true,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 13792 {
            return Some(PaleOakFence {
                r#south: false,
                r#north: false,
                r#west: true,
                r#east: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13762 {
            return Some(PaleOakFence {
                r#south: true,
                r#east: true,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 13771 {
            return Some(PaleOakFence {
                r#west: false,
                r#south: true,
                r#north: false,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13766 {
            return Some(PaleOakFence {
                r#east: true,
                r#waterlogged: true,
                r#north: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 13793 {
            return Some(PaleOakFence {
                r#east: false,
                r#waterlogged: false,
                r#south: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 13769 {
            return Some(PaleOakFence {
                r#north: true,
                r#waterlogged: false,
                r#east: true,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 13783 {
            return Some(PaleOakFence {
                r#east: false,
                r#south: false,
                r#west: false,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13775 {
            return Some(PaleOakFence {
                r#east: true,
                r#waterlogged: true,
                r#north: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 13784 {
            return Some(PaleOakFence {
                r#waterlogged: false,
                r#east: false,
                r#west: true,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 13785 {
            return Some(PaleOakFence {
                r#east: false,
                r#south: false,
                r#waterlogged: false,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 13763 {
            return Some(PaleOakFence {
                r#west: false,
                r#south: true,
                r#north: true,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13778 {
            return Some(PaleOakFence {
                r#south: true,
                r#waterlogged: true,
                r#west: true,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 13776 {
            return Some(PaleOakFence {
                r#north: false,
                r#waterlogged: false,
                r#south: false,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 13777 {
            return Some(PaleOakFence {
                r#east: true,
                r#north: false,
                r#south: false,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13787 {
            return Some(PaleOakFence {
                r#west: false,
                r#south: true,
                r#east: false,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13779 {
            return Some(PaleOakFence {
                r#west: false,
                r#waterlogged: true,
                r#south: true,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 13791 {
            return Some(PaleOakFence {
                r#waterlogged: true,
                r#south: false,
                r#east: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 13786 {
            return Some(PaleOakFence {
                r#north: false,
                r#east: false,
                r#waterlogged: true,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 13781 {
            return Some(PaleOakFence {
                r#east: false,
                r#west: false,
                r#waterlogged: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 13788 {
            return Some(PaleOakFence {
                r#east: false,
                r#waterlogged: false,
                r#south: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 13770 {
            return Some(PaleOakFence {
                r#north: false,
                r#waterlogged: true,
                r#east: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 13774 {
            return Some(PaleOakFence {
                r#west: true,
                r#east: true,
                r#south: false,
                r#waterlogged: true,
                r#north: false,
            });
        }
        return None;
    }
}

