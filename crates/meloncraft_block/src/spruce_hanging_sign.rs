use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceHangingSign {
    pub rotation: i32,
    pub waterlogged: bool,
    pub attached: bool,
}


impl BlockState for SpruceHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 1 && block_state.r#waterlogged == true && block_state.r#attached == false { return 5804; }
        if block_state.r#attached == false && block_state.r#rotation == 6 && block_state.r#waterlogged == false { return 5815; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 10 { return 5791; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 3 { return 5777; }
        if block_state.r#attached == true && block_state.r#rotation == 12 && block_state.r#waterlogged == true { return 5794; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5787; }
        if block_state.r#rotation == 4 && block_state.r#attached == false && block_state.r#waterlogged == false { return 5811; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 7 && block_state.r#attached == false { return 5817; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 11 { return 5824; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == true && block_state.r#attached == false { return 5814; }
        if block_state.r#rotation == 7 && block_state.r#attached == true && block_state.r#waterlogged == false { return 5785; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == false && block_state.r#attached == false { return 5807; }
        if block_state.r#attached == false && block_state.r#rotation == 0 && block_state.r#waterlogged == false { return 5803; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 11 { return 5792; }
        if block_state.r#attached == true && block_state.r#rotation == 13 && block_state.r#waterlogged == true { return 5796; }
        if block_state.r#rotation == 5 && block_state.r#attached == false && block_state.r#waterlogged == true { return 5812; }
        if block_state.r#rotation == 12 && block_state.r#attached == false && block_state.r#waterlogged == false { return 5827; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 15 { return 5832; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 8 { return 5818; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 3 { return 5808; }
        if block_state.r#rotation == 13 && block_state.r#attached == false && block_state.r#waterlogged == false { return 5829; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == false && block_state.r#attached == false { return 5833; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 2 { return 5775; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 15 { return 5801; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 6 { return 5782; }
        if block_state.r#attached == true && block_state.r#rotation == 9 && block_state.r#waterlogged == true { return 5788; }
        if block_state.r#rotation == 9 && block_state.r#attached == false && block_state.r#waterlogged == true { return 5820; }
        if block_state.r#attached == true && block_state.r#rotation == 4 && block_state.r#waterlogged == false { return 5779; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 12 { return 5795; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 12 { return 5826; }
        if block_state.r#rotation == 13 && block_state.r#waterlogged == true && block_state.r#attached == false { return 5828; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 11 { return 5793; }
        if block_state.r#attached == false && block_state.r#rotation == 11 && block_state.r#waterlogged == false { return 5825; }
        if block_state.r#rotation == 0 && block_state.r#attached == true && block_state.r#waterlogged == true { return 5770; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == true && block_state.r#attached == true { return 5780; }
        if block_state.r#rotation == 8 && block_state.r#attached == true && block_state.r#waterlogged == true { return 5786; }
        if block_state.r#rotation == 5 && block_state.r#attached == false && block_state.r#waterlogged == false { return 5813; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == true && block_state.r#attached == true { return 5784; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 1 && block_state.r#attached == false { return 5805; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 3 { return 5776; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 5 { return 5781; }
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 10 { return 5823; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 14 && block_state.r#attached == true { return 5798; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 14 && block_state.r#attached == false { return 5831; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 15 { return 5800; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 2 { return 5774; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 14 { return 5830; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 1 { return 5773; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == true && block_state.r#attached == true { return 5772; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 14 && block_state.r#attached == true { return 5799; }
        if block_state.r#rotation == 7 && block_state.r#attached == false && block_state.r#waterlogged == true { return 5816; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == true && block_state.r#attached == false { return 5806; }
        if block_state.r#attached == false && block_state.r#rotation == 8 && block_state.r#waterlogged == false { return 5819; }
        if block_state.r#attached == true && block_state.r#rotation == 0 && block_state.r#waterlogged == false { return 5771; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 4 { return 5778; }
        if block_state.r#attached == false && block_state.r#rotation == 0 && block_state.r#waterlogged == true { return 5802; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 9 && block_state.r#attached == true { return 5789; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 10 { return 5790; }
        if block_state.r#attached == false && block_state.r#rotation == 9 && block_state.r#waterlogged == false { return 5821; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 4 { return 5810; }
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 3 { return 5809; }
        if block_state.r#rotation == 13 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5797; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 6 { return 5783; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 10 { return 5822; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5804 {
            return Some(SpruceHangingSign {
                r#rotation: 1,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5815 {
            return Some(SpruceHangingSign {
                r#attached: false,
                r#rotation: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 5791 {
            return Some(SpruceHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 10,
            });
        }
        if state_id == 5777 {
            return Some(SpruceHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 3,
            });
        }
        if state_id == 5794 {
            return Some(SpruceHangingSign {
                r#attached: true,
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 5787 {
            return Some(SpruceHangingSign {
                r#rotation: 8,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5811 {
            return Some(SpruceHangingSign {
                r#rotation: 4,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5817 {
            return Some(SpruceHangingSign {
                r#waterlogged: false,
                r#rotation: 7,
                r#attached: false,
            });
        }
        if state_id == 5824 {
            return Some(SpruceHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 11,
            });
        }
        if state_id == 5814 {
            return Some(SpruceHangingSign {
                r#rotation: 6,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5785 {
            return Some(SpruceHangingSign {
                r#rotation: 7,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 5807 {
            return Some(SpruceHangingSign {
                r#rotation: 2,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5803 {
            return Some(SpruceHangingSign {
                r#attached: false,
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 5792 {
            return Some(SpruceHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 11,
            });
        }
        if state_id == 5796 {
            return Some(SpruceHangingSign {
                r#attached: true,
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 5812 {
            return Some(SpruceHangingSign {
                r#rotation: 5,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 5827 {
            return Some(SpruceHangingSign {
                r#rotation: 12,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5832 {
            return Some(SpruceHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 15,
            });
        }
        if state_id == 5818 {
            return Some(SpruceHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 8,
            });
        }
        if state_id == 5808 {
            return Some(SpruceHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 3,
            });
        }
        if state_id == 5829 {
            return Some(SpruceHangingSign {
                r#rotation: 13,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5833 {
            return Some(SpruceHangingSign {
                r#rotation: 15,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5775 {
            return Some(SpruceHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 2,
            });
        }
        if state_id == 5801 {
            return Some(SpruceHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 15,
            });
        }
        if state_id == 5782 {
            return Some(SpruceHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 6,
            });
        }
        if state_id == 5788 {
            return Some(SpruceHangingSign {
                r#attached: true,
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 5820 {
            return Some(SpruceHangingSign {
                r#rotation: 9,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 5779 {
            return Some(SpruceHangingSign {
                r#attached: true,
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 5795 {
            return Some(SpruceHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 12,
            });
        }
        if state_id == 5826 {
            return Some(SpruceHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 12,
            });
        }
        if state_id == 5828 {
            return Some(SpruceHangingSign {
                r#rotation: 13,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5793 {
            return Some(SpruceHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 11,
            });
        }
        if state_id == 5825 {
            return Some(SpruceHangingSign {
                r#attached: false,
                r#rotation: 11,
                r#waterlogged: false,
            });
        }
        if state_id == 5770 {
            return Some(SpruceHangingSign {
                r#rotation: 0,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 5780 {
            return Some(SpruceHangingSign {
                r#rotation: 5,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5786 {
            return Some(SpruceHangingSign {
                r#rotation: 8,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 5813 {
            return Some(SpruceHangingSign {
                r#rotation: 5,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5784 {
            return Some(SpruceHangingSign {
                r#rotation: 7,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5805 {
            return Some(SpruceHangingSign {
                r#waterlogged: false,
                r#rotation: 1,
                r#attached: false,
            });
        }
        if state_id == 5776 {
            return Some(SpruceHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 3,
            });
        }
        if state_id == 5781 {
            return Some(SpruceHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 5,
            });
        }
        if state_id == 5823 {
            return Some(SpruceHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 10,
            });
        }
        if state_id == 5798 {
            return Some(SpruceHangingSign {
                r#waterlogged: true,
                r#rotation: 14,
                r#attached: true,
            });
        }
        if state_id == 5831 {
            return Some(SpruceHangingSign {
                r#waterlogged: false,
                r#rotation: 14,
                r#attached: false,
            });
        }
        if state_id == 5800 {
            return Some(SpruceHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 15,
            });
        }
        if state_id == 5774 {
            return Some(SpruceHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 5830 {
            return Some(SpruceHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 14,
            });
        }
        if state_id == 5773 {
            return Some(SpruceHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 1,
            });
        }
        if state_id == 5772 {
            return Some(SpruceHangingSign {
                r#rotation: 1,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5799 {
            return Some(SpruceHangingSign {
                r#waterlogged: false,
                r#rotation: 14,
                r#attached: true,
            });
        }
        if state_id == 5816 {
            return Some(SpruceHangingSign {
                r#rotation: 7,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 5806 {
            return Some(SpruceHangingSign {
                r#rotation: 2,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5819 {
            return Some(SpruceHangingSign {
                r#attached: false,
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 5771 {
            return Some(SpruceHangingSign {
                r#attached: true,
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 5778 {
            return Some(SpruceHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 4,
            });
        }
        if state_id == 5802 {
            return Some(SpruceHangingSign {
                r#attached: false,
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 5789 {
            return Some(SpruceHangingSign {
                r#waterlogged: false,
                r#rotation: 9,
                r#attached: true,
            });
        }
        if state_id == 5790 {
            return Some(SpruceHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 10,
            });
        }
        if state_id == 5821 {
            return Some(SpruceHangingSign {
                r#attached: false,
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5810 {
            return Some(SpruceHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 4,
            });
        }
        if state_id == 5809 {
            return Some(SpruceHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 3,
            });
        }
        if state_id == 5797 {
            return Some(SpruceHangingSign {
                r#rotation: 13,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5783 {
            return Some(SpruceHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 6,
            });
        }
        if state_id == 5822 {
            return Some(SpruceHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 10,
            });
        }
        return None;
    }
}

