use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCopperBars {
    pub south: bool,
    pub north: bool,
    pub waterlogged: bool,
    pub west: bool,
    pub east: bool,
}


impl BlockState for WaxedOxidizedCopperBars {
    fn to_id(&self) -> i32 {
        if self.r#west == true && self.r#south == false && self.r#north == true && self.r#east == false && self.r#waterlogged == true { return 8033; }
        if self.r#east == false && self.r#south == false && self.r#north == false && self.r#west == true && self.r#waterlogged == false { return 8043; }
        if self.r#west == false && self.r#waterlogged == true && self.r#east == false && self.r#north == true && self.r#south == true { return 8030; }
        if self.r#east == false && self.r#south == false && self.r#north == false && self.r#west == false && self.r#waterlogged == false { return 8044; }
        if self.r#waterlogged == true && self.r#west == false && self.r#east == false && self.r#south == false && self.r#north == true { return 8034; }
        if self.r#east == true && self.r#north == false && self.r#waterlogged == false && self.r#west == false && self.r#south == true { return 8024; }
        if self.r#south == true && self.r#north == false && self.r#waterlogged == true && self.r#west == true && self.r#east == true { return 8021; }
        if self.r#south == false && self.r#west == true && self.r#waterlogged == false && self.r#north == true && self.r#east == false { return 8035; }
        if self.r#south == false && self.r#east == true && self.r#north == true && self.r#waterlogged == false && self.r#west == true { return 8019; }
        if self.r#north == false && self.r#waterlogged == false && self.r#east == false && self.r#south == true && self.r#west == false { return 8040; }
        if self.r#east == true && self.r#waterlogged == false && self.r#west == true && self.r#north == false && self.r#south == true { return 8023; }
        if self.r#west == false && self.r#south == true && self.r#waterlogged == true && self.r#east == true && self.r#north == false { return 8022; }
        if self.r#west == true && self.r#south == false && self.r#north == true && self.r#east == true && self.r#waterlogged == true { return 8017; }
        if self.r#west == true && self.r#south == true && self.r#north == false && self.r#waterlogged == true && self.r#east == false { return 8037; }
        if self.r#waterlogged == false && self.r#south == false && self.r#north == true && self.r#west == false && self.r#east == false { return 8036; }
        if self.r#waterlogged == false && self.r#west == true && self.r#south == false && self.r#north == false && self.r#east == true { return 8027; }
        if self.r#north == false && self.r#waterlogged == true && self.r#east == false && self.r#south == false && self.r#west == true { return 8041; }
        if self.r#east == false && self.r#waterlogged == false && self.r#south == true && self.r#north == true && self.r#west == true { return 8031; }
        if self.r#north == true && self.r#waterlogged == false && self.r#east == true && self.r#south == false && self.r#west == false { return 8020; }
        if self.r#north == false && self.r#waterlogged == true && self.r#east == false && self.r#west == false && self.r#south == true { return 8038; }
        if self.r#south == false && self.r#west == false && self.r#north == true && self.r#waterlogged == true && self.r#east == true { return 8018; }
        if self.r#south == false && self.r#east == true && self.r#north == false && self.r#waterlogged == true && self.r#west == false { return 8026; }
        if self.r#south == false && self.r#north == false && self.r#waterlogged == true && self.r#east == true && self.r#west == true { return 8025; }
        if self.r#south == true && self.r#west == true && self.r#north == true && self.r#east == true && self.r#waterlogged == false { return 8015; }
        if self.r#waterlogged == false && self.r#south == true && self.r#north == true && self.r#west == false && self.r#east == true { return 8016; }
        if self.r#east == true && self.r#waterlogged == true && self.r#north == true && self.r#south == true && self.r#west == true { return 8013; }
        if self.r#east == true && self.r#south == false && self.r#north == false && self.r#waterlogged == false && self.r#west == false { return 8028; }
        if self.r#waterlogged == false && self.r#east == false && self.r#south == true && self.r#west == true && self.r#north == false { return 8039; }
        if self.r#east == true && self.r#west == false && self.r#south == true && self.r#north == true && self.r#waterlogged == true { return 8014; }
        if self.r#east == false && self.r#south == true && self.r#waterlogged == false && self.r#west == false && self.r#north == true { return 8032; }
        if self.r#east == false && self.r#west == true && self.r#waterlogged == true && self.r#south == true && self.r#north == true { return 8029; }
        if self.r#east == false && self.r#south == false && self.r#waterlogged == true && self.r#north == false && self.r#west == false { return 8042; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8033 {
            return Some(WaxedOxidizedCopperBars {
                r#west: true,
                r#south: false,
                r#north: true,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8043 {
            return Some(WaxedOxidizedCopperBars {
                r#east: false,
                r#south: false,
                r#north: false,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8030 {
            return Some(WaxedOxidizedCopperBars {
                r#west: false,
                r#waterlogged: true,
                r#east: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 8044 {
            return Some(WaxedOxidizedCopperBars {
                r#east: false,
                r#south: false,
                r#north: false,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 8034 {
            return Some(WaxedOxidizedCopperBars {
                r#waterlogged: true,
                r#west: false,
                r#east: false,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 8024 {
            return Some(WaxedOxidizedCopperBars {
                r#east: true,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 8021 {
            return Some(WaxedOxidizedCopperBars {
                r#south: true,
                r#north: false,
                r#waterlogged: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 8035 {
            return Some(WaxedOxidizedCopperBars {
                r#south: false,
                r#west: true,
                r#waterlogged: false,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 8019 {
            return Some(WaxedOxidizedCopperBars {
                r#south: false,
                r#east: true,
                r#north: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 8040 {
            return Some(WaxedOxidizedCopperBars {
                r#north: false,
                r#waterlogged: false,
                r#east: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 8023 {
            return Some(WaxedOxidizedCopperBars {
                r#east: true,
                r#waterlogged: false,
                r#west: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 8022 {
            return Some(WaxedOxidizedCopperBars {
                r#west: false,
                r#south: true,
                r#waterlogged: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 8017 {
            return Some(WaxedOxidizedCopperBars {
                r#west: true,
                r#south: false,
                r#north: true,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 8037 {
            return Some(WaxedOxidizedCopperBars {
                r#west: true,
                r#south: true,
                r#north: false,
                r#waterlogged: true,
                r#east: false,
            });
        }
        if state_id == 8036 {
            return Some(WaxedOxidizedCopperBars {
                r#waterlogged: false,
                r#south: false,
                r#north: true,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 8027 {
            return Some(WaxedOxidizedCopperBars {
                r#waterlogged: false,
                r#west: true,
                r#south: false,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 8041 {
            return Some(WaxedOxidizedCopperBars {
                r#north: false,
                r#waterlogged: true,
                r#east: false,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 8031 {
            return Some(WaxedOxidizedCopperBars {
                r#east: false,
                r#waterlogged: false,
                r#south: true,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 8020 {
            return Some(WaxedOxidizedCopperBars {
                r#north: true,
                r#waterlogged: false,
                r#east: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 8038 {
            return Some(WaxedOxidizedCopperBars {
                r#north: false,
                r#waterlogged: true,
                r#east: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 8018 {
            return Some(WaxedOxidizedCopperBars {
                r#south: false,
                r#west: false,
                r#north: true,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 8026 {
            return Some(WaxedOxidizedCopperBars {
                r#south: false,
                r#east: true,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 8025 {
            return Some(WaxedOxidizedCopperBars {
                r#south: false,
                r#north: false,
                r#waterlogged: true,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 8015 {
            return Some(WaxedOxidizedCopperBars {
                r#south: true,
                r#west: true,
                r#north: true,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8016 {
            return Some(WaxedOxidizedCopperBars {
                r#waterlogged: false,
                r#south: true,
                r#north: true,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 8013 {
            return Some(WaxedOxidizedCopperBars {
                r#east: true,
                r#waterlogged: true,
                r#north: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 8028 {
            return Some(WaxedOxidizedCopperBars {
                r#east: true,
                r#south: false,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 8039 {
            return Some(WaxedOxidizedCopperBars {
                r#waterlogged: false,
                r#east: false,
                r#south: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 8014 {
            return Some(WaxedOxidizedCopperBars {
                r#east: true,
                r#west: false,
                r#south: true,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 8032 {
            return Some(WaxedOxidizedCopperBars {
                r#east: false,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 8029 {
            return Some(WaxedOxidizedCopperBars {
                r#east: false,
                r#west: true,
                r#waterlogged: true,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 8042 {
            return Some(WaxedOxidizedCopperBars {
                r#east: false,
                r#south: false,
                r#waterlogged: true,
                r#north: false,
                r#west: false,
            });
        }
        return None;
    }
}

