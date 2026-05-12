use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleStainedGlassPane {
    pub north: bool,
    pub west: bool,
    pub waterlogged: bool,
    pub east: bool,
    pub south: bool,
}


impl BlockState for PurpleStainedGlassPane {
    fn to_id(self) -> i32 {
        if block_state.r#north == false && block_state.r#south == false && block_state.r#east == true && block_state.r#west == false && block_state.r#waterlogged == false { return 11593; }
        if block_state.r#west == false && block_state.r#north == false && block_state.r#south == false && block_state.r#east == false && block_state.r#waterlogged == true { return 11607; }
        if block_state.r#south == false && block_state.r#north == false && block_state.r#east == true && block_state.r#west == false && block_state.r#waterlogged == true { return 11591; }
        if block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#east == true && block_state.r#north == false && block_state.r#west == true { return 11588; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#west == true && block_state.r#south == false && block_state.r#waterlogged == false { return 11608; }
        if block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#east == true && block_state.r#south == true && block_state.r#west == false { return 11587; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#east == false { return 11601; }
        if block_state.r#west == false && block_state.r#south == false && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#north == true { return 11599; }
        if block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#west == false && block_state.r#south == false && block_state.r#east == true { return 11583; }
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#south == true && block_state.r#west == true { return 11586; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#south == true { return 11604; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#west == false { return 11581; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == false { return 11606; }
        if block_state.r#west == true && block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#east == true { return 11584; }
        if block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#east == true && block_state.r#west == false { return 11585; }
        if block_state.r#south == true && block_state.r#east == false && block_state.r#west == true && block_state.r#north == false && block_state.r#waterlogged == true { return 11602; }
        if block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#south == true && block_state.r#north == true && block_state.r#east == false { return 11596; }
        if block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#north == false && block_state.r#east == false { return 11603; }
        if block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#east == true && block_state.r#west == true && block_state.r#north == false { return 11590; }
        if block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#west == true && block_state.r#north == true && block_state.r#east == false { return 11598; }
        if block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#south == true && block_state.r#east == false && block_state.r#west == false { return 11597; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#north == true && block_state.r#west == true && block_state.r#waterlogged == false { return 11580; }
        if block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#south == false && block_state.r#north == false && block_state.r#east == true { return 11592; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#north == true && block_state.r#south == true && block_state.r#waterlogged == true { return 11594; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#south == false { return 11600; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#south == true { return 11605; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == false { return 11609; }
        if block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#north == true && block_state.r#east == true && block_state.r#west == true { return 11578; }
        if block_state.r#east == true && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == true { return 11579; }
        if block_state.r#south == true && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#north == true { return 11595; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#north == true { return 11582; }
        if block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#west == false && block_state.r#north == false && block_state.r#south == true { return 11589; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11593 {
            return Some(PurpleStainedGlassPane {
                r#north: false,
                r#south: false,
                r#east: true,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11607 {
            return Some(PurpleStainedGlassPane {
                r#west: false,
                r#north: false,
                r#south: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11591 {
            return Some(PurpleStainedGlassPane {
                r#south: false,
                r#north: false,
                r#east: true,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11588 {
            return Some(PurpleStainedGlassPane {
                r#waterlogged: false,
                r#south: true,
                r#east: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 11608 {
            return Some(PurpleStainedGlassPane {
                r#east: false,
                r#north: false,
                r#west: true,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11587 {
            return Some(PurpleStainedGlassPane {
                r#waterlogged: true,
                r#north: false,
                r#east: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 11601 {
            return Some(PurpleStainedGlassPane {
                r#south: false,
                r#west: false,
                r#north: true,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 11599 {
            return Some(PurpleStainedGlassPane {
                r#west: false,
                r#south: false,
                r#east: false,
                r#waterlogged: true,
                r#north: true,
            });
        }
        if state_id == 11583 {
            return Some(PurpleStainedGlassPane {
                r#waterlogged: true,
                r#north: true,
                r#west: false,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 11586 {
            return Some(PurpleStainedGlassPane {
                r#east: true,
                r#waterlogged: true,
                r#north: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 11604 {
            return Some(PurpleStainedGlassPane {
                r#east: false,
                r#west: true,
                r#north: false,
                r#waterlogged: false,
                r#south: true,
            });
        }
        if state_id == 11581 {
            return Some(PurpleStainedGlassPane {
                r#east: true,
                r#south: true,
                r#waterlogged: false,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 11606 {
            return Some(PurpleStainedGlassPane {
                r#north: false,
                r#east: false,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 11584 {
            return Some(PurpleStainedGlassPane {
                r#west: true,
                r#north: true,
                r#waterlogged: false,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 11585 {
            return Some(PurpleStainedGlassPane {
                r#north: true,
                r#waterlogged: false,
                r#south: false,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 11602 {
            return Some(PurpleStainedGlassPane {
                r#south: true,
                r#east: false,
                r#west: true,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11596 {
            return Some(PurpleStainedGlassPane {
                r#waterlogged: false,
                r#west: true,
                r#south: true,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 11603 {
            return Some(PurpleStainedGlassPane {
                r#west: false,
                r#waterlogged: true,
                r#south: true,
                r#north: false,
                r#east: false,
            });
        }
        if state_id == 11590 {
            return Some(PurpleStainedGlassPane {
                r#waterlogged: true,
                r#south: false,
                r#east: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 11598 {
            return Some(PurpleStainedGlassPane {
                r#waterlogged: true,
                r#south: false,
                r#west: true,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 11597 {
            return Some(PurpleStainedGlassPane {
                r#waterlogged: false,
                r#north: true,
                r#south: true,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 11580 {
            return Some(PurpleStainedGlassPane {
                r#south: true,
                r#east: true,
                r#north: true,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11592 {
            return Some(PurpleStainedGlassPane {
                r#waterlogged: false,
                r#west: true,
                r#south: false,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 11594 {
            return Some(PurpleStainedGlassPane {
                r#east: false,
                r#west: true,
                r#north: true,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11600 {
            return Some(PurpleStainedGlassPane {
                r#north: true,
                r#east: false,
                r#west: true,
                r#waterlogged: false,
                r#south: false,
            });
        }
        if state_id == 11605 {
            return Some(PurpleStainedGlassPane {
                r#east: false,
                r#north: false,
                r#west: false,
                r#waterlogged: false,
                r#south: true,
            });
        }
        if state_id == 11609 {
            return Some(PurpleStainedGlassPane {
                r#east: false,
                r#north: false,
                r#south: false,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11578 {
            return Some(PurpleStainedGlassPane {
                r#waterlogged: true,
                r#south: true,
                r#north: true,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 11579 {
            return Some(PurpleStainedGlassPane {
                r#east: true,
                r#north: true,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11595 {
            return Some(PurpleStainedGlassPane {
                r#south: true,
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 11582 {
            return Some(PurpleStainedGlassPane {
                r#west: true,
                r#east: true,
                r#south: false,
                r#waterlogged: true,
                r#north: true,
            });
        }
        if state_id == 11589 {
            return Some(PurpleStainedGlassPane {
                r#waterlogged: false,
                r#east: true,
                r#west: false,
                r#north: false,
                r#south: true,
            });
        }
        return None;
    }
}

