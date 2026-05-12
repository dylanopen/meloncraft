use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedStainedGlassPane {
    pub west: bool,
    pub south: bool,
    pub east: bool,
    pub north: bool,
    pub waterlogged: bool,
}


impl BlockState for RedStainedGlassPane {
    fn to_id(self) -> i32 {
        if block_state.r#north == true && block_state.r#south == true && block_state.r#west == false && block_state.r#east == false && block_state.r#waterlogged == false { return 11725; }
        if block_state.r#north == false && block_state.r#west == false && block_state.r#south == false && block_state.r#east == false && block_state.r#waterlogged == false { return 11737; }
        if block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#west == true && block_state.r#east == false { return 11722; }
        if block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#south == true && block_state.r#west == true && block_state.r#east == true { return 11714; }
        if block_state.r#south == false && block_state.r#east == true && block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#north == true { return 11712; }
        if block_state.r#north == true && block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#south == false { return 11728; }
        if block_state.r#east == true && block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#south == true { return 11716; }
        if block_state.r#east == true && block_state.r#north == true && block_state.r#south == true && block_state.r#west == true && block_state.r#waterlogged == false { return 11708; }
        if block_state.r#east == true && block_state.r#west == true && block_state.r#south == false && block_state.r#north == true && block_state.r#waterlogged == true { return 11710; }
        if block_state.r#east == false && block_state.r#west == false && block_state.r#south == true && block_state.r#north == false && block_state.r#waterlogged == true { return 11731; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#west == true { return 11732; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == false { return 11721; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == false { return 11719; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#south == false { return 11727; }
        if block_state.r#south == false && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#north == false { return 11720; }
        if block_state.r#east == true && block_state.r#west == false && block_state.r#south == true && block_state.r#north == true && block_state.r#waterlogged == true { return 11707; }
        if block_state.r#south == true && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#west == false { return 11733; }
        if block_state.r#west == false && block_state.r#north == false && block_state.r#east == true && block_state.r#south == true && block_state.r#waterlogged == false { return 11717; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#north == false && block_state.r#south == false { return 11734; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#north == false && block_state.r#south == false { return 11735; }
        if block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#east == false && block_state.r#west == true { return 11726; }
        if block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#south == false && block_state.r#west == false { return 11729; }
        if block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#west == false && block_state.r#east == true && block_state.r#north == true { return 11713; }
        if block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#east == false && block_state.r#north == true && block_state.r#south == true { return 11723; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#south == true { return 11706; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == false { return 11709; }
        if block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#west == true && block_state.r#north == true { return 11724; }
        if block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#east == false && block_state.r#north == false && block_state.r#south == true { return 11730; }
        if block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#north == true && block_state.r#west == false && block_state.r#east == true { return 11711; }
        if block_state.r#north == false && block_state.r#west == false && block_state.r#east == true && block_state.r#south == true && block_state.r#waterlogged == true { return 11715; }
        if block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#east == false && block_state.r#north == false && block_state.r#west == true { return 11736; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#south == false { return 11718; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11725 {
            return Some(RedStainedGlassPane {
                r#north: true,
                r#south: true,
                r#west: false,
                r#east: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11737 {
            return Some(RedStainedGlassPane {
                r#north: false,
                r#west: false,
                r#south: false,
                r#east: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11722 {
            return Some(RedStainedGlassPane {
                r#north: true,
                r#waterlogged: true,
                r#south: true,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 11714 {
            return Some(RedStainedGlassPane {
                r#waterlogged: true,
                r#north: false,
                r#south: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 11712 {
            return Some(RedStainedGlassPane {
                r#south: false,
                r#east: true,
                r#west: true,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 11728 {
            return Some(RedStainedGlassPane {
                r#north: true,
                r#west: true,
                r#waterlogged: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 11716 {
            return Some(RedStainedGlassPane {
                r#east: true,
                r#west: true,
                r#waterlogged: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 11708 {
            return Some(RedStainedGlassPane {
                r#east: true,
                r#north: true,
                r#south: true,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11710 {
            return Some(RedStainedGlassPane {
                r#east: true,
                r#west: true,
                r#south: false,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11731 {
            return Some(RedStainedGlassPane {
                r#east: false,
                r#west: false,
                r#south: true,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11732 {
            return Some(RedStainedGlassPane {
                r#north: false,
                r#east: false,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11721 {
            return Some(RedStainedGlassPane {
                r#north: false,
                r#east: true,
                r#south: false,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11719 {
            return Some(RedStainedGlassPane {
                r#east: true,
                r#north: false,
                r#south: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 11727 {
            return Some(RedStainedGlassPane {
                r#north: true,
                r#east: false,
                r#west: false,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 11720 {
            return Some(RedStainedGlassPane {
                r#south: false,
                r#east: true,
                r#waterlogged: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 11707 {
            return Some(RedStainedGlassPane {
                r#east: true,
                r#west: false,
                r#south: true,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11733 {
            return Some(RedStainedGlassPane {
                r#south: true,
                r#east: false,
                r#waterlogged: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 11717 {
            return Some(RedStainedGlassPane {
                r#west: false,
                r#north: false,
                r#east: true,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11734 {
            return Some(RedStainedGlassPane {
                r#east: false,
                r#waterlogged: true,
                r#west: true,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 11735 {
            return Some(RedStainedGlassPane {
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 11726 {
            return Some(RedStainedGlassPane {
                r#north: true,
                r#waterlogged: true,
                r#south: false,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 11729 {
            return Some(RedStainedGlassPane {
                r#north: true,
                r#waterlogged: false,
                r#east: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 11713 {
            return Some(RedStainedGlassPane {
                r#waterlogged: false,
                r#south: false,
                r#west: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 11723 {
            return Some(RedStainedGlassPane {
                r#waterlogged: true,
                r#west: false,
                r#east: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 11706 {
            return Some(RedStainedGlassPane {
                r#west: true,
                r#east: true,
                r#waterlogged: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 11709 {
            return Some(RedStainedGlassPane {
                r#south: true,
                r#north: true,
                r#east: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11724 {
            return Some(RedStainedGlassPane {
                r#east: false,
                r#waterlogged: false,
                r#south: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 11730 {
            return Some(RedStainedGlassPane {
                r#waterlogged: true,
                r#west: true,
                r#east: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 11711 {
            return Some(RedStainedGlassPane {
                r#waterlogged: true,
                r#south: false,
                r#north: true,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 11715 {
            return Some(RedStainedGlassPane {
                r#north: false,
                r#west: false,
                r#east: true,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11736 {
            return Some(RedStainedGlassPane {
                r#waterlogged: false,
                r#south: false,
                r#east: false,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 11718 {
            return Some(RedStainedGlassPane {
                r#north: false,
                r#west: true,
                r#east: true,
                r#waterlogged: true,
                r#south: false,
            });
        }
        return None;
    }
}

