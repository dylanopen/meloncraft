use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCopperBars {
    pub west: bool,
    pub waterlogged: bool,
    pub east: bool,
    pub south: bool,
    pub north: bool,
}


impl BlockState for WeatheredCopperBars {
    fn to_id(self) -> i32 {
        if block_state.r#south == true && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#west == true { return 7855; }
        if block_state.r#west == true && block_state.r#south == true && block_state.r#north == true && block_state.r#east == true && block_state.r#waterlogged == true { return 7853; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#east == false { return 7882; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == false { return 7884; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#north == true { return 7858; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#south == false && block_state.r#west == false { return 7868; }
        if block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#west == false && block_state.r#south == true { return 7878; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#north == true { return 7875; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#south == true { return 7861; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#east == false && block_state.r#west == true && block_state.r#waterlogged == false { return 7871; }
        if block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#north == false && block_state.r#south == true { return 7877; }
        if block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#south == true && block_state.r#west == false { return 7862; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#south == true { return 7863; }
        if block_state.r#north == true && block_state.r#south == true && block_state.r#east == true && block_state.r#west == false && block_state.r#waterlogged == true { return 7854; }
        if block_state.r#east == true && block_state.r#north == true && block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#south == false { return 7857; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == true { return 7867; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#west == true && block_state.r#south == true && block_state.r#waterlogged == true { return 7869; }
        if block_state.r#west == false && block_state.r#south == false && block_state.r#north == true && block_state.r#east == false && block_state.r#waterlogged == true { return 7874; }
        if block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#north == true && block_state.r#south == false && block_state.r#east == false { return 7873; }
        if block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#east == false && block_state.r#south == false { return 7881; }
        if block_state.r#west == true && block_state.r#south == false && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#east == true { return 7865; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#north == true { return 7876; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#south == true { return 7880; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#west == true { return 7859; }
        if block_state.r#south == false && block_state.r#north == false && block_state.r#east == true && block_state.r#west == false && block_state.r#waterlogged == true { return 7866; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == true && block_state.r#south == true && block_state.r#east == true { return 7856; }
        if block_state.r#south == true && block_state.r#north == false && block_state.r#west == false && block_state.r#east == true && block_state.r#waterlogged == false { return 7864; }
        if block_state.r#north == true && block_state.r#west == false && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#east == false { return 7870; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#east == false && block_state.r#north == true && block_state.r#south == true { return 7872; }
        if block_state.r#west == false && block_state.r#north == true && block_state.r#south == false && block_state.r#east == true && block_state.r#waterlogged == false { return 7860; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#east == false && block_state.r#west == true && block_state.r#waterlogged == false { return 7883; }
        if block_state.r#west == true && block_state.r#east == false && block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == false { return 7879; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7855 {
            return Some(WeatheredCopperBars {
                r#south: true,
                r#east: true,
                r#waterlogged: false,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 7853 {
            return Some(WeatheredCopperBars {
                r#west: true,
                r#south: true,
                r#north: true,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7882 {
            return Some(WeatheredCopperBars {
                r#north: false,
                r#south: false,
                r#waterlogged: true,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 7884 {
            return Some(WeatheredCopperBars {
                r#east: false,
                r#south: false,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 7858 {
            return Some(WeatheredCopperBars {
                r#east: true,
                r#south: false,
                r#west: false,
                r#waterlogged: true,
                r#north: true,
            });
        }
        if state_id == 7868 {
            return Some(WeatheredCopperBars {
                r#east: true,
                r#waterlogged: false,
                r#north: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 7878 {
            return Some(WeatheredCopperBars {
                r#north: false,
                r#waterlogged: true,
                r#east: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 7875 {
            return Some(WeatheredCopperBars {
                r#south: false,
                r#east: false,
                r#waterlogged: false,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 7861 {
            return Some(WeatheredCopperBars {
                r#west: true,
                r#north: false,
                r#east: true,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 7871 {
            return Some(WeatheredCopperBars {
                r#south: true,
                r#north: true,
                r#east: false,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7877 {
            return Some(WeatheredCopperBars {
                r#west: true,
                r#waterlogged: true,
                r#east: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 7862 {
            return Some(WeatheredCopperBars {
                r#north: false,
                r#waterlogged: true,
                r#east: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 7863 {
            return Some(WeatheredCopperBars {
                r#west: true,
                r#north: false,
                r#east: true,
                r#waterlogged: false,
                r#south: true,
            });
        }
        if state_id == 7854 {
            return Some(WeatheredCopperBars {
                r#north: true,
                r#south: true,
                r#east: true,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7857 {
            return Some(WeatheredCopperBars {
                r#east: true,
                r#north: true,
                r#west: true,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 7867 {
            return Some(WeatheredCopperBars {
                r#north: false,
                r#south: false,
                r#east: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 7869 {
            return Some(WeatheredCopperBars {
                r#east: false,
                r#north: true,
                r#west: true,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7874 {
            return Some(WeatheredCopperBars {
                r#west: false,
                r#south: false,
                r#north: true,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7873 {
            return Some(WeatheredCopperBars {
                r#waterlogged: true,
                r#west: true,
                r#north: true,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 7881 {
            return Some(WeatheredCopperBars {
                r#west: true,
                r#waterlogged: true,
                r#north: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 7865 {
            return Some(WeatheredCopperBars {
                r#west: true,
                r#south: false,
                r#north: false,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 7876 {
            return Some(WeatheredCopperBars {
                r#south: false,
                r#east: false,
                r#west: false,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 7880 {
            return Some(WeatheredCopperBars {
                r#east: false,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 7859 {
            return Some(WeatheredCopperBars {
                r#east: true,
                r#south: false,
                r#north: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 7866 {
            return Some(WeatheredCopperBars {
                r#south: false,
                r#north: false,
                r#east: true,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7856 {
            return Some(WeatheredCopperBars {
                r#waterlogged: false,
                r#west: false,
                r#north: true,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 7864 {
            return Some(WeatheredCopperBars {
                r#south: true,
                r#north: false,
                r#west: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7870 {
            return Some(WeatheredCopperBars {
                r#north: true,
                r#west: false,
                r#south: true,
                r#waterlogged: true,
                r#east: false,
            });
        }
        if state_id == 7872 {
            return Some(WeatheredCopperBars {
                r#waterlogged: false,
                r#west: false,
                r#east: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 7860 {
            return Some(WeatheredCopperBars {
                r#west: false,
                r#north: true,
                r#south: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7883 {
            return Some(WeatheredCopperBars {
                r#north: false,
                r#south: false,
                r#east: false,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7879 {
            return Some(WeatheredCopperBars {
                r#west: true,
                r#east: false,
                r#north: false,
                r#south: true,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

