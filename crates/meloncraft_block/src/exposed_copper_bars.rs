use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCopperBars {
    pub east: bool,
    pub waterlogged: bool,
    pub south: bool,
    pub west: bool,
    pub north: bool,
}


impl BlockState for ExposedCopperBars {
    fn to_id(self) -> i32 {
        if block_state.r#east == false && block_state.r#north == true && block_state.r#west == true && block_state.r#south == false && block_state.r#waterlogged == false { return 7843; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#west == false { return 7828; }
        if block_state.r#west == false && block_state.r#south == false && block_state.r#north == false && block_state.r#east == false && block_state.r#waterlogged == true { return 7850; }
        if block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#south == false && block_state.r#west == false && block_state.r#north == false { return 7834; }
        if block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#west == true && block_state.r#north == true && block_state.r#south == true { return 7823; }
        if block_state.r#west == false && block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#east == true { return 7836; }
        if block_state.r#south == true && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#east == true { return 7832; }
        if block_state.r#north == true && block_state.r#west == false && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#east == true { return 7822; }
        if block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#south == true && block_state.r#east == false { return 7847; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == false { return 7849; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#north == true && block_state.r#west == false && block_state.r#waterlogged == false { return 7844; }
        if block_state.r#east == false && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#south == true { return 7848; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#south == true && block_state.r#east == true && block_state.r#waterlogged == false { return 7831; }
        if block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#east == false && block_state.r#west == false { return 7842; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#west == false { return 7826; }
        if block_state.r#east == true && block_state.r#west == true && block_state.r#north == true && block_state.r#south == false && block_state.r#waterlogged == true { return 7825; }
        if block_state.r#east == true && block_state.r#west == false && block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == true { return 7830; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#east == false { return 7851; }
        if block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#east == true && block_state.r#south == true { return 7824; }
        if block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#north == false && block_state.r#west == false && block_state.r#east == false { return 7852; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == false { return 7840; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#east == false { return 7845; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#west == true { return 7827; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == true { return 7833; }
        if block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#east == false && block_state.r#north == true { return 7841; }
        if block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#west == true && block_state.r#east == true && block_state.r#north == false { return 7829; }
        if block_state.r#south == true && block_state.r#west == false && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#north == true { return 7838; }
        if block_state.r#north == true && block_state.r#south == true && block_state.r#west == true && block_state.r#east == true && block_state.r#waterlogged == true { return 7821; }
        if block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#east == false && block_state.r#west == true && block_state.r#north == true { return 7839; }
        if block_state.r#east == true && block_state.r#west == true && block_state.r#south == false && block_state.r#north == false && block_state.r#waterlogged == false { return 7835; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#west == false && block_state.r#south == true && block_state.r#waterlogged == true { return 7846; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#south == true && block_state.r#west == true { return 7837; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7843 {
            return Some(ExposedCopperBars {
                r#east: false,
                r#north: true,
                r#west: true,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7828 {
            return Some(ExposedCopperBars {
                r#east: true,
                r#south: false,
                r#waterlogged: false,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 7850 {
            return Some(ExposedCopperBars {
                r#west: false,
                r#south: false,
                r#north: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7834 {
            return Some(ExposedCopperBars {
                r#waterlogged: true,
                r#east: true,
                r#south: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 7823 {
            return Some(ExposedCopperBars {
                r#waterlogged: false,
                r#east: true,
                r#west: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 7836 {
            return Some(ExposedCopperBars {
                r#west: false,
                r#north: false,
                r#south: false,
                r#waterlogged: false,
                r#east: true,
            });
        }
        if state_id == 7832 {
            return Some(ExposedCopperBars {
                r#south: true,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 7822 {
            return Some(ExposedCopperBars {
                r#north: true,
                r#west: false,
                r#south: true,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 7847 {
            return Some(ExposedCopperBars {
                r#north: false,
                r#waterlogged: false,
                r#west: true,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 7849 {
            return Some(ExposedCopperBars {
                r#north: false,
                r#east: false,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 7844 {
            return Some(ExposedCopperBars {
                r#east: false,
                r#south: false,
                r#north: true,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7848 {
            return Some(ExposedCopperBars {
                r#east: false,
                r#west: false,
                r#waterlogged: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 7831 {
            return Some(ExposedCopperBars {
                r#west: true,
                r#north: false,
                r#south: true,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7842 {
            return Some(ExposedCopperBars {
                r#south: false,
                r#waterlogged: true,
                r#north: true,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 7826 {
            return Some(ExposedCopperBars {
                r#north: true,
                r#south: false,
                r#waterlogged: true,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 7825 {
            return Some(ExposedCopperBars {
                r#east: true,
                r#west: true,
                r#north: true,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7830 {
            return Some(ExposedCopperBars {
                r#east: true,
                r#west: false,
                r#north: false,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7851 {
            return Some(ExposedCopperBars {
                r#north: false,
                r#west: true,
                r#south: false,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 7824 {
            return Some(ExposedCopperBars {
                r#north: true,
                r#waterlogged: false,
                r#west: false,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 7852 {
            return Some(ExposedCopperBars {
                r#waterlogged: false,
                r#south: false,
                r#north: false,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 7840 {
            return Some(ExposedCopperBars {
                r#west: false,
                r#south: true,
                r#east: false,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7845 {
            return Some(ExposedCopperBars {
                r#north: false,
                r#south: true,
                r#waterlogged: true,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 7827 {
            return Some(ExposedCopperBars {
                r#east: true,
                r#south: false,
                r#waterlogged: false,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 7833 {
            return Some(ExposedCopperBars {
                r#north: false,
                r#south: false,
                r#east: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 7841 {
            return Some(ExposedCopperBars {
                r#west: true,
                r#waterlogged: true,
                r#south: false,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 7829 {
            return Some(ExposedCopperBars {
                r#waterlogged: true,
                r#south: true,
                r#west: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 7838 {
            return Some(ExposedCopperBars {
                r#south: true,
                r#west: false,
                r#east: false,
                r#waterlogged: true,
                r#north: true,
            });
        }
        if state_id == 7821 {
            return Some(ExposedCopperBars {
                r#north: true,
                r#south: true,
                r#west: true,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7839 {
            return Some(ExposedCopperBars {
                r#waterlogged: false,
                r#south: true,
                r#east: false,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 7835 {
            return Some(ExposedCopperBars {
                r#east: true,
                r#west: true,
                r#south: false,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7846 {
            return Some(ExposedCopperBars {
                r#north: false,
                r#east: false,
                r#west: false,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7837 {
            return Some(ExposedCopperBars {
                r#east: false,
                r#waterlogged: true,
                r#north: true,
                r#south: true,
                r#west: true,
            });
        }
        return None;
    }
}

