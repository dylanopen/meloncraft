use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCopperBars {
    pub west: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
}


impl BlockState for OxidizedCopperBars {
    fn to_id(self) -> i32 {
        if block_state.r#north == true && block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#south == false { return 7907; }
        if block_state.r#west == false && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#south == true { return 7910; }
        if block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#west == false && block_state.r#east == false && block_state.r#north == false { return 7912; }
        if block_state.r#west == true && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#south == false { return 7889; }
        if block_state.r#north == true && block_state.r#south == true && block_state.r#west == false && block_state.r#east == true && block_state.r#waterlogged == false { return 7888; }
        if block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#west == true && block_state.r#east == false { return 7901; }
        if block_state.r#east == true && block_state.r#north == true && block_state.r#south == true && block_state.r#west == true && block_state.r#waterlogged == false { return 7887; }
        if block_state.r#south == true && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#north == false { return 7909; }
        if block_state.r#west == true && block_state.r#north == true && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#south == false { return 7905; }
        if block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#south == true && block_state.r#east == false && block_state.r#north == false { return 7911; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == false { return 7894; }
        if block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#east == false && block_state.r#south == false && block_state.r#west == false { return 7914; }
        if block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#east == true && block_state.r#west == false { return 7900; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#east == false && block_state.r#west == true && block_state.r#waterlogged == false { return 7903; }
        if block_state.r#east == true && block_state.r#north == true && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#south == true { return 7886; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#east == true && block_state.r#south == true && block_state.r#waterlogged == true { return 7893; }
        if block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#south == true && block_state.r#east == false && block_state.r#west == false { return 7904; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#north == true && block_state.r#west == true { return 7891; }
        if block_state.r#east == true && block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#north == false { return 7895; }
        if block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#west == false && block_state.r#east == false && block_state.r#south == true { return 7902; }
        if block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#south == false && block_state.r#east == false && block_state.r#west == false { return 7906; }
        if block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#west == true && block_state.r#south == false { return 7915; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#west == false && block_state.r#north == false && block_state.r#waterlogged == false { return 7896; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#south == false && block_state.r#west == true && block_state.r#waterlogged == true { return 7897; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == false { return 7892; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#west == false { return 7898; }
        if block_state.r#south == false && block_state.r#north == true && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == false { return 7890; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#south == false && block_state.r#west == true && block_state.r#waterlogged == false { return 7899; }
        if block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#north == false && block_state.r#south == false && block_state.r#east == false { return 7913; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == false && block_state.r#east == false && block_state.r#south == false { return 7916; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#south == false && block_state.r#north == true && block_state.r#east == false { return 7908; }
        if block_state.r#west == true && block_state.r#south == true && block_state.r#east == true && block_state.r#north == true && block_state.r#waterlogged == true { return 7885; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7907 {
            return Some(OxidizedCopperBars {
                r#north: true,
                r#west: true,
                r#waterlogged: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 7910 {
            return Some(OxidizedCopperBars {
                r#west: false,
                r#north: false,
                r#waterlogged: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 7912 {
            return Some(OxidizedCopperBars {
                r#waterlogged: false,
                r#south: true,
                r#west: false,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 7889 {
            return Some(OxidizedCopperBars {
                r#west: true,
                r#north: true,
                r#waterlogged: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 7888 {
            return Some(OxidizedCopperBars {
                r#north: true,
                r#south: true,
                r#west: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7901 {
            return Some(OxidizedCopperBars {
                r#north: true,
                r#waterlogged: true,
                r#south: true,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 7887 {
            return Some(OxidizedCopperBars {
                r#east: true,
                r#north: true,
                r#south: true,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7909 {
            return Some(OxidizedCopperBars {
                r#south: true,
                r#east: false,
                r#waterlogged: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 7905 {
            return Some(OxidizedCopperBars {
                r#west: true,
                r#north: true,
                r#east: false,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 7911 {
            return Some(OxidizedCopperBars {
                r#waterlogged: false,
                r#west: true,
                r#south: true,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 7894 {
            return Some(OxidizedCopperBars {
                r#south: true,
                r#east: true,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 7914 {
            return Some(OxidizedCopperBars {
                r#waterlogged: true,
                r#north: false,
                r#east: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 7900 {
            return Some(OxidizedCopperBars {
                r#south: false,
                r#waterlogged: false,
                r#north: false,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 7903 {
            return Some(OxidizedCopperBars {
                r#south: true,
                r#north: true,
                r#east: false,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7886 {
            return Some(OxidizedCopperBars {
                r#east: true,
                r#north: true,
                r#west: false,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 7893 {
            return Some(OxidizedCopperBars {
                r#west: true,
                r#north: false,
                r#east: true,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7904 {
            return Some(OxidizedCopperBars {
                r#waterlogged: false,
                r#north: true,
                r#south: true,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 7891 {
            return Some(OxidizedCopperBars {
                r#east: true,
                r#waterlogged: false,
                r#south: false,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 7895 {
            return Some(OxidizedCopperBars {
                r#east: true,
                r#west: true,
                r#waterlogged: false,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 7902 {
            return Some(OxidizedCopperBars {
                r#waterlogged: true,
                r#north: true,
                r#west: false,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 7906 {
            return Some(OxidizedCopperBars {
                r#waterlogged: true,
                r#north: true,
                r#south: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 7915 {
            return Some(OxidizedCopperBars {
                r#east: false,
                r#waterlogged: false,
                r#north: false,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 7896 {
            return Some(OxidizedCopperBars {
                r#south: true,
                r#east: true,
                r#west: false,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7897 {
            return Some(OxidizedCopperBars {
                r#north: false,
                r#east: true,
                r#south: false,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7892 {
            return Some(OxidizedCopperBars {
                r#north: true,
                r#south: false,
                r#east: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 7898 {
            return Some(OxidizedCopperBars {
                r#north: false,
                r#east: true,
                r#waterlogged: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 7890 {
            return Some(OxidizedCopperBars {
                r#south: false,
                r#north: true,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 7899 {
            return Some(OxidizedCopperBars {
                r#east: true,
                r#north: false,
                r#south: false,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7913 {
            return Some(OxidizedCopperBars {
                r#waterlogged: true,
                r#west: true,
                r#north: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 7916 {
            return Some(OxidizedCopperBars {
                r#waterlogged: false,
                r#west: false,
                r#north: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 7908 {
            return Some(OxidizedCopperBars {
                r#waterlogged: false,
                r#west: false,
                r#south: false,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 7885 {
            return Some(OxidizedCopperBars {
                r#west: true,
                r#south: true,
                r#east: true,
                r#north: true,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

