use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scaffolding {
    pub distance: i32,
    pub waterlogged: bool,
    pub bottom: bool,
}


impl BlockState for Scaffolding {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#distance == 4 && self.r#bottom == true { return 20512; }
        if self.r#distance == 4 && self.r#bottom == false && self.r#waterlogged == true { return 20528; }
        if self.r#waterlogged == true && self.r#distance == 2 && self.r#bottom == true { return 20508; }
        if self.r#bottom == true && self.r#distance == 7 && self.r#waterlogged == true { return 20518; }
        if self.r#bottom == false && self.r#distance == 0 && self.r#waterlogged == false { return 20521; }
        if self.r#bottom == true && self.r#distance == 0 && self.r#waterlogged == true { return 20504; }
        if self.r#bottom == false && self.r#distance == 5 && self.r#waterlogged == false { return 20531; }
        if self.r#waterlogged == false && self.r#bottom == false && self.r#distance == 2 { return 20525; }
        if self.r#waterlogged == false && self.r#bottom == true && self.r#distance == 3 { return 20511; }
        if self.r#bottom == true && self.r#waterlogged == true && self.r#distance == 5 { return 20514; }
        if self.r#distance == 1 && self.r#bottom == false && self.r#waterlogged == true { return 20522; }
        if self.r#waterlogged == false && self.r#bottom == false && self.r#distance == 3 { return 20527; }
        if self.r#bottom == false && self.r#distance == 6 && self.r#waterlogged == true { return 20532; }
        if self.r#waterlogged == true && self.r#bottom == false && self.r#distance == 7 { return 20534; }
        if self.r#distance == 3 && self.r#waterlogged == true && self.r#bottom == false { return 20526; }
        if self.r#distance == 2 && self.r#bottom == false && self.r#waterlogged == true { return 20524; }
        if self.r#waterlogged == false && self.r#distance == 6 && self.r#bottom == false { return 20533; }
        if self.r#distance == 3 && self.r#waterlogged == true && self.r#bottom == true { return 20510; }
        if self.r#waterlogged == true && self.r#distance == 1 && self.r#bottom == true { return 20506; }
        if self.r#waterlogged == false && self.r#bottom == true && self.r#distance == 4 { return 20513; }
        if self.r#distance == 4 && self.r#waterlogged == false && self.r#bottom == false { return 20529; }
        if self.r#bottom == true && self.r#waterlogged == false && self.r#distance == 1 { return 20507; }
        if self.r#bottom == true && self.r#distance == 6 && self.r#waterlogged == true { return 20516; }
        if self.r#waterlogged == true && self.r#distance == 0 && self.r#bottom == false { return 20520; }
        if self.r#bottom == false && self.r#distance == 5 && self.r#waterlogged == true { return 20530; }
        if self.r#distance == 0 && self.r#bottom == true && self.r#waterlogged == false { return 20505; }
        if self.r#bottom == true && self.r#distance == 7 && self.r#waterlogged == false { return 20519; }
        if self.r#distance == 1 && self.r#bottom == false && self.r#waterlogged == false { return 20523; }
        if self.r#distance == 5 && self.r#waterlogged == false && self.r#bottom == true { return 20515; }
        if self.r#distance == 6 && self.r#waterlogged == false && self.r#bottom == true { return 20517; }
        if self.r#waterlogged == false && self.r#bottom == true && self.r#distance == 2 { return 20509; }
        if self.r#waterlogged == false && self.r#distance == 7 && self.r#bottom == false { return 20535; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20512 {
            return Some(Scaffolding {
                r#waterlogged: true,
                r#distance: 4,
                r#bottom: true,
            });
        }
        if state_id == 20528 {
            return Some(Scaffolding {
                r#distance: 4,
                r#bottom: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20508 {
            return Some(Scaffolding {
                r#waterlogged: true,
                r#distance: 2,
                r#bottom: true,
            });
        }
        if state_id == 20518 {
            return Some(Scaffolding {
                r#bottom: true,
                r#distance: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 20521 {
            return Some(Scaffolding {
                r#bottom: false,
                r#distance: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 20504 {
            return Some(Scaffolding {
                r#bottom: true,
                r#distance: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 20531 {
            return Some(Scaffolding {
                r#bottom: false,
                r#distance: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 20525 {
            return Some(Scaffolding {
                r#waterlogged: false,
                r#bottom: false,
                r#distance: 2,
            });
        }
        if state_id == 20511 {
            return Some(Scaffolding {
                r#waterlogged: false,
                r#bottom: true,
                r#distance: 3,
            });
        }
        if state_id == 20514 {
            return Some(Scaffolding {
                r#bottom: true,
                r#waterlogged: true,
                r#distance: 5,
            });
        }
        if state_id == 20522 {
            return Some(Scaffolding {
                r#distance: 1,
                r#bottom: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20527 {
            return Some(Scaffolding {
                r#waterlogged: false,
                r#bottom: false,
                r#distance: 3,
            });
        }
        if state_id == 20532 {
            return Some(Scaffolding {
                r#bottom: false,
                r#distance: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 20534 {
            return Some(Scaffolding {
                r#waterlogged: true,
                r#bottom: false,
                r#distance: 7,
            });
        }
        if state_id == 20526 {
            return Some(Scaffolding {
                r#distance: 3,
                r#waterlogged: true,
                r#bottom: false,
            });
        }
        if state_id == 20524 {
            return Some(Scaffolding {
                r#distance: 2,
                r#bottom: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20533 {
            return Some(Scaffolding {
                r#waterlogged: false,
                r#distance: 6,
                r#bottom: false,
            });
        }
        if state_id == 20510 {
            return Some(Scaffolding {
                r#distance: 3,
                r#waterlogged: true,
                r#bottom: true,
            });
        }
        if state_id == 20506 {
            return Some(Scaffolding {
                r#waterlogged: true,
                r#distance: 1,
                r#bottom: true,
            });
        }
        if state_id == 20513 {
            return Some(Scaffolding {
                r#waterlogged: false,
                r#bottom: true,
                r#distance: 4,
            });
        }
        if state_id == 20529 {
            return Some(Scaffolding {
                r#distance: 4,
                r#waterlogged: false,
                r#bottom: false,
            });
        }
        if state_id == 20507 {
            return Some(Scaffolding {
                r#bottom: true,
                r#waterlogged: false,
                r#distance: 1,
            });
        }
        if state_id == 20516 {
            return Some(Scaffolding {
                r#bottom: true,
                r#distance: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 20520 {
            return Some(Scaffolding {
                r#waterlogged: true,
                r#distance: 0,
                r#bottom: false,
            });
        }
        if state_id == 20530 {
            return Some(Scaffolding {
                r#bottom: false,
                r#distance: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 20505 {
            return Some(Scaffolding {
                r#distance: 0,
                r#bottom: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20519 {
            return Some(Scaffolding {
                r#bottom: true,
                r#distance: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 20523 {
            return Some(Scaffolding {
                r#distance: 1,
                r#bottom: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20515 {
            return Some(Scaffolding {
                r#distance: 5,
                r#waterlogged: false,
                r#bottom: true,
            });
        }
        if state_id == 20517 {
            return Some(Scaffolding {
                r#distance: 6,
                r#waterlogged: false,
                r#bottom: true,
            });
        }
        if state_id == 20509 {
            return Some(Scaffolding {
                r#waterlogged: false,
                r#bottom: true,
                r#distance: 2,
            });
        }
        if state_id == 20535 {
            return Some(Scaffolding {
                r#waterlogged: false,
                r#distance: 7,
                r#bottom: false,
            });
        }
        return None;
    }
}

