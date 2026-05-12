use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlassPane {
    pub waterlogged: bool,
    pub east: bool,
    pub north: bool,
    pub west: bool,
    pub south: bool,
}


impl BlockState for GlassPane {
    fn to_id(self) -> i32 {
        if block_state.r#east == false && block_state.r#south == true && block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#north == false { return 8123; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == true { return 8107; }
        if block_state.r#south == true && block_state.r#east == false && block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#north == true { return 8115; }
        if block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#north == true && block_state.r#south == false { return 8120; }
        if block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#west == false && block_state.r#north == false { return 8108; }
        if block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#east == true && block_state.r#north == false { return 8111; }
        if block_state.r#west == true && block_state.r#south == true && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#north == true { return 8117; }
        if block_state.r#south == false && block_state.r#west == true && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#east == false { return 8127; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#east == true { return 8105; }
        if block_state.r#west == false && block_state.r#north == true && block_state.r#south == true && block_state.r#east == false && block_state.r#waterlogged == true { return 8116; }
        if block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#east == false && block_state.r#south == false { return 8129; }
        if block_state.r#south == true && block_state.r#west == true && block_state.r#north == false && block_state.r#east == true && block_state.r#waterlogged == false { return 8109; }
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#north == true && block_state.r#south == true { return 8100; }
        if block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#south == false && block_state.r#east == false { return 8130; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#south == true && block_state.r#north == true { return 8102; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#west == false && block_state.r#east == false && block_state.r#waterlogged == false { return 8126; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == false { return 8110; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#north == true && block_state.r#south == true && block_state.r#waterlogged == true { return 8099; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#north == true { return 8118; }
        if block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#south == false && block_state.r#east == true { return 8114; }
        if block_state.r#west == true && block_state.r#north == true && block_state.r#south == false && block_state.r#east == false && block_state.r#waterlogged == true { return 8119; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#south == true && block_state.r#north == true && block_state.r#waterlogged == false { return 8101; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#east == true && block_state.r#west == false && block_state.r#waterlogged == true { return 8104; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#west == true { return 8121; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#west == false && block_state.r#east == true && block_state.r#waterlogged == false { return 8106; }
        if block_state.r#west == false && block_state.r#south == false && block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == true { return 8128; }
        if block_state.r#south == false && block_state.r#north == true && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == true { return 8103; }
        if block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#south == false && block_state.r#west == false && block_state.r#east == true { return 8112; }
        if block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#north == false && block_state.r#east == false && block_state.r#west == true { return 8125; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#north == false { return 8124; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == false { return 8113; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#north == true && block_state.r#east == false && block_state.r#waterlogged == false { return 8122; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8123 {
            return Some(GlassPane {
                r#east: false,
                r#south: true,
                r#west: true,
                r#waterlogged: true,
                r#north: false,
            });
        }
        if state_id == 8107 {
            return Some(GlassPane {
                r#east: true,
                r#south: true,
                r#north: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 8115 {
            return Some(GlassPane {
                r#south: true,
                r#east: false,
                r#west: true,
                r#waterlogged: true,
                r#north: true,
            });
        }
        if state_id == 8120 {
            return Some(GlassPane {
                r#west: false,
                r#waterlogged: true,
                r#east: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 8108 {
            return Some(GlassPane {
                r#south: true,
                r#waterlogged: true,
                r#east: true,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 8111 {
            return Some(GlassPane {
                r#south: false,
                r#waterlogged: true,
                r#west: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 8117 {
            return Some(GlassPane {
                r#west: true,
                r#south: true,
                r#east: false,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 8127 {
            return Some(GlassPane {
                r#south: false,
                r#west: true,
                r#north: false,
                r#waterlogged: true,
                r#east: false,
            });
        }
        if state_id == 8105 {
            return Some(GlassPane {
                r#north: true,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 8116 {
            return Some(GlassPane {
                r#west: false,
                r#north: true,
                r#south: true,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8129 {
            return Some(GlassPane {
                r#west: true,
                r#waterlogged: false,
                r#north: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 8109 {
            return Some(GlassPane {
                r#south: true,
                r#west: true,
                r#north: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8100 {
            return Some(GlassPane {
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 8130 {
            return Some(GlassPane {
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 8102 {
            return Some(GlassPane {
                r#east: true,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 8126 {
            return Some(GlassPane {
                r#north: false,
                r#south: true,
                r#west: false,
                r#east: false,
                r#waterlogged: false,
            });
        }
        if state_id == 8110 {
            return Some(GlassPane {
                r#east: true,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 8099 {
            return Some(GlassPane {
                r#west: true,
                r#east: true,
                r#north: true,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 8118 {
            return Some(GlassPane {
                r#west: false,
                r#south: true,
                r#east: false,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 8114 {
            return Some(GlassPane {
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 8119 {
            return Some(GlassPane {
                r#west: true,
                r#north: true,
                r#south: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8101 {
            return Some(GlassPane {
                r#west: true,
                r#east: true,
                r#south: true,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8104 {
            return Some(GlassPane {
                r#north: true,
                r#south: false,
                r#east: true,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8121 {
            return Some(GlassPane {
                r#east: false,
                r#north: true,
                r#waterlogged: false,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 8106 {
            return Some(GlassPane {
                r#north: true,
                r#south: false,
                r#west: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8128 {
            return Some(GlassPane {
                r#west: false,
                r#south: false,
                r#east: false,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8103 {
            return Some(GlassPane {
                r#south: false,
                r#north: true,
                r#east: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 8112 {
            return Some(GlassPane {
                r#waterlogged: true,
                r#north: false,
                r#south: false,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 8125 {
            return Some(GlassPane {
                r#waterlogged: false,
                r#south: true,
                r#north: false,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 8124 {
            return Some(GlassPane {
                r#east: false,
                r#south: true,
                r#west: false,
                r#waterlogged: true,
                r#north: false,
            });
        }
        if state_id == 8113 {
            return Some(GlassPane {
                r#west: true,
                r#east: true,
                r#north: false,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 8122 {
            return Some(GlassPane {
                r#south: false,
                r#west: false,
                r#north: true,
                r#east: false,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

