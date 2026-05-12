use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vine {
    pub east: bool,
    pub north: bool,
    pub up: bool,
    pub south: bool,
    pub west: bool,
}


impl BlockState for Vine {
    fn to_id(self) -> i32 {
        if block_state.r#west == false && block_state.r#south == false && block_state.r#east == false && block_state.r#up == false && block_state.r#north == false { return 8188; }
        if block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#north == true && block_state.r#east == false { return 8174; }
        if block_state.r#up == true && block_state.r#east == true && block_state.r#north == true && block_state.r#west == false && block_state.r#south == false { return 8162; }
        if block_state.r#north == false && block_state.r#west == false && block_state.r#south == false && block_state.r#up == true && block_state.r#east == true { return 8170; }
        if block_state.r#up == true && block_state.r#west == true && block_state.r#north == false && block_state.r#east == true && block_state.r#south == true { return 8165; }
        if block_state.r#up == false && block_state.r#north == true && block_state.r#south == false && block_state.r#east == true && block_state.r#west == true { return 8163; }
        if block_state.r#up == false && block_state.r#east == false && block_state.r#north == true && block_state.r#south == false && block_state.r#west == true { return 8179; }
        if block_state.r#north == true && block_state.r#west == false && block_state.r#east == true && block_state.r#up == false && block_state.r#south == true { return 8160; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#up == false && block_state.r#west == true && block_state.r#south == true { return 8175; }
        if block_state.r#west == false && block_state.r#south == false && block_state.r#up == false && block_state.r#east == true && block_state.r#north == false { return 8172; }
        if block_state.r#up == false && block_state.r#south == false && block_state.r#west == true && block_state.r#north == false && block_state.r#east == false { return 8187; }
        if block_state.r#up == false && block_state.r#east == false && block_state.r#west == false && block_state.r#south == false && block_state.r#north == true { return 8180; }
        if block_state.r#west == false && block_state.r#south == false && block_state.r#east == true && block_state.r#north == true && block_state.r#up == false { return 8164; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#north == false && block_state.r#west == true && block_state.r#up == false { return 8167; }
        if block_state.r#west == false && block_state.r#up == true && block_state.r#south == true && block_state.r#east == false && block_state.r#north == false { return 8182; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#west == false && block_state.r#up == true && block_state.r#north == false { return 8186; }
        if block_state.r#west == true && block_state.r#south == false && block_state.r#east == false && block_state.r#north == false && block_state.r#up == true { return 8185; }
        if block_state.r#up == false && block_state.r#north == false && block_state.r#south == false && block_state.r#west == true && block_state.r#east == true { return 8171; }
        if block_state.r#north == true && block_state.r#up == true && block_state.r#south == true && block_state.r#west == false && block_state.r#east == true { return 8158; }
        if block_state.r#north == false && block_state.r#west == false && block_state.r#up == false && block_state.r#south == true && block_state.r#east == true { return 8168; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#north == true && block_state.r#west == true && block_state.r#up == true { return 8157; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#east == false && block_state.r#west == true && block_state.r#up == false { return 8183; }
        if block_state.r#west == true && block_state.r#north == true && block_state.r#east == true && block_state.r#south == true && block_state.r#up == false { return 8159; }
        if block_state.r#up == false && block_state.r#west == false && block_state.r#east == false && block_state.r#north == true && block_state.r#south == true { return 8176; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#north == false && block_state.r#up == true && block_state.r#west == true { return 8181; }
        if block_state.r#east == true && block_state.r#north == true && block_state.r#south == false && block_state.r#up == true && block_state.r#west == true { return 8161; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#up == true && block_state.r#east == true && block_state.r#north == false { return 8166; }
        if block_state.r#south == true && block_state.r#east == false && block_state.r#west == true && block_state.r#up == true && block_state.r#north == true { return 8173; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#north == false && block_state.r#west == false && block_state.r#up == false { return 8184; }
        if block_state.r#south == false && block_state.r#west == true && block_state.r#up == true && block_state.r#north == true && block_state.r#east == false { return 8177; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#east == true && block_state.r#south == false && block_state.r#up == true { return 8169; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#east == false && block_state.r#north == true && block_state.r#up == true { return 8178; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8188 {
            return Some(Vine {
                r#west: false,
                r#south: false,
                r#east: false,
                r#up: false,
                r#north: false,
            });
        }
        if state_id == 8174 {
            return Some(Vine {
                r#south: true,
                r#up: true,
                r#west: false,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 8162 {
            return Some(Vine {
                r#up: true,
                r#east: true,
                r#north: true,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 8170 {
            return Some(Vine {
                r#north: false,
                r#west: false,
                r#south: false,
                r#up: true,
                r#east: true,
            });
        }
        if state_id == 8165 {
            return Some(Vine {
                r#up: true,
                r#west: true,
                r#north: false,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 8163 {
            return Some(Vine {
                r#up: false,
                r#north: true,
                r#south: false,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 8179 {
            return Some(Vine {
                r#up: false,
                r#east: false,
                r#north: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 8160 {
            return Some(Vine {
                r#north: true,
                r#west: false,
                r#east: true,
                r#up: false,
                r#south: true,
            });
        }
        if state_id == 8175 {
            return Some(Vine {
                r#east: false,
                r#north: true,
                r#up: false,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 8172 {
            return Some(Vine {
                r#west: false,
                r#south: false,
                r#up: false,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 8187 {
            return Some(Vine {
                r#up: false,
                r#south: false,
                r#west: true,
                r#north: false,
                r#east: false,
            });
        }
        if state_id == 8180 {
            return Some(Vine {
                r#up: false,
                r#east: false,
                r#west: false,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 8164 {
            return Some(Vine {
                r#west: false,
                r#south: false,
                r#east: true,
                r#north: true,
                r#up: false,
            });
        }
        if state_id == 8167 {
            return Some(Vine {
                r#east: true,
                r#south: true,
                r#north: false,
                r#west: true,
                r#up: false,
            });
        }
        if state_id == 8182 {
            return Some(Vine {
                r#west: false,
                r#up: true,
                r#south: true,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 8186 {
            return Some(Vine {
                r#south: false,
                r#east: false,
                r#west: false,
                r#up: true,
                r#north: false,
            });
        }
        if state_id == 8185 {
            return Some(Vine {
                r#west: true,
                r#south: false,
                r#east: false,
                r#north: false,
                r#up: true,
            });
        }
        if state_id == 8171 {
            return Some(Vine {
                r#up: false,
                r#north: false,
                r#south: false,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 8158 {
            return Some(Vine {
                r#north: true,
                r#up: true,
                r#south: true,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 8168 {
            return Some(Vine {
                r#north: false,
                r#west: false,
                r#up: false,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 8157 {
            return Some(Vine {
                r#south: true,
                r#east: true,
                r#north: true,
                r#west: true,
                r#up: true,
            });
        }
        if state_id == 8183 {
            return Some(Vine {
                r#north: false,
                r#south: true,
                r#east: false,
                r#west: true,
                r#up: false,
            });
        }
        if state_id == 8159 {
            return Some(Vine {
                r#west: true,
                r#north: true,
                r#east: true,
                r#south: true,
                r#up: false,
            });
        }
        if state_id == 8176 {
            return Some(Vine {
                r#up: false,
                r#west: false,
                r#east: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 8181 {
            return Some(Vine {
                r#east: false,
                r#south: true,
                r#north: false,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 8161 {
            return Some(Vine {
                r#east: true,
                r#north: true,
                r#south: false,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 8166 {
            return Some(Vine {
                r#west: false,
                r#south: true,
                r#up: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 8173 {
            return Some(Vine {
                r#south: true,
                r#east: false,
                r#west: true,
                r#up: true,
                r#north: true,
            });
        }
        if state_id == 8184 {
            return Some(Vine {
                r#east: false,
                r#south: true,
                r#north: false,
                r#west: false,
                r#up: false,
            });
        }
        if state_id == 8177 {
            return Some(Vine {
                r#south: false,
                r#west: true,
                r#up: true,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 8169 {
            return Some(Vine {
                r#north: false,
                r#west: true,
                r#east: true,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 8178 {
            return Some(Vine {
                r#south: false,
                r#west: false,
                r#east: false,
                r#north: true,
                r#up: true,
            });
        }
        return None;
    }
}

