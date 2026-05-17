use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlassPane {
    pub east: bool,
    pub south: bool,
    pub north: bool,
    pub waterlogged: bool,
    pub west: bool,
}


impl BlockState for GlassPane {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#east == false && self.r#north == true && self.r#west == false && self.r#south == true { return 8116; }
        if self.r#west == true && self.r#south == false && self.r#east == false && self.r#north == true && self.r#waterlogged == true { return 8119; }
        if self.r#east == false && self.r#waterlogged == false && self.r#south == false && self.r#north == true && self.r#west == false { return 8122; }
        if self.r#waterlogged == true && self.r#west == false && self.r#east == true && self.r#south == false && self.r#north == true { return 8104; }
        if self.r#north == false && self.r#waterlogged == true && self.r#south == true && self.r#east == true && self.r#west == true { return 8107; }
        if self.r#east == false && self.r#south == true && self.r#waterlogged == false && self.r#west == true && self.r#north == true { return 8117; }
        if self.r#north == true && self.r#east == false && self.r#south == true && self.r#waterlogged == false && self.r#west == false { return 8118; }
        if self.r#waterlogged == true && self.r#south == false && self.r#north == true && self.r#east == false && self.r#west == false { return 8120; }
        if self.r#east == false && self.r#south == false && self.r#waterlogged == true && self.r#west == false && self.r#north == false { return 8128; }
        if self.r#north == true && self.r#waterlogged == false && self.r#east == true && self.r#south == false && self.r#west == false { return 8106; }
        if self.r#north == true && self.r#west == true && self.r#waterlogged == true && self.r#east == false && self.r#south == true { return 8115; }
        if self.r#north == true && self.r#east == true && self.r#waterlogged == true && self.r#west == true && self.r#south == false { return 8103; }
        if self.r#north == false && self.r#west == false && self.r#east == true && self.r#south == false && self.r#waterlogged == true { return 8112; }
        if self.r#west == false && self.r#east == true && self.r#north == true && self.r#south == true && self.r#waterlogged == true { return 8100; }
        if self.r#east == true && self.r#south == false && self.r#north == false && self.r#west == true && self.r#waterlogged == true { return 8111; }
        if self.r#east == false && self.r#west == true && self.r#south == false && self.r#waterlogged == false && self.r#north == false { return 8129; }
        if self.r#north == false && self.r#waterlogged == false && self.r#south == false && self.r#west == false && self.r#east == false { return 8130; }
        if self.r#waterlogged == true && self.r#west == true && self.r#north == false && self.r#east == false && self.r#south == false { return 8127; }
        if self.r#south == true && self.r#west == true && self.r#north == false && self.r#waterlogged == true && self.r#east == false { return 8123; }
        if self.r#east == true && self.r#waterlogged == false && self.r#west == true && self.r#south == true && self.r#north == true { return 8101; }
        if self.r#east == true && self.r#south == false && self.r#west == true && self.r#waterlogged == false && self.r#north == false { return 8113; }
        if self.r#north == false && self.r#south == true && self.r#waterlogged == true && self.r#west == false && self.r#east == true { return 8108; }
        if self.r#waterlogged == false && self.r#east == true && self.r#west == false && self.r#north == true && self.r#south == true { return 8102; }
        if self.r#north == true && self.r#east == true && self.r#west == true && self.r#waterlogged == false && self.r#south == false { return 8105; }
        if self.r#north == false && self.r#waterlogged == false && self.r#west == false && self.r#south == false && self.r#east == true { return 8114; }
        if self.r#west == false && self.r#north == false && self.r#east == false && self.r#waterlogged == false && self.r#south == true { return 8126; }
        if self.r#west == false && self.r#north == false && self.r#east == true && self.r#south == true && self.r#waterlogged == false { return 8110; }
        if self.r#south == true && self.r#east == false && self.r#west == false && self.r#waterlogged == true && self.r#north == false { return 8124; }
        if self.r#west == true && self.r#north == true && self.r#east == false && self.r#south == false && self.r#waterlogged == false { return 8121; }
        if self.r#south == true && self.r#north == false && self.r#east == false && self.r#waterlogged == false && self.r#west == true { return 8125; }
        if self.r#west == true && self.r#north == true && self.r#waterlogged == true && self.r#south == true && self.r#east == true { return 8099; }
        if self.r#waterlogged == false && self.r#west == true && self.r#east == true && self.r#south == true && self.r#north == false { return 8109; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8116 {
            return Some(GlassPane {
                r#waterlogged: true,
                r#east: false,
                r#north: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 8119 {
            return Some(GlassPane {
                r#west: true,
                r#south: false,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 8122 {
            return Some(GlassPane {
                r#east: false,
                r#waterlogged: false,
                r#south: false,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 8104 {
            return Some(GlassPane {
                r#waterlogged: true,
                r#west: false,
                r#east: true,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 8107 {
            return Some(GlassPane {
                r#north: false,
                r#waterlogged: true,
                r#south: true,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 8117 {
            return Some(GlassPane {
                r#east: false,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 8118 {
            return Some(GlassPane {
                r#north: true,
                r#east: false,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 8120 {
            return Some(GlassPane {
                r#waterlogged: true,
                r#south: false,
                r#north: true,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 8128 {
            return Some(GlassPane {
                r#east: false,
                r#south: false,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 8106 {
            return Some(GlassPane {
                r#north: true,
                r#waterlogged: false,
                r#east: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 8115 {
            return Some(GlassPane {
                r#north: true,
                r#west: true,
                r#waterlogged: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 8103 {
            return Some(GlassPane {
                r#north: true,
                r#east: true,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 8112 {
            return Some(GlassPane {
                r#north: false,
                r#west: false,
                r#east: true,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8100 {
            return Some(GlassPane {
                r#west: false,
                r#east: true,
                r#north: true,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 8111 {
            return Some(GlassPane {
                r#east: true,
                r#south: false,
                r#north: false,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 8129 {
            return Some(GlassPane {
                r#east: false,
                r#west: true,
                r#south: false,
                r#waterlogged: false,
                r#north: false,
            });
        }
        if state_id == 8130 {
            return Some(GlassPane {
                r#north: false,
                r#waterlogged: false,
                r#south: false,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 8127 {
            return Some(GlassPane {
                r#waterlogged: true,
                r#west: true,
                r#north: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 8123 {
            return Some(GlassPane {
                r#south: true,
                r#west: true,
                r#north: false,
                r#waterlogged: true,
                r#east: false,
            });
        }
        if state_id == 8101 {
            return Some(GlassPane {
                r#east: true,
                r#waterlogged: false,
                r#west: true,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 8113 {
            return Some(GlassPane {
                r#east: true,
                r#south: false,
                r#west: true,
                r#waterlogged: false,
                r#north: false,
            });
        }
        if state_id == 8108 {
            return Some(GlassPane {
                r#north: false,
                r#south: true,
                r#waterlogged: true,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 8102 {
            return Some(GlassPane {
                r#waterlogged: false,
                r#east: true,
                r#west: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 8105 {
            return Some(GlassPane {
                r#north: true,
                r#east: true,
                r#west: true,
                r#waterlogged: false,
                r#south: false,
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
        if state_id == 8126 {
            return Some(GlassPane {
                r#west: false,
                r#north: false,
                r#east: false,
                r#waterlogged: false,
                r#south: true,
            });
        }
        if state_id == 8110 {
            return Some(GlassPane {
                r#west: false,
                r#north: false,
                r#east: true,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8124 {
            return Some(GlassPane {
                r#south: true,
                r#east: false,
                r#west: false,
                r#waterlogged: true,
                r#north: false,
            });
        }
        if state_id == 8121 {
            return Some(GlassPane {
                r#west: true,
                r#north: true,
                r#east: false,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 8125 {
            return Some(GlassPane {
                r#south: true,
                r#north: false,
                r#east: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 8099 {
            return Some(GlassPane {
                r#west: true,
                r#north: true,
                r#waterlogged: true,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 8109 {
            return Some(GlassPane {
                r#waterlogged: false,
                r#west: true,
                r#east: true,
                r#south: true,
                r#north: false,
            });
        }
        return None;
    }
}

