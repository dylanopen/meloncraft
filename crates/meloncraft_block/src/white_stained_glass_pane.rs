use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteStainedGlassPane {
    pub waterlogged: bool,
    pub west: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
}


impl BlockState for WhiteStainedGlassPane {
    fn to_id(self) -> i32 {
        if block_state.r#north == true && block_state.r#west == true && block_state.r#east == true && block_state.r#south == false && block_state.r#waterlogged == false { return 11264; }
        if block_state.r#east == true && block_state.r#west == false && block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == false { return 11269; }
        if block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == false && block_state.r#east == false && block_state.r#north == false { return 11286; }
        if block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#north == true && block_state.r#west == false && block_state.r#south == false { return 11279; }
        if block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#south == false && block_state.r#north == true && block_state.r#west == true { return 11280; }
        if block_state.r#west == false && block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#south == false { return 11287; }
        if block_state.r#west == true && block_state.r#south == false && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#east == true { return 11262; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == false { return 11288; }
        if block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#north == false && block_state.r#east == false && block_state.r#west == false { return 11285; }
        if block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#east == false && block_state.r#south == true { return 11276; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == false { return 11281; }
        if block_state.r#east == true && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == true { return 11259; }
        if block_state.r#south == true && block_state.r#north == false && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#west == true { return 11284; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#west == true { return 11266; }
        if block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#west == false && block_state.r#north == false && block_state.r#south == false { return 11289; }
        if block_state.r#north == false && block_state.r#west == false && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#south == false { return 11273; }
        if block_state.r#north == true && block_state.r#south == true && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == true { return 11260; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#north == true { return 11265; }
        if block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#east == false && block_state.r#south == false { return 11278; }
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == false && block_state.r#north == false { return 11270; }
        if block_state.r#west == true && block_state.r#north == true && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#south == true { return 11258; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#west == false && block_state.r#north == false && block_state.r#waterlogged == true { return 11267; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#west == true { return 11272; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == false { return 11271; }
        if block_state.r#north == true && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#east == false { return 11274; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#north == false { return 11282; }
        if block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#south == false && block_state.r#west == false && block_state.r#east == true { return 11263; }
        if block_state.r#south == true && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#north == true { return 11261; }
        if block_state.r#east == false && block_state.r#west == false && block_state.r#south == true && block_state.r#north == false && block_state.r#waterlogged == true { return 11283; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#south == true && block_state.r#west == false && block_state.r#waterlogged == true { return 11275; }
        if block_state.r#west == false && block_state.r#north == true && block_state.r#east == false && block_state.r#south == true && block_state.r#waterlogged == false { return 11277; }
        if block_state.r#west == true && block_state.r#south == true && block_state.r#north == false && block_state.r#east == true && block_state.r#waterlogged == false { return 11268; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11264 {
            return Some(WhiteStainedGlassPane {
                r#north: true,
                r#west: true,
                r#east: true,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11269 {
            return Some(WhiteStainedGlassPane {
                r#east: true,
                r#west: false,
                r#north: false,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11286 {
            return Some(WhiteStainedGlassPane {
                r#waterlogged: true,
                r#west: true,
                r#south: false,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 11279 {
            return Some(WhiteStainedGlassPane {
                r#waterlogged: true,
                r#east: false,
                r#north: true,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 11280 {
            return Some(WhiteStainedGlassPane {
                r#waterlogged: false,
                r#east: false,
                r#south: false,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 11287 {
            return Some(WhiteStainedGlassPane {
                r#west: false,
                r#east: false,
                r#north: false,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 11262 {
            return Some(WhiteStainedGlassPane {
                r#west: true,
                r#south: false,
                r#north: true,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 11288 {
            return Some(WhiteStainedGlassPane {
                r#west: true,
                r#north: false,
                r#east: false,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11285 {
            return Some(WhiteStainedGlassPane {
                r#waterlogged: false,
                r#south: true,
                r#north: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 11276 {
            return Some(WhiteStainedGlassPane {
                r#north: true,
                r#waterlogged: false,
                r#west: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 11281 {
            return Some(WhiteStainedGlassPane {
                r#east: false,
                r#north: true,
                r#south: false,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11259 {
            return Some(WhiteStainedGlassPane {
                r#east: true,
                r#north: true,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11284 {
            return Some(WhiteStainedGlassPane {
                r#south: true,
                r#north: false,
                r#east: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11266 {
            return Some(WhiteStainedGlassPane {
                r#south: true,
                r#east: true,
                r#waterlogged: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 11289 {
            return Some(WhiteStainedGlassPane {
                r#waterlogged: false,
                r#east: false,
                r#west: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 11273 {
            return Some(WhiteStainedGlassPane {
                r#north: false,
                r#west: false,
                r#east: true,
                r#waterlogged: false,
                r#south: false,
            });
        }
        if state_id == 11260 {
            return Some(WhiteStainedGlassPane {
                r#north: true,
                r#south: true,
                r#east: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11265 {
            return Some(WhiteStainedGlassPane {
                r#east: true,
                r#south: false,
                r#west: false,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 11278 {
            return Some(WhiteStainedGlassPane {
                r#west: true,
                r#waterlogged: true,
                r#north: true,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 11270 {
            return Some(WhiteStainedGlassPane {
                r#east: true,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
                r#north: false,
            });
        }
        if state_id == 11258 {
            return Some(WhiteStainedGlassPane {
                r#west: true,
                r#north: true,
                r#east: true,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 11267 {
            return Some(WhiteStainedGlassPane {
                r#east: true,
                r#south: true,
                r#west: false,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11272 {
            return Some(WhiteStainedGlassPane {
                r#east: true,
                r#south: false,
                r#waterlogged: false,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 11271 {
            return Some(WhiteStainedGlassPane {
                r#north: false,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 11274 {
            return Some(WhiteStainedGlassPane {
                r#north: true,
                r#south: true,
                r#waterlogged: true,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 11282 {
            return Some(WhiteStainedGlassPane {
                r#east: false,
                r#south: true,
                r#waterlogged: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 11263 {
            return Some(WhiteStainedGlassPane {
                r#waterlogged: true,
                r#north: true,
                r#south: false,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 11261 {
            return Some(WhiteStainedGlassPane {
                r#south: true,
                r#west: false,
                r#waterlogged: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 11283 {
            return Some(WhiteStainedGlassPane {
                r#east: false,
                r#west: false,
                r#south: true,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11275 {
            return Some(WhiteStainedGlassPane {
                r#east: false,
                r#north: true,
                r#south: true,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11277 {
            return Some(WhiteStainedGlassPane {
                r#west: false,
                r#north: true,
                r#east: false,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11268 {
            return Some(WhiteStainedGlassPane {
                r#west: true,
                r#south: true,
                r#north: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

