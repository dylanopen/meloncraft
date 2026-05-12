use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AzaleaLeaves {
    pub waterlogged: bool,
    pub distance: i32,
    pub persistent: bool,
}


impl BlockState for AzaleaLeaves {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#distance == 4 && block_state.r#persistent == true { return 516; }
        if block_state.r#waterlogged == false && block_state.r#distance == 6 && block_state.r#persistent == true { return 525; }
        if block_state.r#distance == 5 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 521; }
        if block_state.r#waterlogged == true && block_state.r#persistent == true && block_state.r#distance == 1 { return 504; }
        if block_state.r#waterlogged == true && block_state.r#distance == 1 && block_state.r#persistent == false { return 506; }
        if block_state.r#distance == 2 && block_state.r#waterlogged == false && block_state.r#persistent == true { return 509; }
        if block_state.r#distance == 6 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 526; }
        if block_state.r#distance == 4 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 517; }
        if block_state.r#persistent == false && block_state.r#distance == 6 && block_state.r#waterlogged == false { return 527; }
        if block_state.r#persistent == false && block_state.r#distance == 7 && block_state.r#waterlogged == false { return 531; }
        if block_state.r#distance == 1 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 505; }
        if block_state.r#distance == 7 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 530; }
        if block_state.r#distance == 2 && block_state.r#persistent == true && block_state.r#waterlogged == true { return 508; }
        if block_state.r#persistent == true && block_state.r#distance == 3 && block_state.r#waterlogged == true { return 512; }
        if block_state.r#persistent == false && block_state.r#waterlogged == true && block_state.r#distance == 3 { return 514; }
        if block_state.r#persistent == true && block_state.r#distance == 3 && block_state.r#waterlogged == false { return 513; }
        if block_state.r#persistent == false && block_state.r#distance == 3 && block_state.r#waterlogged == false { return 515; }
        if block_state.r#distance == 4 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 518; }
        if block_state.r#distance == 5 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 522; }
        if block_state.r#waterlogged == false && block_state.r#distance == 2 && block_state.r#persistent == false { return 511; }
        if block_state.r#distance == 4 && block_state.r#waterlogged == false && block_state.r#persistent == false { return 519; }
        if block_state.r#waterlogged == true && block_state.r#distance == 6 && block_state.r#persistent == true { return 524; }
        if block_state.r#distance == 2 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 510; }
        if block_state.r#distance == 7 && block_state.r#persistent == true && block_state.r#waterlogged == true { return 528; }
        if block_state.r#distance == 1 && block_state.r#persistent == false && block_state.r#waterlogged == false { return 507; }
        if block_state.r#waterlogged == false && block_state.r#persistent == true && block_state.r#distance == 7 { return 529; }
        if block_state.r#persistent == true && block_state.r#waterlogged == true && block_state.r#distance == 5 { return 520; }
        if block_state.r#distance == 5 && block_state.r#waterlogged == false && block_state.r#persistent == false { return 523; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 516 {
            return Some(AzaleaLeaves {
                r#waterlogged: true,
                r#distance: 4,
                r#persistent: true,
            });
        }
        if state_id == 525 {
            return Some(AzaleaLeaves {
                r#waterlogged: false,
                r#distance: 6,
                r#persistent: true,
            });
        }
        if state_id == 521 {
            return Some(AzaleaLeaves {
                r#distance: 5,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 504 {
            return Some(AzaleaLeaves {
                r#waterlogged: true,
                r#persistent: true,
                r#distance: 1,
            });
        }
        if state_id == 506 {
            return Some(AzaleaLeaves {
                r#waterlogged: true,
                r#distance: 1,
                r#persistent: false,
            });
        }
        if state_id == 509 {
            return Some(AzaleaLeaves {
                r#distance: 2,
                r#waterlogged: false,
                r#persistent: true,
            });
        }
        if state_id == 526 {
            return Some(AzaleaLeaves {
                r#distance: 6,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 517 {
            return Some(AzaleaLeaves {
                r#distance: 4,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 527 {
            return Some(AzaleaLeaves {
                r#persistent: false,
                r#distance: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 531 {
            return Some(AzaleaLeaves {
                r#persistent: false,
                r#distance: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 505 {
            return Some(AzaleaLeaves {
                r#distance: 1,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 530 {
            return Some(AzaleaLeaves {
                r#distance: 7,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 508 {
            return Some(AzaleaLeaves {
                r#distance: 2,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 512 {
            return Some(AzaleaLeaves {
                r#persistent: true,
                r#distance: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 514 {
            return Some(AzaleaLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 3,
            });
        }
        if state_id == 513 {
            return Some(AzaleaLeaves {
                r#persistent: true,
                r#distance: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 515 {
            return Some(AzaleaLeaves {
                r#persistent: false,
                r#distance: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 518 {
            return Some(AzaleaLeaves {
                r#distance: 4,
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
        if state_id == 511 {
            return Some(AzaleaLeaves {
                r#waterlogged: false,
                r#distance: 2,
                r#persistent: false,
            });
        }
        if state_id == 519 {
            return Some(AzaleaLeaves {
                r#distance: 4,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 524 {
            return Some(AzaleaLeaves {
                r#waterlogged: true,
                r#distance: 6,
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
        if state_id == 528 {
            return Some(AzaleaLeaves {
                r#distance: 7,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 507 {
            return Some(AzaleaLeaves {
                r#distance: 1,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 529 {
            return Some(AzaleaLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 7,
            });
        }
        if state_id == 520 {
            return Some(AzaleaLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 5,
            });
        }
        if state_id == 523 {
            return Some(AzaleaLeaves {
                r#distance: 5,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        return None;
    }
}

