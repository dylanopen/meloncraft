use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakHangingSign {
    pub waterlogged: bool,
    pub attached: bool,
    pub rotation: i32,
}


impl BlockState for OakHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#rotation == 8 && block_state.r#attached == false { return 5755; }
        if block_state.r#attached == true && block_state.r#rotation == 13 && block_state.r#waterlogged == true { return 5732; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == false && block_state.r#attached == false { return 5753; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 4 && block_state.r#attached == false { return 5746; }
        if block_state.r#attached == true && block_state.r#rotation == 3 && block_state.r#waterlogged == true { return 5712; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == false && block_state.r#attached == false { return 5745; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 10 { return 5758; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 7 && block_state.r#attached == true { return 5721; }
        if block_state.r#attached == true && block_state.r#rotation == 9 && block_state.r#waterlogged == false { return 5725; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == true && block_state.r#attached == false { return 5752; }
        if block_state.r#rotation == 15 && block_state.r#attached == true && block_state.r#waterlogged == false { return 5737; }
        if block_state.r#rotation == 11 && block_state.r#waterlogged == true && block_state.r#attached == true { return 5728; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5731; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 2 { return 5711; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 0 && block_state.r#attached == true { return 5706; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 5 { return 5717; }
        if block_state.r#attached == false && block_state.r#rotation == 15 && block_state.r#waterlogged == true { return 5768; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 9 { return 5724; }
        if block_state.r#rotation == 8 && block_state.r#attached == true && block_state.r#waterlogged == true { return 5722; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 8 { return 5723; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 2 && block_state.r#attached == true { return 5710; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 10 { return 5726; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 1 && block_state.r#attached == false { return 5740; }
        if block_state.r#attached == false && block_state.r#rotation == 9 && block_state.r#waterlogged == true { return 5756; }
        if block_state.r#attached == true && block_state.r#rotation == 10 && block_state.r#waterlogged == false { return 5727; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == true && block_state.r#attached == true { return 5718; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 13 { return 5733; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 14 && block_state.r#attached == false { return 5767; }
        if block_state.r#rotation == 11 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5729; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == true && block_state.r#attached == false { return 5742; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5719; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == true && block_state.r#attached == true { return 5708; }
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 10 { return 5759; }
        if block_state.r#rotation == 13 && block_state.r#waterlogged == false && block_state.r#attached == false { return 5765; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == false && block_state.r#attached == false { return 5751; }
        if block_state.r#rotation == 14 && block_state.r#attached == true && block_state.r#waterlogged == true { return 5734; }
        if block_state.r#rotation == 12 && block_state.r#attached == false && block_state.r#waterlogged == false { return 5763; }
        if block_state.r#rotation == 9 && block_state.r#attached == false && block_state.r#waterlogged == false { return 5757; }
        if block_state.r#attached == false && block_state.r#rotation == 6 && block_state.r#waterlogged == true { return 5750; }
        if block_state.r#rotation == 14 && block_state.r#attached == true && block_state.r#waterlogged == false { return 5735; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == true && block_state.r#attached == true { return 5736; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 5 { return 5749; }
        if block_state.r#rotation == 0 && block_state.r#attached == false && block_state.r#waterlogged == true { return 5738; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 12 { return 5762; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 8 && block_state.r#attached == false { return 5754; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 5 { return 5716; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == true && block_state.r#attached == true { return 5720; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 5 && block_state.r#attached == false { return 5748; }
        if block_state.r#rotation == 13 && block_state.r#waterlogged == true && block_state.r#attached == false { return 5764; }
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 15 { return 5769; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 3 { return 5713; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5709; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 4 && block_state.r#attached == true { return 5714; }
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 4 { return 5747; }
        if block_state.r#rotation == 1 && block_state.r#attached == false && block_state.r#waterlogged == false { return 5741; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 11 { return 5761; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 3 { return 5744; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 0 { return 5707; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 12 { return 5730; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 4 { return 5715; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 11 { return 5760; }
        if block_state.r#rotation == 0 && block_state.r#attached == false && block_state.r#waterlogged == false { return 5739; }
        if block_state.r#attached == false && block_state.r#rotation == 2 && block_state.r#waterlogged == false { return 5743; }
        if block_state.r#rotation == 14 && block_state.r#attached == false && block_state.r#waterlogged == true { return 5766; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5755 {
            return Some(OakHangingSign {
                r#waterlogged: false,
                r#rotation: 8,
                r#attached: false,
            });
        }
        if state_id == 5732 {
            return Some(OakHangingSign {
                r#attached: true,
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 5753 {
            return Some(OakHangingSign {
                r#rotation: 7,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5746 {
            return Some(OakHangingSign {
                r#waterlogged: true,
                r#rotation: 4,
                r#attached: false,
            });
        }
        if state_id == 5712 {
            return Some(OakHangingSign {
                r#attached: true,
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 5745 {
            return Some(OakHangingSign {
                r#rotation: 3,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5758 {
            return Some(OakHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 10,
            });
        }
        if state_id == 5721 {
            return Some(OakHangingSign {
                r#waterlogged: false,
                r#rotation: 7,
                r#attached: true,
            });
        }
        if state_id == 5725 {
            return Some(OakHangingSign {
                r#attached: true,
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5752 {
            return Some(OakHangingSign {
                r#rotation: 7,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5737 {
            return Some(OakHangingSign {
                r#rotation: 15,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 5728 {
            return Some(OakHangingSign {
                r#rotation: 11,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5731 {
            return Some(OakHangingSign {
                r#rotation: 12,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5711 {
            return Some(OakHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 2,
            });
        }
        if state_id == 5706 {
            return Some(OakHangingSign {
                r#waterlogged: true,
                r#rotation: 0,
                r#attached: true,
            });
        }
        if state_id == 5717 {
            return Some(OakHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 5,
            });
        }
        if state_id == 5768 {
            return Some(OakHangingSign {
                r#attached: false,
                r#rotation: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 5724 {
            return Some(OakHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 9,
            });
        }
        if state_id == 5722 {
            return Some(OakHangingSign {
                r#rotation: 8,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 5723 {
            return Some(OakHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 8,
            });
        }
        if state_id == 5710 {
            return Some(OakHangingSign {
                r#waterlogged: true,
                r#rotation: 2,
                r#attached: true,
            });
        }
        if state_id == 5726 {
            return Some(OakHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 10,
            });
        }
        if state_id == 5740 {
            return Some(OakHangingSign {
                r#waterlogged: true,
                r#rotation: 1,
                r#attached: false,
            });
        }
        if state_id == 5756 {
            return Some(OakHangingSign {
                r#attached: false,
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 5727 {
            return Some(OakHangingSign {
                r#attached: true,
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 5718 {
            return Some(OakHangingSign {
                r#rotation: 6,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5733 {
            return Some(OakHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 13,
            });
        }
        if state_id == 5767 {
            return Some(OakHangingSign {
                r#waterlogged: false,
                r#rotation: 14,
                r#attached: false,
            });
        }
        if state_id == 5729 {
            return Some(OakHangingSign {
                r#rotation: 11,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5742 {
            return Some(OakHangingSign {
                r#rotation: 2,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5719 {
            return Some(OakHangingSign {
                r#rotation: 6,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5708 {
            return Some(OakHangingSign {
                r#rotation: 1,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5759 {
            return Some(OakHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 10,
            });
        }
        if state_id == 5765 {
            return Some(OakHangingSign {
                r#rotation: 13,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5751 {
            return Some(OakHangingSign {
                r#rotation: 6,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5734 {
            return Some(OakHangingSign {
                r#rotation: 14,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 5763 {
            return Some(OakHangingSign {
                r#rotation: 12,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5757 {
            return Some(OakHangingSign {
                r#rotation: 9,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5750 {
            return Some(OakHangingSign {
                r#attached: false,
                r#rotation: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 5735 {
            return Some(OakHangingSign {
                r#rotation: 14,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 5736 {
            return Some(OakHangingSign {
                r#rotation: 15,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5749 {
            return Some(OakHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 5,
            });
        }
        if state_id == 5738 {
            return Some(OakHangingSign {
                r#rotation: 0,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 5762 {
            return Some(OakHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 12,
            });
        }
        if state_id == 5754 {
            return Some(OakHangingSign {
                r#waterlogged: true,
                r#rotation: 8,
                r#attached: false,
            });
        }
        if state_id == 5716 {
            return Some(OakHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 5,
            });
        }
        if state_id == 5720 {
            return Some(OakHangingSign {
                r#rotation: 7,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5748 {
            return Some(OakHangingSign {
                r#waterlogged: true,
                r#rotation: 5,
                r#attached: false,
            });
        }
        if state_id == 5764 {
            return Some(OakHangingSign {
                r#rotation: 13,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5769 {
            return Some(OakHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 15,
            });
        }
        if state_id == 5713 {
            return Some(OakHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 3,
            });
        }
        if state_id == 5709 {
            return Some(OakHangingSign {
                r#rotation: 1,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5714 {
            return Some(OakHangingSign {
                r#waterlogged: true,
                r#rotation: 4,
                r#attached: true,
            });
        }
        if state_id == 5747 {
            return Some(OakHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 4,
            });
        }
        if state_id == 5741 {
            return Some(OakHangingSign {
                r#rotation: 1,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5761 {
            return Some(OakHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 11,
            });
        }
        if state_id == 5744 {
            return Some(OakHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 3,
            });
        }
        if state_id == 5707 {
            return Some(OakHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 0,
            });
        }
        if state_id == 5730 {
            return Some(OakHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 12,
            });
        }
        if state_id == 5715 {
            return Some(OakHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 4,
            });
        }
        if state_id == 5760 {
            return Some(OakHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 11,
            });
        }
        if state_id == 5739 {
            return Some(OakHangingSign {
                r#rotation: 0,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5743 {
            return Some(OakHangingSign {
                r#attached: false,
                r#rotation: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 5766 {
            return Some(OakHangingSign {
                r#rotation: 14,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

