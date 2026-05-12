use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleFence {
    pub north: bool,
    pub waterlogged: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
}


impl BlockState for JungleFence {
    fn to_id(self) -> i32 {
        if block_state.r#south == false && block_state.r#east == true && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == false { return 13647; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#east == true && block_state.r#north == true && block_state.r#waterlogged == true { return 13635; }
        if block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#north == false && block_state.r#south == true && block_state.r#east == true { return 13642; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#south == true { return 13650; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#south == true && block_state.r#north == false && block_state.r#east == true { return 13645; }
        if block_state.r#south == false && block_state.r#north == false && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == false { return 13663; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#south == false { return 13657; }
        if block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#south == true && block_state.r#west == true && block_state.r#east == true { return 13634; }
        if block_state.r#south == true && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#north == true { return 13653; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#south == false && block_state.r#north == true && block_state.r#waterlogged == false { return 13656; }
        if block_state.r#south == true && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#west == true { return 13658; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#north == true { return 13641; }
        if block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#north == false && block_state.r#south == true && block_state.r#east == true { return 13643; }
        if block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#west == false && block_state.r#south == false && block_state.r#east == false { return 13665; }
        if block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#north == true && block_state.r#east == false && block_state.r#west == false { return 13651; }
        if block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == false && block_state.r#east == false { return 13655; }
        if block_state.r#east == true && block_state.r#west == true && block_state.r#south == false && block_state.r#north == true && block_state.r#waterlogged == false { return 13640; }
        if block_state.r#west == false && block_state.r#north == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#east == true { return 13637; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#north == true && block_state.r#west == true && block_state.r#waterlogged == false { return 13652; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#west == true && block_state.r#south == false && block_state.r#waterlogged == false { return 13648; }
        if block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#west == true && block_state.r#east == true && block_state.r#south == true { return 13644; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#west == false { return 13649; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#south == false && block_state.r#east == false && block_state.r#waterlogged == true { return 13662; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#west == true { return 13664; }
        if block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#east == true && block_state.r#north == true && block_state.r#south == false { return 13638; }
        if block_state.r#south == false && block_state.r#west == true && block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == true { return 13654; }
        if block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#west == false && block_state.r#east == false && block_state.r#south == true { return 13661; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == false { return 13646; }
        if block_state.r#south == true && block_state.r#west == true && block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == false { return 13660; }
        if block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#west == false && block_state.r#east == false && block_state.r#south == true { return 13659; }
        if block_state.r#west == true && block_state.r#north == true && block_state.r#south == true && block_state.r#east == true && block_state.r#waterlogged == false { return 13636; }
        if block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#east == true && block_state.r#north == true { return 13639; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13647 {
            return Some(JungleFence {
                r#south: false,
                r#east: true,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 13635 {
            return Some(JungleFence {
                r#west: false,
                r#south: true,
                r#east: true,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13642 {
            return Some(JungleFence {
                r#waterlogged: true,
                r#west: true,
                r#north: false,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 13650 {
            return Some(JungleFence {
                r#east: false,
                r#north: true,
                r#west: true,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 13645 {
            return Some(JungleFence {
                r#waterlogged: false,
                r#west: false,
                r#south: true,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 13663 {
            return Some(JungleFence {
                r#south: false,
                r#north: false,
                r#east: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 13657 {
            return Some(JungleFence {
                r#north: true,
                r#east: false,
                r#west: false,
                r#waterlogged: false,
                r#south: false,
            });
        }
        if state_id == 13634 {
            return Some(JungleFence {
                r#waterlogged: true,
                r#north: true,
                r#south: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 13653 {
            return Some(JungleFence {
                r#south: true,
                r#west: false,
                r#waterlogged: false,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 13656 {
            return Some(JungleFence {
                r#east: false,
                r#west: true,
                r#south: false,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13658 {
            return Some(JungleFence {
                r#south: true,
                r#north: false,
                r#waterlogged: true,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 13641 {
            return Some(JungleFence {
                r#east: true,
                r#south: false,
                r#west: false,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 13643 {
            return Some(JungleFence {
                r#waterlogged: true,
                r#west: false,
                r#north: false,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 13665 {
            return Some(JungleFence {
                r#waterlogged: false,
                r#north: false,
                r#west: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 13651 {
            return Some(JungleFence {
                r#waterlogged: true,
                r#south: true,
                r#north: true,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 13655 {
            return Some(JungleFence {
                r#north: true,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 13640 {
            return Some(JungleFence {
                r#east: true,
                r#west: true,
                r#south: false,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13637 {
            return Some(JungleFence {
                r#west: false,
                r#north: true,
                r#south: true,
                r#waterlogged: false,
                r#east: true,
            });
        }
        if state_id == 13652 {
            return Some(JungleFence {
                r#east: false,
                r#south: true,
                r#north: true,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13648 {
            return Some(JungleFence {
                r#east: true,
                r#north: false,
                r#west: true,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13644 {
            return Some(JungleFence {
                r#waterlogged: false,
                r#north: false,
                r#west: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 13649 {
            return Some(JungleFence {
                r#north: false,
                r#south: false,
                r#waterlogged: false,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 13662 {
            return Some(JungleFence {
                r#north: false,
                r#west: true,
                r#south: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13664 {
            return Some(JungleFence {
                r#north: false,
                r#east: false,
                r#waterlogged: false,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 13638 {
            return Some(JungleFence {
                r#waterlogged: true,
                r#west: true,
                r#east: true,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 13654 {
            return Some(JungleFence {
                r#south: false,
                r#west: true,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13661 {
            return Some(JungleFence {
                r#waterlogged: false,
                r#north: false,
                r#west: false,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 13646 {
            return Some(JungleFence {
                r#north: false,
                r#east: true,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 13660 {
            return Some(JungleFence {
                r#south: true,
                r#west: true,
                r#east: false,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13659 {
            return Some(JungleFence {
                r#waterlogged: true,
                r#north: false,
                r#west: false,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 13636 {
            return Some(JungleFence {
                r#west: true,
                r#north: true,
                r#south: true,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13639 {
            return Some(JungleFence {
                r#south: false,
                r#waterlogged: true,
                r#west: false,
                r#east: true,
                r#north: true,
            });
        }
        return None;
    }
}

