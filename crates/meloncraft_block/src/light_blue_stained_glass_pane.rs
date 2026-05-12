use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightBlueStainedGlassPane {
    pub waterlogged: bool,
    pub north: bool,
    pub west: bool,
    pub south: bool,
    pub east: bool,
}


impl BlockState for LightBlueStainedGlassPane {
    fn to_id(self) -> i32 {
        if block_state.r#east == true && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#north == true { return 11355; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == false { return 11364; }
        if block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#east == true && block_state.r#south == false { return 11358; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#west == false { return 11359; }
        if block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#north == false && block_state.r#west == true && block_state.r#south == false { return 11368; }
        if block_state.r#south == true && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#west == true { return 11378; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#west == false && block_state.r#north == true { return 11357; }
        if block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#south == true && block_state.r#east == true { return 11363; }
        if block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#north == false && block_state.r#south == false && block_state.r#west == true { return 11384; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#east == false { return 11380; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#west == false { return 11373; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#west == true { return 11356; }
        if block_state.r#west == false && block_state.r#north == false && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#south == true { return 11379; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#east == true { return 11367; }
        if block_state.r#west == true && block_state.r#south == true && block_state.r#north == true && block_state.r#east == false && block_state.r#waterlogged == true { return 11370; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == false && block_state.r#east == true && block_state.r#south == false { return 11369; }
        if block_state.r#south == false && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#east == false { return 11383; }
        if block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#west == true && block_state.r#north == true { return 11360; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#east == false { return 11377; }
        if block_state.r#west == true && block_state.r#north == true && block_state.r#east == false && block_state.r#south == true && block_state.r#waterlogged == false { return 11372; }
        if block_state.r#south == true && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#west == false { return 11365; }
        if block_state.r#south == false && block_state.r#east == true && block_state.r#north == true && block_state.r#west == false && block_state.r#waterlogged == false { return 11361; }
        if block_state.r#west == false && block_state.r#east == false && block_state.r#north == true && block_state.r#south == true && block_state.r#waterlogged == true { return 11371; }
        if block_state.r#east == true && block_state.r#north == true && block_state.r#south == true && block_state.r#west == true && block_state.r#waterlogged == true { return 11354; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#west == true { return 11362; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#west == true { return 11374; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == false { return 11375; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == false && block_state.r#south == true && block_state.r#east == false { return 11381; }
        if block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#east == true && block_state.r#west == true { return 11366; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#south == false && block_state.r#north == false && block_state.r#waterlogged == true { return 11382; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#west == true && block_state.r#north == true && block_state.r#waterlogged == false { return 11376; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#west == false && block_state.r#east == false && block_state.r#waterlogged == false { return 11385; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11355 {
            return Some(LightBlueStainedGlassPane {
                r#east: true,
                r#west: false,
                r#waterlogged: true,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 11364 {
            return Some(LightBlueStainedGlassPane {
                r#west: true,
                r#east: true,
                r#north: false,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11358 {
            return Some(LightBlueStainedGlassPane {
                r#north: true,
                r#waterlogged: true,
                r#west: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 11359 {
            return Some(LightBlueStainedGlassPane {
                r#east: true,
                r#south: false,
                r#waterlogged: true,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 11368 {
            return Some(LightBlueStainedGlassPane {
                r#waterlogged: false,
                r#east: true,
                r#north: false,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 11378 {
            return Some(LightBlueStainedGlassPane {
                r#south: true,
                r#north: false,
                r#waterlogged: true,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 11357 {
            return Some(LightBlueStainedGlassPane {
                r#east: true,
                r#waterlogged: false,
                r#south: true,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 11363 {
            return Some(LightBlueStainedGlassPane {
                r#west: false,
                r#waterlogged: true,
                r#north: false,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 11384 {
            return Some(LightBlueStainedGlassPane {
                r#waterlogged: false,
                r#east: false,
                r#north: false,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 11380 {
            return Some(LightBlueStainedGlassPane {
                r#west: true,
                r#north: false,
                r#waterlogged: false,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 11373 {
            return Some(LightBlueStainedGlassPane {
                r#east: false,
                r#north: true,
                r#waterlogged: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 11356 {
            return Some(LightBlueStainedGlassPane {
                r#north: true,
                r#east: true,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11379 {
            return Some(LightBlueStainedGlassPane {
                r#west: false,
                r#north: false,
                r#east: false,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 11367 {
            return Some(LightBlueStainedGlassPane {
                r#north: false,
                r#south: false,
                r#west: false,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 11370 {
            return Some(LightBlueStainedGlassPane {
                r#west: true,
                r#south: true,
                r#north: true,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11369 {
            return Some(LightBlueStainedGlassPane {
                r#waterlogged: false,
                r#west: false,
                r#north: false,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 11383 {
            return Some(LightBlueStainedGlassPane {
                r#south: false,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 11360 {
            return Some(LightBlueStainedGlassPane {
                r#south: false,
                r#waterlogged: false,
                r#east: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 11377 {
            return Some(LightBlueStainedGlassPane {
                r#north: true,
                r#south: false,
                r#west: false,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 11372 {
            return Some(LightBlueStainedGlassPane {
                r#west: true,
                r#north: true,
                r#east: false,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11365 {
            return Some(LightBlueStainedGlassPane {
                r#south: true,
                r#north: false,
                r#waterlogged: false,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 11361 {
            return Some(LightBlueStainedGlassPane {
                r#south: false,
                r#east: true,
                r#north: true,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11371 {
            return Some(LightBlueStainedGlassPane {
                r#west: false,
                r#east: false,
                r#north: true,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11354 {
            return Some(LightBlueStainedGlassPane {
                r#east: true,
                r#north: true,
                r#south: true,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11362 {
            return Some(LightBlueStainedGlassPane {
                r#east: true,
                r#south: true,
                r#waterlogged: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 11374 {
            return Some(LightBlueStainedGlassPane {
                r#east: false,
                r#north: true,
                r#waterlogged: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 11375 {
            return Some(LightBlueStainedGlassPane {
                r#east: false,
                r#south: false,
                r#north: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 11381 {
            return Some(LightBlueStainedGlassPane {
                r#waterlogged: false,
                r#west: false,
                r#north: false,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 11366 {
            return Some(LightBlueStainedGlassPane {
                r#south: false,
                r#waterlogged: true,
                r#north: false,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 11382 {
            return Some(LightBlueStainedGlassPane {
                r#east: false,
                r#west: true,
                r#south: false,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11376 {
            return Some(LightBlueStainedGlassPane {
                r#east: false,
                r#south: false,
                r#west: true,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11385 {
            return Some(LightBlueStainedGlassPane {
                r#north: false,
                r#south: false,
                r#west: false,
                r#east: false,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

