use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakFence {
    pub east: bool,
    pub south: bool,
    pub west: bool,
    pub waterlogged: bool,
    pub north: bool,
}


impl BlockState for OakFence {
    fn to_id(self) -> i32 {
        if block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#east == false && block_state.r#north == true { return 6785; }
        if block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#east == false && block_state.r#south == true { return 6789; }
        if block_state.r#east == true && block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#south == false { return 6770; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#west == true { return 6786; }
        if block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#east == true && block_state.r#north == false { return 6774; }
        if block_state.r#north == true && block_state.r#west == true && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#south == false { return 6768; }
        if block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#west == false && block_state.r#east == false && block_state.r#north == false { return 6791; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#south == false { return 6792; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#south == true { return 6781; }
        if block_state.r#west == true && block_state.r#south == true && block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == true { return 6788; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#east == false && block_state.r#south == false && block_state.r#north == false { return 6795; }
        if block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#south == true && block_state.r#east == false && block_state.r#north == false { return 6790; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#west == true { return 6776; }
        if block_state.r#west == true && block_state.r#east == false && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#north == true { return 6782; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == false { return 6765; }
        if block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#north == true && block_state.r#east == true { return 6766; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == false && block_state.r#south == true && block_state.r#east == true { return 6775; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#north == false && block_state.r#east == true && block_state.r#waterlogged == false { return 6779; }
        if block_state.r#south == true && block_state.r#west == true && block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == true { return 6780; }
        if block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#south == true && block_state.r#west == false { return 6773; }
        if block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#east == true && block_state.r#west == true && block_state.r#north == false { return 6772; }
        if block_state.r#west == false && block_state.r#south == false && block_state.r#east == true && block_state.r#north == true && block_state.r#waterlogged == true { return 6769; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == true { return 6764; }
        if block_state.r#north == true && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#south == true { return 6783; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#west == true && block_state.r#south == false && block_state.r#waterlogged == false { return 6778; }
        if block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#east == true && block_state.r#north == true && block_state.r#west == false { return 6767; }
        if block_state.r#north == true && block_state.r#west == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#east == true { return 6771; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == false { return 6793; }
        if block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#south == false && block_state.r#west == true && block_state.r#east == false { return 6784; }
        if block_state.r#south == false && block_state.r#east == true && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == false { return 6777; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#west == false { return 6787; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#south == false { return 6794; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6785 {
            return Some(OakFence {
                r#south: false,
                r#waterlogged: true,
                r#west: false,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 6789 {
            return Some(OakFence {
                r#north: false,
                r#waterlogged: true,
                r#west: false,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 6770 {
            return Some(OakFence {
                r#east: true,
                r#west: true,
                r#waterlogged: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 6786 {
            return Some(OakFence {
                r#north: true,
                r#south: false,
                r#waterlogged: false,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 6774 {
            return Some(OakFence {
                r#south: true,
                r#waterlogged: false,
                r#west: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 6768 {
            return Some(OakFence {
                r#north: true,
                r#west: true,
                r#east: true,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 6791 {
            return Some(OakFence {
                r#waterlogged: false,
                r#south: true,
                r#west: false,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 6792 {
            return Some(OakFence {
                r#west: true,
                r#north: false,
                r#waterlogged: true,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 6781 {
            return Some(OakFence {
                r#east: false,
                r#north: true,
                r#west: false,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 6788 {
            return Some(OakFence {
                r#west: true,
                r#south: true,
                r#east: false,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6795 {
            return Some(OakFence {
                r#waterlogged: false,
                r#west: false,
                r#east: false,
                r#south: false,
                r#north: false,
            });
        }
        if state_id == 6790 {
            return Some(OakFence {
                r#waterlogged: false,
                r#west: true,
                r#south: true,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 6776 {
            return Some(OakFence {
                r#east: true,
                r#south: false,
                r#waterlogged: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 6782 {
            return Some(OakFence {
                r#west: true,
                r#east: false,
                r#south: true,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 6765 {
            return Some(OakFence {
                r#south: true,
                r#east: true,
                r#north: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 6766 {
            return Some(OakFence {
                r#west: true,
                r#waterlogged: false,
                r#south: true,
                r#north: true,
                r#east: true,
            });
        }
        if state_id == 6775 {
            return Some(OakFence {
                r#waterlogged: false,
                r#west: false,
                r#north: false,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 6779 {
            return Some(OakFence {
                r#south: false,
                r#west: false,
                r#north: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6780 {
            return Some(OakFence {
                r#south: true,
                r#west: true,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6773 {
            return Some(OakFence {
                r#north: false,
                r#waterlogged: true,
                r#east: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 6772 {
            return Some(OakFence {
                r#waterlogged: true,
                r#south: true,
                r#east: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 6769 {
            return Some(OakFence {
                r#west: false,
                r#south: false,
                r#east: true,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6764 {
            return Some(OakFence {
                r#east: true,
                r#south: true,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 6783 {
            return Some(OakFence {
                r#north: true,
                r#west: false,
                r#waterlogged: false,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 6778 {
            return Some(OakFence {
                r#north: false,
                r#east: true,
                r#west: true,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6767 {
            return Some(OakFence {
                r#waterlogged: false,
                r#south: true,
                r#east: true,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 6771 {
            return Some(OakFence {
                r#north: true,
                r#west: false,
                r#south: false,
                r#waterlogged: false,
                r#east: true,
            });
        }
        if state_id == 6793 {
            return Some(OakFence {
                r#east: false,
                r#south: false,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 6784 {
            return Some(OakFence {
                r#waterlogged: true,
                r#north: true,
                r#south: false,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 6777 {
            return Some(OakFence {
                r#south: false,
                r#east: true,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 6787 {
            return Some(OakFence {
                r#east: false,
                r#south: false,
                r#waterlogged: false,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 6794 {
            return Some(OakFence {
                r#east: false,
                r#west: true,
                r#north: false,
                r#waterlogged: false,
                r#south: false,
            });
        }
        return None;
    }
}

