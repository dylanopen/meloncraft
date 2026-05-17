use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AzaleaLeaves {
    pub waterlogged: bool,
    pub distance: i32,
    pub persistent: bool,
}


impl BlockState for AzaleaLeaves {
    fn to_id(&self) -> i32 {
        if self.r#distance == 5 && self.r#persistent == false && self.r#waterlogged == false { return 523; }
        if self.r#persistent == false && self.r#distance == 4 && self.r#waterlogged == true { return 518; }
        if self.r#distance == 6 && self.r#persistent == false && self.r#waterlogged == true { return 526; }
        if self.r#persistent == false && self.r#waterlogged == false && self.r#distance == 6 { return 527; }
        if self.r#distance == 2 && self.r#waterlogged == false && self.r#persistent == false { return 511; }
        if self.r#distance == 3 && self.r#persistent == false && self.r#waterlogged == false { return 515; }
        if self.r#distance == 2 && self.r#waterlogged == false && self.r#persistent == true { return 509; }
        if self.r#distance == 2 && self.r#persistent == false && self.r#waterlogged == true { return 510; }
        if self.r#waterlogged == true && self.r#distance == 1 && self.r#persistent == true { return 504; }
        if self.r#distance == 7 && self.r#persistent == false && self.r#waterlogged == false { return 531; }
        if self.r#distance == 6 && self.r#persistent == true && self.r#waterlogged == false { return 525; }
        if self.r#persistent == true && self.r#waterlogged == true && self.r#distance == 4 { return 516; }
        if self.r#distance == 2 && self.r#waterlogged == true && self.r#persistent == true { return 508; }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 4 { return 517; }
        if self.r#distance == 1 && self.r#waterlogged == false && self.r#persistent == false { return 507; }
        if self.r#waterlogged == true && self.r#persistent == true && self.r#distance == 7 { return 528; }
        if self.r#distance == 4 && self.r#persistent == false && self.r#waterlogged == false { return 519; }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 3 { return 513; }
        if self.r#distance == 7 && self.r#persistent == true && self.r#waterlogged == false { return 529; }
        if self.r#persistent == true && self.r#distance == 3 && self.r#waterlogged == true { return 512; }
        if self.r#waterlogged == false && self.r#distance == 5 && self.r#persistent == true { return 521; }
        if self.r#waterlogged == true && self.r#persistent == false && self.r#distance == 1 { return 506; }
        if self.r#persistent == true && self.r#distance == 1 && self.r#waterlogged == false { return 505; }
        if self.r#distance == 5 && self.r#waterlogged == true && self.r#persistent == true { return 520; }
        if self.r#persistent == true && self.r#distance == 6 && self.r#waterlogged == true { return 524; }
        if self.r#distance == 7 && self.r#persistent == false && self.r#waterlogged == true { return 530; }
        if self.r#distance == 5 && self.r#persistent == false && self.r#waterlogged == true { return 522; }
        if self.r#distance == 3 && self.r#persistent == false && self.r#waterlogged == true { return 514; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 523 {
            return Some(AzaleaLeaves {
                r#distance: 5,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 518 {
            return Some(AzaleaLeaves {
                r#persistent: false,
                r#distance: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 526 {
            return Some(AzaleaLeaves {
                r#distance: 6,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 527 {
            return Some(AzaleaLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 6,
            });
        }
        if state_id == 511 {
            return Some(AzaleaLeaves {
                r#distance: 2,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 515 {
            return Some(AzaleaLeaves {
                r#distance: 3,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 509 {
            return Some(AzaleaLeaves {
                r#distance: 2,
                r#waterlogged: false,
                r#persistent: true,
            });
        }
        if state_id == 510 {
            return Some(AzaleaLeaves {
                r#distance: 2,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 504 {
            return Some(AzaleaLeaves {
                r#waterlogged: true,
                r#distance: 1,
                r#persistent: true,
            });
        }
        if state_id == 531 {
            return Some(AzaleaLeaves {
                r#distance: 7,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 525 {
            return Some(AzaleaLeaves {
                r#distance: 6,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 516 {
            return Some(AzaleaLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 4,
            });
        }
        if state_id == 508 {
            return Some(AzaleaLeaves {
                r#distance: 2,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 517 {
            return Some(AzaleaLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 4,
            });
        }
        if state_id == 507 {
            return Some(AzaleaLeaves {
                r#distance: 1,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 528 {
            return Some(AzaleaLeaves {
                r#waterlogged: true,
                r#persistent: true,
                r#distance: 7,
            });
        }
        if state_id == 519 {
            return Some(AzaleaLeaves {
                r#distance: 4,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 513 {
            return Some(AzaleaLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 3,
            });
        }
        if state_id == 529 {
            return Some(AzaleaLeaves {
                r#distance: 7,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 512 {
            return Some(AzaleaLeaves {
                r#persistent: true,
                r#distance: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 521 {
            return Some(AzaleaLeaves {
                r#waterlogged: false,
                r#distance: 5,
                r#persistent: true,
            });
        }
        if state_id == 506 {
            return Some(AzaleaLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 1,
            });
        }
        if state_id == 505 {
            return Some(AzaleaLeaves {
                r#persistent: true,
                r#distance: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 520 {
            return Some(AzaleaLeaves {
                r#distance: 5,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 524 {
            return Some(AzaleaLeaves {
                r#persistent: true,
                r#distance: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 530 {
            return Some(AzaleaLeaves {
                r#distance: 7,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 522 {
            return Some(AzaleaLeaves {
                r#distance: 5,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 514 {
            return Some(AzaleaLeaves {
                r#distance: 3,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

