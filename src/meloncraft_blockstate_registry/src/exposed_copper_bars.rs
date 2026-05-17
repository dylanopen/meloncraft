use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCopperBars {
    pub east: bool,
    pub north: bool,
    pub waterlogged: bool,
    pub west: bool,
    pub south: bool,
}


impl BlockState for ExposedCopperBars {
    fn to_id(&self) -> i32 {
        if self.r#north == false && self.r#west == true && self.r#south == true && self.r#waterlogged == false && self.r#east == false { return 7847; }
        if self.r#north == true && self.r#waterlogged == true && self.r#west == false && self.r#south == false && self.r#east == false { return 7842; }
        if self.r#west == true && self.r#north == true && self.r#south == false && self.r#east == true && self.r#waterlogged == false { return 7827; }
        if self.r#east == true && self.r#west == false && self.r#waterlogged == false && self.r#south == true && self.r#north == false { return 7832; }
        if self.r#north == false && self.r#waterlogged == false && self.r#south == false && self.r#east == false && self.r#west == false { return 7852; }
        if self.r#waterlogged == true && self.r#west == true && self.r#south == false && self.r#north == true && self.r#east == false { return 7841; }
        if self.r#east == false && self.r#waterlogged == false && self.r#south == false && self.r#west == true && self.r#north == false { return 7851; }
        if self.r#west == true && self.r#east == true && self.r#south == false && self.r#waterlogged == true && self.r#north == false { return 7833; }
        if self.r#north == true && self.r#south == true && self.r#waterlogged == false && self.r#east == false && self.r#west == true { return 7839; }
        if self.r#east == false && self.r#south == true && self.r#waterlogged == true && self.r#west == true && self.r#north == false { return 7845; }
        if self.r#waterlogged == true && self.r#south == false && self.r#north == false && self.r#east == false && self.r#west == false { return 7850; }
        if self.r#south == true && self.r#east == true && self.r#north == false && self.r#waterlogged == false && self.r#west == true { return 7831; }
        if self.r#west == true && self.r#south == false && self.r#east == false && self.r#waterlogged == false && self.r#north == true { return 7843; }
        if self.r#north == false && self.r#east == false && self.r#waterlogged == true && self.r#west == true && self.r#south == false { return 7849; }
        if self.r#north == true && self.r#west == false && self.r#south == false && self.r#waterlogged == false && self.r#east == false { return 7844; }
        if self.r#west == true && self.r#south == false && self.r#north == false && self.r#east == true && self.r#waterlogged == false { return 7835; }
        if self.r#waterlogged == true && self.r#north == true && self.r#west == false && self.r#south == true && self.r#east == true { return 7822; }
        if self.r#south == true && self.r#east == true && self.r#north == false && self.r#west == true && self.r#waterlogged == true { return 7829; }
        if self.r#east == false && self.r#south == true && self.r#north == true && self.r#west == false && self.r#waterlogged == true { return 7838; }
        if self.r#west == false && self.r#east == true && self.r#north == true && self.r#waterlogged == false && self.r#south == true { return 7824; }
        if self.r#south == true && self.r#waterlogged == true && self.r#north == true && self.r#west == true && self.r#east == true { return 7821; }
        if self.r#north == false && self.r#south == false && self.r#east == true && self.r#waterlogged == true && self.r#west == false { return 7834; }
        if self.r#east == false && self.r#north == false && self.r#south == true && self.r#west == false && self.r#waterlogged == true { return 7846; }
        if self.r#north == true && self.r#west == false && self.r#waterlogged == false && self.r#south == false && self.r#east == true { return 7828; }
        if self.r#south == true && self.r#north == true && self.r#east == false && self.r#waterlogged == false && self.r#west == false { return 7840; }
        if self.r#west == true && self.r#east == false && self.r#north == true && self.r#waterlogged == true && self.r#south == true { return 7837; }
        if self.r#west == false && self.r#south == true && self.r#north == false && self.r#east == true && self.r#waterlogged == true { return 7830; }
        if self.r#east == true && self.r#waterlogged == false && self.r#south == true && self.r#west == true && self.r#north == true { return 7823; }
        if self.r#east == true && self.r#south == false && self.r#waterlogged == true && self.r#north == true && self.r#west == false { return 7826; }
        if self.r#east == false && self.r#north == false && self.r#waterlogged == false && self.r#west == false && self.r#south == true { return 7848; }
        if self.r#waterlogged == false && self.r#west == false && self.r#east == true && self.r#north == false && self.r#south == false { return 7836; }
        if self.r#east == true && self.r#west == true && self.r#south == false && self.r#waterlogged == true && self.r#north == true { return 7825; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7847 {
            return Some(ExposedCopperBars {
                r#north: false,
                r#west: true,
                r#south: true,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 7842 {
            return Some(ExposedCopperBars {
                r#north: true,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 7827 {
            return Some(ExposedCopperBars {
                r#west: true,
                r#north: true,
                r#south: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7832 {
            return Some(ExposedCopperBars {
                r#east: true,
                r#west: false,
                r#waterlogged: false,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 7852 {
            return Some(ExposedCopperBars {
                r#north: false,
                r#waterlogged: false,
                r#south: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 7841 {
            return Some(ExposedCopperBars {
                r#waterlogged: true,
                r#west: true,
                r#south: false,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 7851 {
            return Some(ExposedCopperBars {
                r#east: false,
                r#waterlogged: false,
                r#south: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 7833 {
            return Some(ExposedCopperBars {
                r#west: true,
                r#east: true,
                r#south: false,
                r#waterlogged: true,
                r#north: false,
            });
        }
        if state_id == 7839 {
            return Some(ExposedCopperBars {
                r#north: true,
                r#south: true,
                r#waterlogged: false,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 7845 {
            return Some(ExposedCopperBars {
                r#east: false,
                r#south: true,
                r#waterlogged: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 7850 {
            return Some(ExposedCopperBars {
                r#waterlogged: true,
                r#south: false,
                r#north: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 7831 {
            return Some(ExposedCopperBars {
                r#south: true,
                r#east: true,
                r#north: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 7843 {
            return Some(ExposedCopperBars {
                r#west: true,
                r#south: false,
                r#east: false,
                r#waterlogged: false,
                r#north: true,
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
                r#north: true,
                r#west: false,
                r#south: false,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 7835 {
            return Some(ExposedCopperBars {
                r#west: true,
                r#south: false,
                r#north: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7822 {
            return Some(ExposedCopperBars {
                r#waterlogged: true,
                r#north: true,
                r#west: false,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 7829 {
            return Some(ExposedCopperBars {
                r#south: true,
                r#east: true,
                r#north: false,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7838 {
            return Some(ExposedCopperBars {
                r#east: false,
                r#south: true,
                r#north: true,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7824 {
            return Some(ExposedCopperBars {
                r#west: false,
                r#east: true,
                r#north: true,
                r#waterlogged: false,
                r#south: true,
            });
        }
        if state_id == 7821 {
            return Some(ExposedCopperBars {
                r#south: true,
                r#waterlogged: true,
                r#north: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 7834 {
            return Some(ExposedCopperBars {
                r#north: false,
                r#south: false,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 7846 {
            return Some(ExposedCopperBars {
                r#east: false,
                r#north: false,
                r#south: true,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7828 {
            return Some(ExposedCopperBars {
                r#north: true,
                r#west: false,
                r#waterlogged: false,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 7840 {
            return Some(ExposedCopperBars {
                r#south: true,
                r#north: true,
                r#east: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 7837 {
            return Some(ExposedCopperBars {
                r#west: true,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 7830 {
            return Some(ExposedCopperBars {
                r#west: false,
                r#south: true,
                r#north: false,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7823 {
            return Some(ExposedCopperBars {
                r#east: true,
                r#waterlogged: false,
                r#south: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 7826 {
            return Some(ExposedCopperBars {
                r#east: true,
                r#south: false,
                r#waterlogged: true,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 7848 {
            return Some(ExposedCopperBars {
                r#east: false,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 7836 {
            return Some(ExposedCopperBars {
                r#waterlogged: false,
                r#west: false,
                r#east: true,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 7825 {
            return Some(ExposedCopperBars {
                r#east: true,
                r#west: true,
                r#south: false,
                r#waterlogged: true,
                r#north: true,
            });
        }
        return None;
    }
}

