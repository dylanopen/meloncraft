use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakFence {
    pub waterlogged: bool,
    pub north: bool,
    pub east: bool,
    pub west: bool,
    pub south: bool,
}


impl BlockState for DarkOakFence {
    fn to_id(self) -> i32 {
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#south == false && block_state.r#west == true { return 13736; }
        if block_state.r#north == true && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#east == true { return 13731; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#east == true && block_state.r#west == true && block_state.r#waterlogged == false { return 13740; }
        if block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#east == true && block_state.r#west == false { return 13739; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#north == true { return 13749; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#south == true && block_state.r#west == false { return 13733; }
        if block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#east == true && block_state.r#north == false && block_state.r#west == true { return 13738; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#west == false { return 13751; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#west == false { return 13757; }
        if block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#south == false && block_state.r#north == false { return 13760; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#west == false && block_state.r#south == false && block_state.r#waterlogged == true { return 13743; }
        if block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#south == true && block_state.r#east == true && block_state.r#north == true { return 13732; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#east == false && block_state.r#north == true && block_state.r#south == false { return 13753; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#north == true { return 13748; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#west == true && block_state.r#north == false { return 13744; }
        if block_state.r#east == true && block_state.r#west == false && block_state.r#north == true && block_state.r#south == false && block_state.r#waterlogged == false { return 13737; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#south == true { return 13756; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == true { return 13750; }
        if block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#east == false && block_state.r#north == true && block_state.r#west == false { return 13747; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#west == false && block_state.r#east == false && block_state.r#waterlogged == true { return 13755; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == false { return 13741; }
        if block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#north == false && block_state.r#south == false && block_state.r#west == false { return 13745; }
        if block_state.r#south == true && block_state.r#west == true && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#east == true { return 13730; }
        if block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#east == false && block_state.r#north == false { return 13761; }
        if block_state.r#south == false && block_state.r#east == true && block_state.r#west == true && block_state.r#north == true && block_state.r#waterlogged == true { return 13734; }
        if block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#east == false && block_state.r#west == true && block_state.r#north == false { return 13754; }
        if block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#east == false && block_state.r#west == true && block_state.r#north == true { return 13746; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#south == false { return 13758; }
        if block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#north == false && block_state.r#east == false && block_state.r#south == false { return 13759; }
        if block_state.r#south == false && block_state.r#east == true && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#north == true { return 13735; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == false { return 13742; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == true { return 13752; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13736 {
            return Some(DarkOakFence {
                r#east: true,
                r#waterlogged: false,
                r#north: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 13731 {
            return Some(DarkOakFence {
                r#north: true,
                r#west: false,
                r#waterlogged: true,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 13740 {
            return Some(DarkOakFence {
                r#north: false,
                r#south: true,
                r#east: true,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13739 {
            return Some(DarkOakFence {
                r#north: false,
                r#waterlogged: true,
                r#south: true,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 13749 {
            return Some(DarkOakFence {
                r#west: false,
                r#south: true,
                r#east: false,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 13733 {
            return Some(DarkOakFence {
                r#east: true,
                r#waterlogged: false,
                r#north: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 13738 {
            return Some(DarkOakFence {
                r#waterlogged: true,
                r#south: true,
                r#east: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 13751 {
            return Some(DarkOakFence {
                r#south: false,
                r#east: false,
                r#waterlogged: true,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 13757 {
            return Some(DarkOakFence {
                r#east: false,
                r#north: false,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 13760 {
            return Some(DarkOakFence {
                r#east: false,
                r#waterlogged: false,
                r#west: true,
                r#south: false,
                r#north: false,
            });
        }
        if state_id == 13743 {
            return Some(DarkOakFence {
                r#north: false,
                r#east: true,
                r#west: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13732 {
            return Some(DarkOakFence {
                r#waterlogged: false,
                r#west: true,
                r#south: true,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 13753 {
            return Some(DarkOakFence {
                r#waterlogged: false,
                r#west: false,
                r#east: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 13748 {
            return Some(DarkOakFence {
                r#east: false,
                r#west: true,
                r#south: true,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 13744 {
            return Some(DarkOakFence {
                r#east: true,
                r#waterlogged: false,
                r#south: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 13737 {
            return Some(DarkOakFence {
                r#east: true,
                r#west: false,
                r#north: true,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13756 {
            return Some(DarkOakFence {
                r#north: false,
                r#west: true,
                r#east: false,
                r#waterlogged: false,
                r#south: true,
            });
        }
        if state_id == 13750 {
            return Some(DarkOakFence {
                r#south: false,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 13747 {
            return Some(DarkOakFence {
                r#waterlogged: true,
                r#south: true,
                r#east: false,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 13755 {
            return Some(DarkOakFence {
                r#north: false,
                r#south: true,
                r#west: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13741 {
            return Some(DarkOakFence {
                r#east: true,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 13745 {
            return Some(DarkOakFence {
                r#waterlogged: false,
                r#east: true,
                r#north: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 13730 {
            return Some(DarkOakFence {
                r#south: true,
                r#west: true,
                r#north: true,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 13761 {
            return Some(DarkOakFence {
                r#south: false,
                r#waterlogged: false,
                r#west: false,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 13734 {
            return Some(DarkOakFence {
                r#south: false,
                r#east: true,
                r#west: true,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13754 {
            return Some(DarkOakFence {
                r#waterlogged: true,
                r#south: true,
                r#east: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 13746 {
            return Some(DarkOakFence {
                r#waterlogged: true,
                r#south: true,
                r#east: false,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 13758 {
            return Some(DarkOakFence {
                r#north: false,
                r#west: true,
                r#waterlogged: true,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 13759 {
            return Some(DarkOakFence {
                r#waterlogged: true,
                r#west: false,
                r#north: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 13735 {
            return Some(DarkOakFence {
                r#south: false,
                r#east: true,
                r#west: false,
                r#waterlogged: true,
                r#north: true,
            });
        }
        if state_id == 13742 {
            return Some(DarkOakFence {
                r#east: true,
                r#north: false,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 13752 {
            return Some(DarkOakFence {
                r#east: false,
                r#north: true,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        return None;
    }
}

