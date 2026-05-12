use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCopperBars {
    pub east: bool,
    pub south: bool,
    pub north: bool,
    pub waterlogged: bool,
    pub west: bool,
}


impl BlockState for WaxedWeatheredCopperBars {
    fn to_id(self) -> i32 {
        if block_state.r#south == true && block_state.r#west == true && block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == false { return 7999; }
        if block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#west == true && block_state.r#north == true { return 7987; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == false { return 8012; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == true && block_state.r#south == true && block_state.r#east == true { return 7984; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#north == false { return 7990; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#north == false && block_state.r#east == true && block_state.r#waterlogged == false { return 7992; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#east == true && block_state.r#west == false && block_state.r#waterlogged == false { return 7996; }
        if block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#west == false && block_state.r#south == true && block_state.r#north == true { return 7998; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#east == false { return 8009; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#north == true && block_state.r#west == false && block_state.r#waterlogged == false { return 8004; }
        if block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#east == true && block_state.r#south == false { return 7986; }
        if block_state.r#west == true && block_state.r#south == false && block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == false { return 8003; }
        if block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#north == false && block_state.r#south == true { return 7991; }
        if block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == true && block_state.r#east == true { return 7988; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#north == false { return 8010; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#north == true && block_state.r#west == false && block_state.r#waterlogged == false { return 8000; }
        if block_state.r#south == true && block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#east == true { return 7989; }
        if block_state.r#north == true && block_state.r#west == false && block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == true { return 8002; }
        if block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#north == true && block_state.r#south == true { return 7997; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#south == true { return 8005; }
        if block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#south == true && block_state.r#west == false { return 7982; }
        if block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#north == true && block_state.r#east == true && block_state.r#west == true { return 7981; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#south == false { return 7995; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == false { return 7994; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#north == false { return 8008; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#west == false { return 8006; }
        if block_state.r#west == true && block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#south == false { return 8011; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == true { return 7993; }
        if block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#west == true && block_state.r#north == true { return 7985; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == false && block_state.r#north == true { return 8001; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#south == true { return 8007; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#north == true && block_state.r#south == true && block_state.r#waterlogged == false { return 7983; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7999 {
            return Some(WaxedWeatheredCopperBars {
                r#south: true,
                r#west: true,
                r#east: false,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7987 {
            return Some(WaxedWeatheredCopperBars {
                r#south: false,
                r#waterlogged: false,
                r#east: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 8012 {
            return Some(WaxedWeatheredCopperBars {
                r#north: false,
                r#east: false,
                r#south: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 7984 {
            return Some(WaxedWeatheredCopperBars {
                r#waterlogged: false,
                r#west: false,
                r#north: true,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 7990 {
            return Some(WaxedWeatheredCopperBars {
                r#west: false,
                r#south: true,
                r#east: true,
                r#waterlogged: true,
                r#north: false,
            });
        }
        if state_id == 7992 {
            return Some(WaxedWeatheredCopperBars {
                r#west: false,
                r#south: true,
                r#north: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7996 {
            return Some(WaxedWeatheredCopperBars {
                r#north: false,
                r#south: false,
                r#east: true,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7998 {
            return Some(WaxedWeatheredCopperBars {
                r#waterlogged: true,
                r#east: false,
                r#west: false,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 8009 {
            return Some(WaxedWeatheredCopperBars {
                r#north: false,
                r#west: true,
                r#south: false,
                r#waterlogged: true,
                r#east: false,
            });
        }
        if state_id == 8004 {
            return Some(WaxedWeatheredCopperBars {
                r#east: false,
                r#south: false,
                r#north: true,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7986 {
            return Some(WaxedWeatheredCopperBars {
                r#west: false,
                r#waterlogged: true,
                r#north: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 8003 {
            return Some(WaxedWeatheredCopperBars {
                r#west: true,
                r#south: false,
                r#east: false,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7991 {
            return Some(WaxedWeatheredCopperBars {
                r#west: true,
                r#waterlogged: false,
                r#east: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 7988 {
            return Some(WaxedWeatheredCopperBars {
                r#south: false,
                r#waterlogged: false,
                r#west: false,
                r#north: true,
                r#east: true,
            });
        }
        if state_id == 8010 {
            return Some(WaxedWeatheredCopperBars {
                r#east: false,
                r#south: false,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 8000 {
            return Some(WaxedWeatheredCopperBars {
                r#east: false,
                r#south: true,
                r#north: true,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7989 {
            return Some(WaxedWeatheredCopperBars {
                r#south: true,
                r#west: true,
                r#waterlogged: true,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 8002 {
            return Some(WaxedWeatheredCopperBars {
                r#north: true,
                r#west: false,
                r#east: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7997 {
            return Some(WaxedWeatheredCopperBars {
                r#west: true,
                r#waterlogged: true,
                r#east: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 8005 {
            return Some(WaxedWeatheredCopperBars {
                r#north: false,
                r#east: false,
                r#west: true,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 7982 {
            return Some(WaxedWeatheredCopperBars {
                r#north: true,
                r#waterlogged: true,
                r#east: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 7981 {
            return Some(WaxedWeatheredCopperBars {
                r#waterlogged: true,
                r#south: true,
                r#north: true,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 7995 {
            return Some(WaxedWeatheredCopperBars {
                r#west: true,
                r#north: false,
                r#waterlogged: false,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 7994 {
            return Some(WaxedWeatheredCopperBars {
                r#east: true,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 8008 {
            return Some(WaxedWeatheredCopperBars {
                r#west: false,
                r#south: true,
                r#waterlogged: false,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 8006 {
            return Some(WaxedWeatheredCopperBars {
                r#east: false,
                r#north: false,
                r#south: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 8011 {
            return Some(WaxedWeatheredCopperBars {
                r#west: true,
                r#east: false,
                r#north: false,
                r#waterlogged: false,
                r#south: false,
            });
        }
        if state_id == 7993 {
            return Some(WaxedWeatheredCopperBars {
                r#east: true,
                r#north: false,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 7985 {
            return Some(WaxedWeatheredCopperBars {
                r#south: false,
                r#waterlogged: true,
                r#east: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 8001 {
            return Some(WaxedWeatheredCopperBars {
                r#east: false,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 8007 {
            return Some(WaxedWeatheredCopperBars {
                r#east: false,
                r#north: false,
                r#west: true,
                r#waterlogged: false,
                r#south: true,
            });
        }
        if state_id == 7983 {
            return Some(WaxedWeatheredCopperBars {
                r#west: true,
                r#east: true,
                r#north: true,
                r#south: true,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

