use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyanStainedGlassPane {
    pub waterlogged: bool,
    pub west: bool,
    pub south: bool,
    pub north: bool,
    pub east: bool,
}


impl BlockState for CyanStainedGlassPane {
    fn to_id(self) -> i32 {
        if block_state.r#south == false && block_state.r#east == true && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == true { return 11550; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#west == false && block_state.r#south == true { return 11549; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#west == true { return 11572; }
        if block_state.r#south == true && block_state.r#west == true && block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == true { return 11562; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#west == false && block_state.r#south == false && block_state.r#waterlogged == false { return 11569; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == true { return 11576; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#east == true { return 11561; }
        if block_state.r#south == false && block_state.r#north == true && block_state.r#west == true && block_state.r#east == true && block_state.r#waterlogged == false { return 11552; }
        if block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#east == true && block_state.r#west == false && block_state.r#north == false { return 11559; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#east == false { return 11573; }
        if block_state.r#west == false && block_state.r#north == false && block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == false { return 11577; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == false { return 11575; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#north == false && block_state.r#west == true && block_state.r#waterlogged == true { return 11554; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#east == true { return 11560; }
        if block_state.r#west == true && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#north == false { return 11570; }
        if block_state.r#south == false && block_state.r#east == true && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == true { return 11558; }
        if block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#east == true && block_state.r#south == true && block_state.r#west == true { return 11548; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == false { return 11551; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == true { return 11556; }
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#west == false && block_state.r#north == true { return 11547; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#south == true && block_state.r#east == true && block_state.r#north == false { return 11557; }
        if block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#east == false && block_state.r#south == true && block_state.r#west == false { return 11563; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#west == true { return 11564; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#west == false { return 11565; }
        if block_state.r#south == false && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#east == false { return 11566; }
        if block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#south == false && block_state.r#west == false { return 11567; }
        if block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#west == false && block_state.r#north == false && block_state.r#south == true { return 11571; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == true { return 11574; }
        if block_state.r#north == false && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#east == true { return 11555; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == true && block_state.r#south == false { return 11553; }
        if block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#east == true && block_state.r#north == true && block_state.r#west == true { return 11546; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#west == true { return 11568; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11550 {
            return Some(CyanStainedGlassPane {
                r#south: false,
                r#east: true,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11549 {
            return Some(CyanStainedGlassPane {
                r#east: true,
                r#waterlogged: false,
                r#north: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11572 {
            return Some(CyanStainedGlassPane {
                r#east: false,
                r#north: false,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11562 {
            return Some(CyanStainedGlassPane {
                r#south: true,
                r#west: true,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11569 {
            return Some(CyanStainedGlassPane {
                r#north: true,
                r#east: false,
                r#west: false,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11576 {
            return Some(CyanStainedGlassPane {
                r#east: false,
                r#south: false,
                r#north: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11561 {
            return Some(CyanStainedGlassPane {
                r#north: false,
                r#south: false,
                r#waterlogged: false,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 11552 {
            return Some(CyanStainedGlassPane {
                r#south: false,
                r#north: true,
                r#west: true,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11559 {
            return Some(CyanStainedGlassPane {
                r#waterlogged: true,
                r#south: false,
                r#east: true,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 11573 {
            return Some(CyanStainedGlassPane {
                r#north: false,
                r#south: true,
                r#west: false,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 11577 {
            return Some(CyanStainedGlassPane {
                r#west: false,
                r#north: false,
                r#east: false,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11575 {
            return Some(CyanStainedGlassPane {
                r#north: false,
                r#south: false,
                r#east: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 11554 {
            return Some(CyanStainedGlassPane {
                r#south: true,
                r#east: true,
                r#north: false,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11560 {
            return Some(CyanStainedGlassPane {
                r#west: true,
                r#north: false,
                r#south: false,
                r#waterlogged: false,
                r#east: true,
            });
        }
        if state_id == 11570 {
            return Some(CyanStainedGlassPane {
                r#west: true,
                r#east: false,
                r#waterlogged: true,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 11558 {
            return Some(CyanStainedGlassPane {
                r#south: false,
                r#east: true,
                r#north: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11548 {
            return Some(CyanStainedGlassPane {
                r#waterlogged: false,
                r#north: true,
                r#east: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 11551 {
            return Some(CyanStainedGlassPane {
                r#north: true,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 11556 {
            return Some(CyanStainedGlassPane {
                r#east: true,
                r#south: true,
                r#north: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11547 {
            return Some(CyanStainedGlassPane {
                r#east: true,
                r#waterlogged: true,
                r#south: true,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 11557 {
            return Some(CyanStainedGlassPane {
                r#waterlogged: false,
                r#west: false,
                r#south: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 11563 {
            return Some(CyanStainedGlassPane {
                r#waterlogged: true,
                r#north: true,
                r#east: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 11564 {
            return Some(CyanStainedGlassPane {
                r#south: true,
                r#north: true,
                r#east: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11565 {
            return Some(CyanStainedGlassPane {
                r#east: false,
                r#north: true,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11566 {
            return Some(CyanStainedGlassPane {
                r#south: false,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 11567 {
            return Some(CyanStainedGlassPane {
                r#north: true,
                r#waterlogged: true,
                r#east: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 11571 {
            return Some(CyanStainedGlassPane {
                r#waterlogged: true,
                r#east: false,
                r#west: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 11574 {
            return Some(CyanStainedGlassPane {
                r#east: false,
                r#north: false,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11555 {
            return Some(CyanStainedGlassPane {
                r#north: false,
                r#west: false,
                r#waterlogged: true,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 11553 {
            return Some(CyanStainedGlassPane {
                r#east: true,
                r#waterlogged: false,
                r#west: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 11546 {
            return Some(CyanStainedGlassPane {
                r#waterlogged: true,
                r#south: true,
                r#east: true,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 11568 {
            return Some(CyanStainedGlassPane {
                r#east: false,
                r#south: false,
                r#waterlogged: false,
                r#north: true,
                r#west: true,
            });
        }
        return None;
    }
}

