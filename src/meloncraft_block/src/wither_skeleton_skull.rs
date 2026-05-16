use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitherSkeletonSkull {
    pub powered: bool,
    pub rotation: i32,
}


impl BlockState for WitherSkeletonSkull {
    fn to_id(&self) -> i32 {
        if self.r#rotation == 8 && self.r#powered == true { return 10761; }
        if self.r#powered == false && self.r#rotation == 1 { return 10770; }
        if self.r#powered == false && self.r#rotation == 2 { return 10771; }
        if self.r#powered == true && self.r#rotation == 1 { return 10754; }
        if self.r#rotation == 0 && self.r#powered == false { return 10769; }
        if self.r#powered == false && self.r#rotation == 3 { return 10772; }
        if self.r#rotation == 10 && self.r#powered == false { return 10779; }
        if self.r#rotation == 15 && self.r#powered == false { return 10784; }
        if self.r#powered == true && self.r#rotation == 6 { return 10759; }
        if self.r#rotation == 2 && self.r#powered == true { return 10755; }
        if self.r#powered == false && self.r#rotation == 8 { return 10777; }
        if self.r#powered == false && self.r#rotation == 13 { return 10782; }
        if self.r#rotation == 14 && self.r#powered == false { return 10783; }
        if self.r#rotation == 7 && self.r#powered == true { return 10760; }
        if self.r#rotation == 11 && self.r#powered == true { return 10764; }
        if self.r#rotation == 4 && self.r#powered == false { return 10773; }
        if self.r#powered == true && self.r#rotation == 4 { return 10757; }
        if self.r#powered == false && self.r#rotation == 9 { return 10778; }
        if self.r#powered == true && self.r#rotation == 3 { return 10756; }
        if self.r#powered == true && self.r#rotation == 15 { return 10768; }
        if self.r#rotation == 5 && self.r#powered == true { return 10758; }
        if self.r#powered == true && self.r#rotation == 13 { return 10766; }
        if self.r#powered == true && self.r#rotation == 14 { return 10767; }
        if self.r#rotation == 9 && self.r#powered == true { return 10762; }
        if self.r#rotation == 6 && self.r#powered == false { return 10775; }
        if self.r#rotation == 10 && self.r#powered == true { return 10763; }
        if self.r#rotation == 7 && self.r#powered == false { return 10776; }
        if self.r#rotation == 5 && self.r#powered == false { return 10774; }
        if self.r#rotation == 11 && self.r#powered == false { return 10780; }
        if self.r#powered == true && self.r#rotation == 12 { return 10765; }
        if self.r#powered == true && self.r#rotation == 0 { return 10753; }
        if self.r#rotation == 12 && self.r#powered == false { return 10781; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10761 {
            return Some(WitherSkeletonSkull {
                r#rotation: 8,
                r#powered: true,
            });
        }
        if state_id == 10770 {
            return Some(WitherSkeletonSkull {
                r#powered: false,
                r#rotation: 1,
            });
        }
        if state_id == 10771 {
            return Some(WitherSkeletonSkull {
                r#powered: false,
                r#rotation: 2,
            });
        }
        if state_id == 10754 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 1,
            });
        }
        if state_id == 10769 {
            return Some(WitherSkeletonSkull {
                r#rotation: 0,
                r#powered: false,
            });
        }
        if state_id == 10772 {
            return Some(WitherSkeletonSkull {
                r#powered: false,
                r#rotation: 3,
            });
        }
        if state_id == 10779 {
            return Some(WitherSkeletonSkull {
                r#rotation: 10,
                r#powered: false,
            });
        }
        if state_id == 10784 {
            return Some(WitherSkeletonSkull {
                r#rotation: 15,
                r#powered: false,
            });
        }
        if state_id == 10759 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 6,
            });
        }
        if state_id == 10755 {
            return Some(WitherSkeletonSkull {
                r#rotation: 2,
                r#powered: true,
            });
        }
        if state_id == 10777 {
            return Some(WitherSkeletonSkull {
                r#powered: false,
                r#rotation: 8,
            });
        }
        if state_id == 10782 {
            return Some(WitherSkeletonSkull {
                r#powered: false,
                r#rotation: 13,
            });
        }
        if state_id == 10783 {
            return Some(WitherSkeletonSkull {
                r#rotation: 14,
                r#powered: false,
            });
        }
        if state_id == 10760 {
            return Some(WitherSkeletonSkull {
                r#rotation: 7,
                r#powered: true,
            });
        }
        if state_id == 10764 {
            return Some(WitherSkeletonSkull {
                r#rotation: 11,
                r#powered: true,
            });
        }
        if state_id == 10773 {
            return Some(WitherSkeletonSkull {
                r#rotation: 4,
                r#powered: false,
            });
        }
        if state_id == 10757 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 4,
            });
        }
        if state_id == 10778 {
            return Some(WitherSkeletonSkull {
                r#powered: false,
                r#rotation: 9,
            });
        }
        if state_id == 10756 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 3,
            });
        }
        if state_id == 10768 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 15,
            });
        }
        if state_id == 10758 {
            return Some(WitherSkeletonSkull {
                r#rotation: 5,
                r#powered: true,
            });
        }
        if state_id == 10766 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 13,
            });
        }
        if state_id == 10767 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 14,
            });
        }
        if state_id == 10762 {
            return Some(WitherSkeletonSkull {
                r#rotation: 9,
                r#powered: true,
            });
        }
        if state_id == 10775 {
            return Some(WitherSkeletonSkull {
                r#rotation: 6,
                r#powered: false,
            });
        }
        if state_id == 10763 {
            return Some(WitherSkeletonSkull {
                r#rotation: 10,
                r#powered: true,
            });
        }
        if state_id == 10776 {
            return Some(WitherSkeletonSkull {
                r#rotation: 7,
                r#powered: false,
            });
        }
        if state_id == 10774 {
            return Some(WitherSkeletonSkull {
                r#rotation: 5,
                r#powered: false,
            });
        }
        if state_id == 10780 {
            return Some(WitherSkeletonSkull {
                r#rotation: 11,
                r#powered: false,
            });
        }
        if state_id == 10765 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 12,
            });
        }
        if state_id == 10753 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 0,
            });
        }
        if state_id == 10781 {
            return Some(WitherSkeletonSkull {
                r#rotation: 12,
                r#powered: false,
            });
        }
        return None;
    }
}

