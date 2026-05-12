use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitherSkeletonSkull {
    pub powered: bool,
    pub rotation: i32,
}


impl BlockState for WitherSkeletonSkull {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#rotation == 13 { return 10766; }
        if block_state.r#powered == false && block_state.r#rotation == 15 { return 10784; }
        if block_state.r#powered == true && block_state.r#rotation == 8 { return 10761; }
        if block_state.r#rotation == 1 && block_state.r#powered == true { return 10754; }
        if block_state.r#rotation == 2 && block_state.r#powered == false { return 10771; }
        if block_state.r#powered == true && block_state.r#rotation == 12 { return 10765; }
        if block_state.r#powered == true && block_state.r#rotation == 10 { return 10763; }
        if block_state.r#rotation == 15 && block_state.r#powered == true { return 10768; }
        if block_state.r#powered == false && block_state.r#rotation == 10 { return 10779; }
        if block_state.r#powered == false && block_state.r#rotation == 4 { return 10773; }
        if block_state.r#rotation == 5 && block_state.r#powered == true { return 10758; }
        if block_state.r#rotation == 7 && block_state.r#powered == false { return 10776; }
        if block_state.r#rotation == 14 && block_state.r#powered == false { return 10783; }
        if block_state.r#powered == true && block_state.r#rotation == 4 { return 10757; }
        if block_state.r#powered == false && block_state.r#rotation == 9 { return 10778; }
        if block_state.r#powered == true && block_state.r#rotation == 11 { return 10764; }
        if block_state.r#powered == false && block_state.r#rotation == 3 { return 10772; }
        if block_state.r#rotation == 2 && block_state.r#powered == true { return 10755; }
        if block_state.r#powered == true && block_state.r#rotation == 7 { return 10760; }
        if block_state.r#rotation == 9 && block_state.r#powered == true { return 10762; }
        if block_state.r#rotation == 8 && block_state.r#powered == false { return 10777; }
        if block_state.r#rotation == 0 && block_state.r#powered == false { return 10769; }
        if block_state.r#powered == true && block_state.r#rotation == 3 { return 10756; }
        if block_state.r#rotation == 12 && block_state.r#powered == false { return 10781; }
        if block_state.r#powered == true && block_state.r#rotation == 0 { return 10753; }
        if block_state.r#powered == false && block_state.r#rotation == 1 { return 10770; }
        if block_state.r#rotation == 5 && block_state.r#powered == false { return 10774; }
        if block_state.r#powered == false && block_state.r#rotation == 11 { return 10780; }
        if block_state.r#rotation == 6 && block_state.r#powered == false { return 10775; }
        if block_state.r#powered == false && block_state.r#rotation == 13 { return 10782; }
        if block_state.r#rotation == 6 && block_state.r#powered == true { return 10759; }
        if block_state.r#powered == true && block_state.r#rotation == 14 { return 10767; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10766 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 13,
            });
        }
        if state_id == 10784 {
            return Some(WitherSkeletonSkull {
                r#powered: false,
                r#rotation: 15,
            });
        }
        if state_id == 10761 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 8,
            });
        }
        if state_id == 10754 {
            return Some(WitherSkeletonSkull {
                r#rotation: 1,
                r#powered: true,
            });
        }
        if state_id == 10771 {
            return Some(WitherSkeletonSkull {
                r#rotation: 2,
                r#powered: false,
            });
        }
        if state_id == 10765 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 12,
            });
        }
        if state_id == 10763 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 10,
            });
        }
        if state_id == 10768 {
            return Some(WitherSkeletonSkull {
                r#rotation: 15,
                r#powered: true,
            });
        }
        if state_id == 10779 {
            return Some(WitherSkeletonSkull {
                r#powered: false,
                r#rotation: 10,
            });
        }
        if state_id == 10773 {
            return Some(WitherSkeletonSkull {
                r#powered: false,
                r#rotation: 4,
            });
        }
        if state_id == 10758 {
            return Some(WitherSkeletonSkull {
                r#rotation: 5,
                r#powered: true,
            });
        }
        if state_id == 10776 {
            return Some(WitherSkeletonSkull {
                r#rotation: 7,
                r#powered: false,
            });
        }
        if state_id == 10783 {
            return Some(WitherSkeletonSkull {
                r#rotation: 14,
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
        if state_id == 10764 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 11,
            });
        }
        if state_id == 10772 {
            return Some(WitherSkeletonSkull {
                r#powered: false,
                r#rotation: 3,
            });
        }
        if state_id == 10755 {
            return Some(WitherSkeletonSkull {
                r#rotation: 2,
                r#powered: true,
            });
        }
        if state_id == 10760 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 7,
            });
        }
        if state_id == 10762 {
            return Some(WitherSkeletonSkull {
                r#rotation: 9,
                r#powered: true,
            });
        }
        if state_id == 10777 {
            return Some(WitherSkeletonSkull {
                r#rotation: 8,
                r#powered: false,
            });
        }
        if state_id == 10769 {
            return Some(WitherSkeletonSkull {
                r#rotation: 0,
                r#powered: false,
            });
        }
        if state_id == 10756 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 3,
            });
        }
        if state_id == 10781 {
            return Some(WitherSkeletonSkull {
                r#rotation: 12,
                r#powered: false,
            });
        }
        if state_id == 10753 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 0,
            });
        }
        if state_id == 10770 {
            return Some(WitherSkeletonSkull {
                r#powered: false,
                r#rotation: 1,
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
                r#powered: false,
                r#rotation: 11,
            });
        }
        if state_id == 10775 {
            return Some(WitherSkeletonSkull {
                r#rotation: 6,
                r#powered: false,
            });
        }
        if state_id == 10782 {
            return Some(WitherSkeletonSkull {
                r#powered: false,
                r#rotation: 13,
            });
        }
        if state_id == 10759 {
            return Some(WitherSkeletonSkull {
                r#rotation: 6,
                r#powered: true,
            });
        }
        if state_id == 10767 {
            return Some(WitherSkeletonSkull {
                r#powered: true,
                r#rotation: 14,
            });
        }
        return None;
    }
}

