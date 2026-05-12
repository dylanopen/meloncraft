use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackStainedGlassPane {
    pub waterlogged: bool,
    pub south: bool,
    pub east: bool,
    pub north: bool,
    pub west: bool,
}


impl BlockState for BlackStainedGlassPane {
    fn to_id(self) -> i32 {
        if block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#east == true && block_state.r#north == true { return 11745; }
        if block_state.r#east == true && block_state.r#west == true && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#south == false { return 11742; }
        if block_state.r#north == true && block_state.r#west == false && block_state.r#south == false && block_state.r#east == true && block_state.r#waterlogged == true { return 11743; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == false { return 11758; }
        if block_state.r#north == true && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#south == true { return 11755; }
        if block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#east == true && block_state.r#north == false && block_state.r#west == true { return 11746; }
        if block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#east == false && block_state.r#south == true { return 11754; }
        if block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#south == true && block_state.r#west == true { return 11756; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#north == false { return 11768; }
        if block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#east == true && block_state.r#north == true { return 11738; }
        if block_state.r#north == false && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#east == true { return 11747; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#west == false { return 11767; }
        if block_state.r#east == true && block_state.r#west == false && block_state.r#north == true && block_state.r#south == true && block_state.r#waterlogged == false { return 11741; }
        if block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#east == false && block_state.r#west == false { return 11765; }
        if block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == false && block_state.r#north == false && block_state.r#east == true { return 11751; }
        if block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#east == false && block_state.r#west == false { return 11757; }
        if block_state.r#west == false && block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#south == true { return 11763; }
        if block_state.r#south == false && block_state.r#west == true && block_state.r#north == true && block_state.r#east == false && block_state.r#waterlogged == false { return 11760; }
        if block_state.r#south == false && block_state.r#west == true && block_state.r#east == true && block_state.r#north == false && block_state.r#waterlogged == true { return 11750; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#west == false && block_state.r#south == false && block_state.r#waterlogged == false { return 11761; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#south == true && block_state.r#west == true { return 11762; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#south == true && block_state.r#west == true && block_state.r#waterlogged == false { return 11740; }
        if block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#east == true && block_state.r#west == false { return 11739; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == false { return 11759; }
        if block_state.r#west == true && block_state.r#east == false && block_state.r#south == true && block_state.r#north == false && block_state.r#waterlogged == false { return 11764; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == false { return 11753; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#west == false { return 11769; }
        if block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#north == false && block_state.r#west == true { return 11752; }
        if block_state.r#east == true && block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#south == false { return 11744; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#north == false && block_state.r#west == false && block_state.r#waterlogged == false { return 11749; }
        if block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#south == true && block_state.r#north == false && block_state.r#west == true { return 11748; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#north == false { return 11766; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11745 {
            return Some(BlackStainedGlassPane {
                r#south: false,
                r#waterlogged: false,
                r#west: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 11742 {
            return Some(BlackStainedGlassPane {
                r#east: true,
                r#west: true,
                r#north: true,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 11743 {
            return Some(BlackStainedGlassPane {
                r#north: true,
                r#west: false,
                r#south: false,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11758 {
            return Some(BlackStainedGlassPane {
                r#north: true,
                r#east: false,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 11755 {
            return Some(BlackStainedGlassPane {
                r#north: true,
                r#west: false,
                r#waterlogged: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 11746 {
            return Some(BlackStainedGlassPane {
                r#waterlogged: true,
                r#south: true,
                r#east: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 11754 {
            return Some(BlackStainedGlassPane {
                r#north: true,
                r#waterlogged: true,
                r#west: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 11756 {
            return Some(BlackStainedGlassPane {
                r#north: true,
                r#waterlogged: false,
                r#east: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 11768 {
            return Some(BlackStainedGlassPane {
                r#east: false,
                r#south: false,
                r#west: true,
                r#waterlogged: false,
                r#north: false,
            });
        }
        if state_id == 11738 {
            return Some(BlackStainedGlassPane {
                r#south: true,
                r#waterlogged: true,
                r#west: true,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 11747 {
            return Some(BlackStainedGlassPane {
                r#north: false,
                r#west: false,
                r#waterlogged: true,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 11767 {
            return Some(BlackStainedGlassPane {
                r#east: false,
                r#north: false,
                r#waterlogged: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 11741 {
            return Some(BlackStainedGlassPane {
                r#east: true,
                r#west: false,
                r#north: true,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11765 {
            return Some(BlackStainedGlassPane {
                r#south: true,
                r#waterlogged: false,
                r#north: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 11751 {
            return Some(BlackStainedGlassPane {
                r#waterlogged: true,
                r#west: false,
                r#south: false,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 11757 {
            return Some(BlackStainedGlassPane {
                r#south: true,
                r#waterlogged: false,
                r#north: true,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 11763 {
            return Some(BlackStainedGlassPane {
                r#west: false,
                r#east: false,
                r#north: false,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 11760 {
            return Some(BlackStainedGlassPane {
                r#south: false,
                r#west: true,
                r#north: true,
                r#east: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11750 {
            return Some(BlackStainedGlassPane {
                r#south: false,
                r#west: true,
                r#east: true,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11761 {
            return Some(BlackStainedGlassPane {
                r#north: true,
                r#east: false,
                r#west: false,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11762 {
            return Some(BlackStainedGlassPane {
                r#east: false,
                r#waterlogged: true,
                r#north: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 11740 {
            return Some(BlackStainedGlassPane {
                r#north: true,
                r#east: true,
                r#south: true,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11739 {
            return Some(BlackStainedGlassPane {
                r#south: true,
                r#waterlogged: true,
                r#north: true,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 11759 {
            return Some(BlackStainedGlassPane {
                r#north: true,
                r#east: false,
                r#south: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 11764 {
            return Some(BlackStainedGlassPane {
                r#west: true,
                r#east: false,
                r#south: true,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11753 {
            return Some(BlackStainedGlassPane {
                r#north: false,
                r#south: false,
                r#east: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11769 {
            return Some(BlackStainedGlassPane {
                r#east: false,
                r#south: false,
                r#waterlogged: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 11752 {
            return Some(BlackStainedGlassPane {
                r#south: false,
                r#waterlogged: false,
                r#east: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 11744 {
            return Some(BlackStainedGlassPane {
                r#east: true,
                r#north: true,
                r#waterlogged: false,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 11749 {
            return Some(BlackStainedGlassPane {
                r#east: true,
                r#south: true,
                r#north: false,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11748 {
            return Some(BlackStainedGlassPane {
                r#waterlogged: false,
                r#east: true,
                r#south: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 11766 {
            return Some(BlackStainedGlassPane {
                r#east: false,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
                r#north: false,
            });
        }
        return None;
    }
}

