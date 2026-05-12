use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimeStainedGlassPane {
    pub south: bool,
    pub west: bool,
    pub north: bool,
    pub waterlogged: bool,
    pub east: bool,
}


impl BlockState for LimeStainedGlassPane {
    fn to_id(self) -> i32 {
        if block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#north == false && block_state.r#south == true { return 11444; }
        if block_state.r#north == true && block_state.r#west == true && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#south == true { return 11420; }
        if block_state.r#south == false && block_state.r#west == true && block_state.r#east == true && block_state.r#north == true && block_state.r#waterlogged == true { return 11422; }
        if block_state.r#east == true && block_state.r#west == false && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#south == true { return 11429; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == false { return 11430; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#west == false { return 11439; }
        if block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#west == true && block_state.r#east == false { return 11446; }
        if block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#east == false && block_state.r#south == false { return 11448; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#south == false { return 11449; }
        if block_state.r#south == false && block_state.r#east == true && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#north == false { return 11431; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#south == true && block_state.r#west == false && block_state.r#waterlogged == false { return 11445; }
        if block_state.r#west == false && block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#south == true { return 11421; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == true { return 11424; }
        if block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#west == false && block_state.r#south == true { return 11427; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#west == true { return 11426; }
        if block_state.r#west == false && block_state.r#east == false && block_state.r#north == true && block_state.r#south == true && block_state.r#waterlogged == false { return 11437; }
        if block_state.r#south == false && block_state.r#north == true && block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#east == false { return 11440; }
        if block_state.r#south == true && block_state.r#east == false && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#north == false { return 11443; }
        if block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#south == false && block_state.r#west == true { return 11432; }
        if block_state.r#east == false && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#south == false { return 11441; }
        if block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#east == true && block_state.r#north == true { return 11419; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#north == true { return 11436; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == true { return 11418; }
        if block_state.r#north == true && block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#south == false { return 11438; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == false { return 11433; }
        if block_state.r#west == true && block_state.r#south == true && block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == true { return 11442; }
        if block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#north == false && block_state.r#south == false && block_state.r#west == false { return 11447; }
        if block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#west == false && block_state.r#east == true && block_state.r#north == true { return 11425; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == true { return 11423; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#west == true && block_state.r#south == true && block_state.r#waterlogged == true { return 11434; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#south == true { return 11428; }
        if block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#west == false && block_state.r#north == true { return 11435; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11444 {
            return Some(LimeStainedGlassPane {
                r#east: false,
                r#waterlogged: false,
                r#west: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 11420 {
            return Some(LimeStainedGlassPane {
                r#north: true,
                r#west: true,
                r#east: true,
                r#waterlogged: false,
                r#south: true,
            });
        }
        if state_id == 11422 {
            return Some(LimeStainedGlassPane {
                r#south: false,
                r#west: true,
                r#east: true,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11429 {
            return Some(LimeStainedGlassPane {
                r#east: true,
                r#west: false,
                r#north: false,
                r#waterlogged: false,
                r#south: true,
            });
        }
        if state_id == 11430 {
            return Some(LimeStainedGlassPane {
                r#north: false,
                r#east: true,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 11439 {
            return Some(LimeStainedGlassPane {
                r#east: false,
                r#south: false,
                r#waterlogged: true,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 11446 {
            return Some(LimeStainedGlassPane {
                r#south: false,
                r#waterlogged: true,
                r#north: false,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 11448 {
            return Some(LimeStainedGlassPane {
                r#north: false,
                r#waterlogged: false,
                r#west: true,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 11449 {
            return Some(LimeStainedGlassPane {
                r#east: false,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 11431 {
            return Some(LimeStainedGlassPane {
                r#south: false,
                r#east: true,
                r#west: false,
                r#waterlogged: true,
                r#north: false,
            });
        }
        if state_id == 11445 {
            return Some(LimeStainedGlassPane {
                r#east: false,
                r#north: false,
                r#south: true,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11421 {
            return Some(LimeStainedGlassPane {
                r#west: false,
                r#north: true,
                r#waterlogged: false,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 11424 {
            return Some(LimeStainedGlassPane {
                r#north: true,
                r#east: true,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11427 {
            return Some(LimeStainedGlassPane {
                r#north: false,
                r#waterlogged: true,
                r#east: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11426 {
            return Some(LimeStainedGlassPane {
                r#east: true,
                r#south: true,
                r#waterlogged: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 11437 {
            return Some(LimeStainedGlassPane {
                r#west: false,
                r#east: false,
                r#north: true,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11440 {
            return Some(LimeStainedGlassPane {
                r#south: false,
                r#north: true,
                r#west: true,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 11443 {
            return Some(LimeStainedGlassPane {
                r#south: true,
                r#east: false,
                r#west: false,
                r#waterlogged: true,
                r#north: false,
            });
        }
        if state_id == 11432 {
            return Some(LimeStainedGlassPane {
                r#north: false,
                r#waterlogged: false,
                r#east: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 11441 {
            return Some(LimeStainedGlassPane {
                r#east: false,
                r#west: false,
                r#waterlogged: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 11419 {
            return Some(LimeStainedGlassPane {
                r#south: true,
                r#waterlogged: true,
                r#west: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 11436 {
            return Some(LimeStainedGlassPane {
                r#east: false,
                r#west: true,
                r#south: true,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 11418 {
            return Some(LimeStainedGlassPane {
                r#east: true,
                r#south: true,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11438 {
            return Some(LimeStainedGlassPane {
                r#north: true,
                r#west: true,
                r#waterlogged: true,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 11433 {
            return Some(LimeStainedGlassPane {
                r#north: false,
                r#east: true,
                r#south: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11442 {
            return Some(LimeStainedGlassPane {
                r#west: true,
                r#south: true,
                r#east: false,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11447 {
            return Some(LimeStainedGlassPane {
                r#waterlogged: true,
                r#east: false,
                r#north: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 11425 {
            return Some(LimeStainedGlassPane {
                r#waterlogged: false,
                r#south: false,
                r#west: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 11423 {
            return Some(LimeStainedGlassPane {
                r#north: true,
                r#east: true,
                r#south: false,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11434 {
            return Some(LimeStainedGlassPane {
                r#north: true,
                r#east: false,
                r#west: true,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11428 {
            return Some(LimeStainedGlassPane {
                r#east: true,
                r#north: false,
                r#waterlogged: false,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 11435 {
            return Some(LimeStainedGlassPane {
                r#south: true,
                r#waterlogged: true,
                r#east: false,
                r#west: false,
                r#north: true,
            });
        }
        return None;
    }
}

