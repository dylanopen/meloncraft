use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCopperBars {
    pub south: bool,
    pub west: bool,
    pub east: bool,
    pub waterlogged: bool,
    pub north: bool,
}


impl BlockState for WaxedCopperBars {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#east == false && block_state.r#south == true && block_state.r#west == true { return 7943; }
        if block_state.r#east == true && block_state.r#west == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#north == true { return 7923; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#west == false { return 7926; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#south == true && block_state.r#west == false && block_state.r#waterlogged == false { return 7928; }
        if block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#west == true && block_state.r#north == true { return 7939; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#south == true && block_state.r#east == false && block_state.r#waterlogged == true { return 7941; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#north == true && block_state.r#south == false && block_state.r#waterlogged == true { return 7937; }
        if block_state.r#east == true && block_state.r#north == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == false { return 7924; }
        if block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#south == false && block_state.r#north == false { return 7930; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#south == false && block_state.r#west == true { return 7945; }
        if block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#north == true && block_state.r#east == true && block_state.r#south == true { return 7918; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#south == false { return 7946; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#north == false && block_state.r#south == true { return 7942; }
        if block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#north == true && block_state.r#west == true && block_state.r#south == true { return 7917; }
        if block_state.r#south == true && block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == true { return 7933; }
        if block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#south == false && block_state.r#east == false && block_state.r#west == false { return 7948; }
        if block_state.r#north == true && block_state.r#west == false && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#east == false { return 7934; }
        if block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#east == false && block_state.r#south == false { return 7940; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#east == false { return 7944; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == true { return 7938; }
        if block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#north == true && block_state.r#south == true && block_state.r#west == false { return 7920; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#west == false && block_state.r#north == true && block_state.r#waterlogged == true { return 7922; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#south == false { return 7932; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#south == false && block_state.r#east == false && block_state.r#waterlogged == false { return 7947; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#east == true { return 7925; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == true { return 7927; }
        if block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#west == true && block_state.r#north == true { return 7919; }
        if block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#west == true && block_state.r#east == true { return 7929; }
        if block_state.r#east == true && block_state.r#north == true && block_state.r#south == false && block_state.r#west == true && block_state.r#waterlogged == true { return 7921; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == true { return 7936; }
        if block_state.r#west == true && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#south == true { return 7935; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#south == false { return 7931; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7943 {
            return Some(WaxedCopperBars {
                r#waterlogged: false,
                r#north: false,
                r#east: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 7923 {
            return Some(WaxedCopperBars {
                r#east: true,
                r#west: true,
                r#south: false,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 7926 {
            return Some(WaxedCopperBars {
                r#north: false,
                r#east: true,
                r#waterlogged: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 7928 {
            return Some(WaxedCopperBars {
                r#east: true,
                r#north: false,
                r#south: true,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7939 {
            return Some(WaxedCopperBars {
                r#south: false,
                r#waterlogged: false,
                r#east: false,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 7941 {
            return Some(WaxedCopperBars {
                r#west: true,
                r#north: false,
                r#south: true,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7937 {
            return Some(WaxedCopperBars {
                r#east: false,
                r#west: true,
                r#north: true,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7924 {
            return Some(WaxedCopperBars {
                r#east: true,
                r#north: true,
                r#south: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 7930 {
            return Some(WaxedCopperBars {
                r#west: false,
                r#waterlogged: true,
                r#east: true,
                r#south: false,
                r#north: false,
            });
        }
        if state_id == 7945 {
            return Some(WaxedCopperBars {
                r#east: false,
                r#waterlogged: true,
                r#north: false,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 7918 {
            return Some(WaxedCopperBars {
                r#waterlogged: true,
                r#west: false,
                r#north: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 7946 {
            return Some(WaxedCopperBars {
                r#east: false,
                r#north: false,
                r#west: false,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 7942 {
            return Some(WaxedCopperBars {
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 7917 {
            return Some(WaxedCopperBars {
                r#waterlogged: true,
                r#east: true,
                r#north: true,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 7933 {
            return Some(WaxedCopperBars {
                r#south: true,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 7948 {
            return Some(WaxedCopperBars {
                r#waterlogged: false,
                r#north: false,
                r#south: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 7934 {
            return Some(WaxedCopperBars {
                r#north: true,
                r#west: false,
                r#south: true,
                r#waterlogged: true,
                r#east: false,
            });
        }
        if state_id == 7940 {
            return Some(WaxedCopperBars {
                r#north: true,
                r#waterlogged: false,
                r#west: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 7944 {
            return Some(WaxedCopperBars {
                r#west: false,
                r#south: true,
                r#north: false,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 7938 {
            return Some(WaxedCopperBars {
                r#north: true,
                r#east: false,
                r#south: false,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7920 {
            return Some(WaxedCopperBars {
                r#waterlogged: false,
                r#east: true,
                r#north: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 7922 {
            return Some(WaxedCopperBars {
                r#east: true,
                r#south: false,
                r#west: false,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7932 {
            return Some(WaxedCopperBars {
                r#east: true,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 7947 {
            return Some(WaxedCopperBars {
                r#north: false,
                r#west: true,
                r#south: false,
                r#east: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7925 {
            return Some(WaxedCopperBars {
                r#north: false,
                r#south: true,
                r#west: true,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 7927 {
            return Some(WaxedCopperBars {
                r#north: false,
                r#south: true,
                r#east: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 7919 {
            return Some(WaxedCopperBars {
                r#south: true,
                r#waterlogged: false,
                r#east: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 7929 {
            return Some(WaxedCopperBars {
                r#north: false,
                r#waterlogged: true,
                r#south: false,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 7921 {
            return Some(WaxedCopperBars {
                r#east: true,
                r#north: true,
                r#south: false,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7936 {
            return Some(WaxedCopperBars {
                r#east: false,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 7935 {
            return Some(WaxedCopperBars {
                r#west: true,
                r#east: false,
                r#waterlogged: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 7931 {
            return Some(WaxedCopperBars {
                r#west: true,
                r#north: false,
                r#east: true,
                r#waterlogged: false,
                r#south: false,
            });
        }
        return None;
    }
}

