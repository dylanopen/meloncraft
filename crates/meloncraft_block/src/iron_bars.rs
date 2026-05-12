use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IronBars {
    pub north: bool,
    pub south: bool,
    pub west: bool,
    pub waterlogged: bool,
    pub east: bool,
}


impl BlockState for IronBars {
    fn to_id(self) -> i32 {
        if block_state.r#north == true && block_state.r#east == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#west == false { return 7760; }
        if block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#north == true && block_state.r#west == true && block_state.r#south == true { return 7759; }
        if block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == true && block_state.r#east == false { return 7782; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == false { return 7778; }
        if block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#east == true && block_state.r#south == false { return 7764; }
        if block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#south == false && block_state.r#east == true && block_state.r#west == true { return 7771; }
        if block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#east == true && block_state.r#north == false && block_state.r#west == false { return 7770; }
        if block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#east == true && block_state.r#north == false && block_state.r#south == true { return 7767; }
        if block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#west == true && block_state.r#south == false && block_state.r#east == false { return 7785; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#west == false { return 7774; }
        if block_state.r#south == true && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#east == false { return 7784; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#south == false && block_state.r#west == false { return 7786; }
        if block_state.r#north == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#west == false { return 7776; }
        if block_state.r#west == false && block_state.r#south == false && block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == false { return 7788; }
        if block_state.r#north == true && block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#south == true { return 7757; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#south == false && block_state.r#north == true && block_state.r#waterlogged == false { return 7779; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#west == false { return 7768; }
        if block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == false && block_state.r#east == true { return 7772; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == false { return 7762; }
        if block_state.r#south == true && block_state.r#west == false && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#north == true { return 7758; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#south == false { return 7769; }
        if block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#north == false && block_state.r#south == true && block_state.r#west == true { return 7783; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#west == true && block_state.r#east == true && block_state.r#waterlogged == false { return 7763; }
        if block_state.r#west == false && block_state.r#north == true && block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == false { return 7780; }
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#west == true && block_state.r#south == false { return 7761; }
        if block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#north == true && block_state.r#south == true && block_state.r#east == false { return 7775; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#north == true { return 7777; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#east == false { return 7781; }
        if block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == true && block_state.r#east == true { return 7765; }
        if block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#south == true && block_state.r#east == true && block_state.r#west == false { return 7766; }
        if block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#south == true && block_state.r#north == true { return 7773; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#north == false { return 7787; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7760 {
            return Some(IronBars {
                r#north: true,
                r#east: true,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 7759 {
            return Some(IronBars {
                r#waterlogged: false,
                r#east: true,
                r#north: true,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 7782 {
            return Some(IronBars {
                r#north: false,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 7778 {
            return Some(IronBars {
                r#east: false,
                r#north: true,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 7764 {
            return Some(IronBars {
                r#north: true,
                r#waterlogged: false,
                r#west: false,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 7771 {
            return Some(IronBars {
                r#waterlogged: false,
                r#north: false,
                r#south: false,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 7770 {
            return Some(IronBars {
                r#waterlogged: true,
                r#south: false,
                r#east: true,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 7767 {
            return Some(IronBars {
                r#waterlogged: false,
                r#west: true,
                r#east: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 7785 {
            return Some(IronBars {
                r#waterlogged: true,
                r#north: false,
                r#west: true,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 7774 {
            return Some(IronBars {
                r#north: true,
                r#east: false,
                r#south: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 7784 {
            return Some(IronBars {
                r#south: true,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 7786 {
            return Some(IronBars {
                r#east: false,
                r#waterlogged: true,
                r#north: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 7776 {
            return Some(IronBars {
                r#north: true,
                r#south: true,
                r#waterlogged: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 7788 {
            return Some(IronBars {
                r#west: false,
                r#south: false,
                r#east: false,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7757 {
            return Some(IronBars {
                r#north: true,
                r#west: true,
                r#waterlogged: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 7779 {
            return Some(IronBars {
                r#east: false,
                r#west: true,
                r#south: false,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7768 {
            return Some(IronBars {
                r#east: true,
                r#north: false,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 7772 {
            return Some(IronBars {
                r#south: false,
                r#waterlogged: false,
                r#west: false,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 7762 {
            return Some(IronBars {
                r#north: true,
                r#south: false,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 7758 {
            return Some(IronBars {
                r#south: true,
                r#west: false,
                r#east: true,
                r#waterlogged: true,
                r#north: true,
            });
        }
        if state_id == 7769 {
            return Some(IronBars {
                r#north: false,
                r#west: true,
                r#east: true,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 7783 {
            return Some(IronBars {
                r#waterlogged: false,
                r#east: false,
                r#north: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 7763 {
            return Some(IronBars {
                r#north: true,
                r#south: false,
                r#west: true,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7780 {
            return Some(IronBars {
                r#west: false,
                r#north: true,
                r#east: false,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7761 {
            return Some(IronBars {
                r#east: true,
                r#waterlogged: true,
                r#north: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 7775 {
            return Some(IronBars {
                r#waterlogged: false,
                r#west: true,
                r#north: true,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 7777 {
            return Some(IronBars {
                r#east: false,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 7781 {
            return Some(IronBars {
                r#north: false,
                r#west: true,
                r#waterlogged: true,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 7765 {
            return Some(IronBars {
                r#north: false,
                r#waterlogged: true,
                r#west: true,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 7766 {
            return Some(IronBars {
                r#waterlogged: true,
                r#north: false,
                r#south: true,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 7773 {
            return Some(IronBars {
                r#west: true,
                r#waterlogged: true,
                r#east: false,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 7787 {
            return Some(IronBars {
                r#east: false,
                r#west: true,
                r#south: false,
                r#waterlogged: false,
                r#north: false,
            });
        }
        return None;
    }
}

