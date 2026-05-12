use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkeletonSkull {
    pub rotation: i32,
    pub powered: bool,
}


impl BlockState for SkeletonSkull {
    fn to_id(&self) -> i32 {
        if self.r#powered == true && self.r#rotation == 2 { return 10715; }
        if self.r#powered == false && self.r#rotation == 2 { return 10731; }
        if self.r#powered == false && self.r#rotation == 3 { return 10732; }
        if self.r#powered == true && self.r#rotation == 3 { return 10716; }
        if self.r#powered == true && self.r#rotation == 14 { return 10727; }
        if self.r#powered == true && self.r#rotation == 5 { return 10718; }
        if self.r#powered == false && self.r#rotation == 4 { return 10733; }
        if self.r#rotation == 9 && self.r#powered == true { return 10722; }
        if self.r#rotation == 1 && self.r#powered == false { return 10730; }
        if self.r#powered == true && self.r#rotation == 15 { return 10728; }
        if self.r#powered == false && self.r#rotation == 6 { return 10735; }
        if self.r#rotation == 8 && self.r#powered == true { return 10721; }
        if self.r#powered == true && self.r#rotation == 0 { return 10713; }
        if self.r#powered == false && self.r#rotation == 11 { return 10740; }
        if self.r#powered == false && self.r#rotation == 0 { return 10729; }
        if self.r#rotation == 14 && self.r#powered == false { return 10743; }
        if self.r#rotation == 12 && self.r#powered == false { return 10741; }
        if self.r#powered == false && self.r#rotation == 13 { return 10742; }
        if self.r#rotation == 15 && self.r#powered == false { return 10744; }
        if self.r#rotation == 5 && self.r#powered == false { return 10734; }
        if self.r#powered == true && self.r#rotation == 7 { return 10720; }
        if self.r#rotation == 11 && self.r#powered == true { return 10724; }
        if self.r#rotation == 12 && self.r#powered == true { return 10725; }
        if self.r#powered == false && self.r#rotation == 8 { return 10737; }
        if self.r#rotation == 4 && self.r#powered == true { return 10717; }
        if self.r#powered == true && self.r#rotation == 6 { return 10719; }
        if self.r#rotation == 10 && self.r#powered == true { return 10723; }
        if self.r#rotation == 13 && self.r#powered == true { return 10726; }
        if self.r#powered == false && self.r#rotation == 7 { return 10736; }
        if self.r#powered == false && self.r#rotation == 10 { return 10739; }
        if self.r#powered == true && self.r#rotation == 1 { return 10714; }
        if self.r#powered == false && self.r#rotation == 9 { return 10738; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10715 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 2,
            });
        }
        if state_id == 10731 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 2,
            });
        }
        if state_id == 10732 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 3,
            });
        }
        if state_id == 10716 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 3,
            });
        }
        if state_id == 10727 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 14,
            });
        }
        if state_id == 10718 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 5,
            });
        }
        if state_id == 10733 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 4,
            });
        }
        if state_id == 10722 {
            return Some(SkeletonSkull {
                r#rotation: 9,
                r#powered: true,
            });
        }
        if state_id == 10730 {
            return Some(SkeletonSkull {
                r#rotation: 1,
                r#powered: false,
            });
        }
        if state_id == 10728 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 15,
            });
        }
        if state_id == 10735 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 6,
            });
        }
        if state_id == 10721 {
            return Some(SkeletonSkull {
                r#rotation: 8,
                r#powered: true,
            });
        }
        if state_id == 10713 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 0,
            });
        }
        if state_id == 10740 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 11,
            });
        }
        if state_id == 10729 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 0,
            });
        }
        if state_id == 10743 {
            return Some(SkeletonSkull {
                r#rotation: 14,
                r#powered: false,
            });
        }
        if state_id == 10741 {
            return Some(SkeletonSkull {
                r#rotation: 12,
                r#powered: false,
            });
        }
        if state_id == 10742 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 13,
            });
        }
        if state_id == 10744 {
            return Some(SkeletonSkull {
                r#rotation: 15,
                r#powered: false,
            });
        }
        if state_id == 10734 {
            return Some(SkeletonSkull {
                r#rotation: 5,
                r#powered: false,
            });
        }
        if state_id == 10720 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 7,
            });
        }
        if state_id == 10724 {
            return Some(SkeletonSkull {
                r#rotation: 11,
                r#powered: true,
            });
        }
        if state_id == 10725 {
            return Some(SkeletonSkull {
                r#rotation: 12,
                r#powered: true,
            });
        }
        if state_id == 10737 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 8,
            });
        }
        if state_id == 10717 {
            return Some(SkeletonSkull {
                r#rotation: 4,
                r#powered: true,
            });
        }
        if state_id == 10719 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 6,
            });
        }
        if state_id == 10723 {
            return Some(SkeletonSkull {
                r#rotation: 10,
                r#powered: true,
            });
        }
        if state_id == 10726 {
            return Some(SkeletonSkull {
                r#rotation: 13,
                r#powered: true,
            });
        }
        if state_id == 10736 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 7,
            });
        }
        if state_id == 10739 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 10,
            });
        }
        if state_id == 10714 {
            return Some(SkeletonSkull {
                r#powered: true,
                r#rotation: 1,
            });
        }
        if state_id == 10738 {
            return Some(SkeletonSkull {
                r#powered: false,
                r#rotation: 9,
            });
        }
        return None;
    }
}

