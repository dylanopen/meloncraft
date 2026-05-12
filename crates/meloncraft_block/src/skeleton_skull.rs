use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkeletonSkull {
    pub rotation: i32,
    pub powered: bool,
}


impl BlockState for SkeletonSkull {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#rotation == 15 { return 10728; }
        if block_state.r#rotation == 9 && block_state.r#powered == false { return 10738; }
        if block_state.r#powered == true && block_state.r#rotation == 2 { return 10715; }
        if block_state.r#powered == false && block_state.r#rotation == 12 { return 10741; }
        if block_state.r#powered == false && block_state.r#rotation == 4 { return 10733; }
        if block_state.r#powered == false && block_state.r#rotation == 15 { return 10744; }
        if block_state.r#rotation == 13 && block_state.r#powered == false { return 10742; }
        if block_state.r#powered == true && block_state.r#rotation == 4 { return 10717; }
        if block_state.r#rotation == 6 && block_state.r#powered == true { return 10719; }
        if block_state.r#rotation == 13 && block_state.r#powered == true { return 10726; }
        if block_state.r#powered == false && block_state.r#rotation == 10 { return 10739; }
        if block_state.r#powered == false && block_state.r#rotation == 7 { return 10736; }
        if block_state.r#powered == true && block_state.r#rotation == 14 { return 10727; }
        if block_state.r#powered == false && block_state.r#rotation == 6 { return 10735; }
        if block_state.r#rotation == 8 && block_state.r#powered == false { return 10737; }
        if block_state.r#rotation == 9 && block_state.r#powered == true { return 10722; }
        if block_state.r#powered == false && block_state.r#rotation == 5 { return 10734; }
        if block_state.r#rotation == 11 && block_state.r#powered == false { return 10740; }
        if block_state.r#rotation == 0 && block_state.r#powered == true { return 10713; }
        if block_state.r#powered == true && block_state.r#rotation == 11 { return 10724; }
        if block_state.r#powered == false && block_state.r#rotation == 3 { return 10732; }
        if block_state.r#rotation == 5 && block_state.r#powered == true { return 10718; }
        if block_state.r#powered == true && block_state.r#rotation == 3 { return 10716; }
        if block_state.r#powered == false && block_state.r#rotation == 14 { return 10743; }
        if block_state.r#powered == false && block_state.r#rotation == 0 { return 10729; }
        if block_state.r#powered == false && block_state.r#rotation == 2 { return 10731; }
        if block_state.r#rotation == 12 && block_state.r#powered == true { return 10725; }
        if block_state.r#powered == true && block_state.r#rotation == 10 { return 10723; }
        if block_state.r#powered == true && block_state.r#rotation == 1 { return 10714; }
        if block_state.r#powered == true && block_state.r#rotation == 8 { return 10721; }
        if block_state.r#rotation == 1 && block_state.r#powered == false { return 10730; }
        if block_state.r#powered == true && block_state.r#rotation == 7 { return 10720; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10728 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 15,
            });
        }
        if state_id == 10738 {
            return Some(SkeletonSkull {
                r#rotation: 9,
                r#powered: false,
            });
        }
        if state_id == 10715 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 2,
            });
        }
        if state_id == 10741 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 12,
            });
        }
        if state_id == 10733 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 4,
            });
        }
        if state_id == 10744 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 15,
            });
        }
        if state_id == 10742 {
            return Some(SkeletonSkull {
                r#rotation: 13,
                r#powered: false,
            });
        }
        if state_id == 10717 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 4,
            });
        }
        if state_id == 10719 {
            return Some(SkeletonSkull {
                r#rotation: 6,
                r#powered: true,
            });
        }
        if state_id == 10726 {
            return Some(SkeletonSkull {
                r#rotation: 13,
                r#powered: true,
            });
        }
        if state_id == 10739 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 10,
            });
        }
        if state_id == 10736 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 7,
            });
        }
        if state_id == 10727 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 14,
            });
        }
        if state_id == 10735 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 6,
            });
        }
        if state_id == 10737 {
            return Some(SkeletonSkull {
                r#rotation: 8,
                r#powered: false,
            });
        }
        if state_id == 10722 {
            return Some(SkeletonSkull {
                r#rotation: 9,
                r#powered: true,
            });
        }
        if state_id == 10734 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 5,
            });
        }
        if state_id == 10740 {
            return Some(SkeletonSkull {
                r#rotation: 11,
                r#powered: false,
            });
        }
        if state_id == 10713 {
            return Some(SkeletonSkull {
                r#rotation: 0,
                r#powered: true,
            });
        }
        if state_id == 10724 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 11,
            });
        }
        if state_id == 10732 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 3,
            });
        }
        if state_id == 10718 {
            return Some(SkeletonSkull {
                r#rotation: 5,
                r#powered: true,
            });
        }
        if state_id == 10716 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 3,
            });
        }
        if state_id == 10743 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 14,
            });
        }
        if state_id == 10729 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 0,
            });
        }
        if state_id == 10731 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 2,
            });
        }
        if state_id == 10725 {
            return Some(SkeletonSkull {
                r#rotation: 12,
                r#powered: true,
            });
        }
        if state_id == 10723 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 10,
            });
        }
        if state_id == 10714 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 1,
            });
        }
        if state_id == 10721 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 8,
            });
        }
        if state_id == 10730 {
            return Some(SkeletonSkull {
                r#rotation: 1,
                r#powered: false,
            });
        }
        if state_id == 10720 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 7,
            });
        }
        return None;
    }
}

