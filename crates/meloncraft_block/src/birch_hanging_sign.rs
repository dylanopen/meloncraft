use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchHangingSign {
    pub rotation: i32,
    pub attached: bool,
    pub waterlogged: bool,
}


impl BlockState for BirchHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 8 { return 5883; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 2 && block_state.r#attached == true { return 5839; }
        if block_state.r#attached == true && block_state.r#rotation == 15 && block_state.r#waterlogged == false { return 5865; }
        if block_state.r#attached == false && block_state.r#rotation == 10 && block_state.r#waterlogged == false { return 5887; }
        if block_state.r#attached == true && block_state.r#rotation == 9 && block_state.r#waterlogged == false { return 5853; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 14 { return 5862; }
        if block_state.r#attached == true && block_state.r#rotation == 13 && block_state.r#waterlogged == false { return 5861; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == false && block_state.r#attached == false { return 5871; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 14 { return 5863; }
        if block_state.r#rotation == 3 && block_state.r#attached == true && block_state.r#waterlogged == true { return 5840; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 15 && block_state.r#attached == false { return 5897; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == true && block_state.r#attached == false { return 5884; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == true && block_state.r#attached == true { return 5848; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 13 { return 5893; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 4 { return 5842; }
        if block_state.r#rotation == 1 && block_state.r#attached == true && block_state.r#waterlogged == false { return 5837; }
        if block_state.r#rotation == 6 && block_state.r#attached == false && block_state.r#waterlogged == false { return 5879; }
        if block_state.r#rotation == 10 && block_state.r#attached == true && block_state.r#waterlogged == false { return 5855; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 13 { return 5892; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 11 { return 5857; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5847; }
        if block_state.r#rotation == 3 && block_state.r#attached == false && block_state.r#waterlogged == true { return 5872; }
        if block_state.r#attached == true && block_state.r#rotation == 1 && block_state.r#waterlogged == true { return 5836; }
        if block_state.r#attached == true && block_state.r#rotation == 12 && block_state.r#waterlogged == true { return 5858; }
        if block_state.r#attached == false && block_state.r#rotation == 1 && block_state.r#waterlogged == false { return 5869; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 7 && block_state.r#attached == false { return 5881; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 8 && block_state.r#attached == false { return 5882; }
        if block_state.r#attached == false && block_state.r#rotation == 15 && block_state.r#waterlogged == true { return 5896; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 2 && block_state.r#attached == true { return 5838; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == true && block_state.r#attached == true { return 5850; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5845; }
        if block_state.r#rotation == 12 && block_state.r#attached == false && block_state.r#waterlogged == false { return 5891; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 14 { return 5894; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 13 && block_state.r#attached == true { return 5860; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 12 && block_state.r#attached == false { return 5890; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 10 { return 5854; }
        if block_state.r#rotation == 4 && block_state.r#attached == true && block_state.r#waterlogged == false { return 5843; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 3 && block_state.r#attached == false { return 5873; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 1 { return 5868; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 11 { return 5888; }
        if block_state.r#attached == true && block_state.r#rotation == 7 && block_state.r#waterlogged == false { return 5849; }
        if block_state.r#rotation == 5 && block_state.r#attached == true && block_state.r#waterlogged == true { return 5844; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == false && block_state.r#attached == false { return 5867; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 7 && block_state.r#attached == false { return 5880; }
        if block_state.r#attached == false && block_state.r#rotation == 5 && block_state.r#waterlogged == true { return 5876; }
        if block_state.r#attached == true && block_state.r#rotation == 0 && block_state.r#waterlogged == true { return 5834; }
        if block_state.r#rotation == 9 && block_state.r#attached == false && block_state.r#waterlogged == false { return 5885; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == false && block_state.r#attached == false { return 5895; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 15 && block_state.r#attached == true { return 5864; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 2 { return 5870; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == true && block_state.r#attached == true { return 5852; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 6 { return 5846; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 12 { return 5859; }
        if block_state.r#rotation == 4 && block_state.r#attached == false && block_state.r#waterlogged == true { return 5874; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == true && block_state.r#attached == false { return 5886; }
        if block_state.r#attached == true && block_state.r#rotation == 11 && block_state.r#waterlogged == true { return 5856; }
        if block_state.r#rotation == 3 && block_state.r#attached == true && block_state.r#waterlogged == false { return 5841; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5851; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 6 { return 5878; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 0 && block_state.r#attached == true { return 5835; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 0 { return 5866; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 11 { return 5889; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 4 { return 5875; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 5 { return 5877; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5883 {
            return Some(BirchHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 8,
            });
        }
        if state_id == 5839 {
            return Some(BirchHangingSign {
                r#waterlogged: false,
                r#rotation: 2,
                r#attached: true,
            });
        }
        if state_id == 5865 {
            return Some(BirchHangingSign {
                r#attached: true,
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 5887 {
            return Some(BirchHangingSign {
                r#attached: false,
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 5853 {
            return Some(BirchHangingSign {
                r#attached: true,
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5862 {
            return Some(BirchHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 14,
            });
        }
        if state_id == 5861 {
            return Some(BirchHangingSign {
                r#attached: true,
                r#rotation: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 5871 {
            return Some(BirchHangingSign {
                r#rotation: 2,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5863 {
            return Some(BirchHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 14,
            });
        }
        if state_id == 5840 {
            return Some(BirchHangingSign {
                r#rotation: 3,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 5897 {
            return Some(BirchHangingSign {
                r#waterlogged: false,
                r#rotation: 15,
                r#attached: false,
            });
        }
        if state_id == 5884 {
            return Some(BirchHangingSign {
                r#rotation: 9,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5848 {
            return Some(BirchHangingSign {
                r#rotation: 7,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5893 {
            return Some(BirchHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 13,
            });
        }
        if state_id == 5842 {
            return Some(BirchHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 4,
            });
        }
        if state_id == 5837 {
            return Some(BirchHangingSign {
                r#rotation: 1,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 5879 {
            return Some(BirchHangingSign {
                r#rotation: 6,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5855 {
            return Some(BirchHangingSign {
                r#rotation: 10,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 5892 {
            return Some(BirchHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 13,
            });
        }
        if state_id == 5857 {
            return Some(BirchHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 11,
            });
        }
        if state_id == 5847 {
            return Some(BirchHangingSign {
                r#rotation: 6,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5872 {
            return Some(BirchHangingSign {
                r#rotation: 3,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 5836 {
            return Some(BirchHangingSign {
                r#attached: true,
                r#rotation: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 5858 {
            return Some(BirchHangingSign {
                r#attached: true,
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 5869 {
            return Some(BirchHangingSign {
                r#attached: false,
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 5881 {
            return Some(BirchHangingSign {
                r#waterlogged: false,
                r#rotation: 7,
                r#attached: false,
            });
        }
        if state_id == 5882 {
            return Some(BirchHangingSign {
                r#waterlogged: true,
                r#rotation: 8,
                r#attached: false,
            });
        }
        if state_id == 5896 {
            return Some(BirchHangingSign {
                r#attached: false,
                r#rotation: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 5838 {
            return Some(BirchHangingSign {
                r#waterlogged: true,
                r#rotation: 2,
                r#attached: true,
            });
        }
        if state_id == 5850 {
            return Some(BirchHangingSign {
                r#rotation: 8,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5845 {
            return Some(BirchHangingSign {
                r#rotation: 5,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5891 {
            return Some(BirchHangingSign {
                r#rotation: 12,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5894 {
            return Some(BirchHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 14,
            });
        }
        if state_id == 5860 {
            return Some(BirchHangingSign {
                r#waterlogged: true,
                r#rotation: 13,
                r#attached: true,
            });
        }
        if state_id == 5890 {
            return Some(BirchHangingSign {
                r#waterlogged: true,
                r#rotation: 12,
                r#attached: false,
            });
        }
        if state_id == 5854 {
            return Some(BirchHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 10,
            });
        }
        if state_id == 5843 {
            return Some(BirchHangingSign {
                r#rotation: 4,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 5873 {
            return Some(BirchHangingSign {
                r#waterlogged: false,
                r#rotation: 3,
                r#attached: false,
            });
        }
        if state_id == 5868 {
            return Some(BirchHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 1,
            });
        }
        if state_id == 5888 {
            return Some(BirchHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 11,
            });
        }
        if state_id == 5849 {
            return Some(BirchHangingSign {
                r#attached: true,
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 5844 {
            return Some(BirchHangingSign {
                r#rotation: 5,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 5867 {
            return Some(BirchHangingSign {
                r#rotation: 0,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5880 {
            return Some(BirchHangingSign {
                r#waterlogged: true,
                r#rotation: 7,
                r#attached: false,
            });
        }
        if state_id == 5876 {
            return Some(BirchHangingSign {
                r#attached: false,
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 5834 {
            return Some(BirchHangingSign {
                r#attached: true,
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 5885 {
            return Some(BirchHangingSign {
                r#rotation: 9,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5895 {
            return Some(BirchHangingSign {
                r#rotation: 14,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5864 {
            return Some(BirchHangingSign {
                r#waterlogged: true,
                r#rotation: 15,
                r#attached: true,
            });
        }
        if state_id == 5870 {
            return Some(BirchHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 2,
            });
        }
        if state_id == 5852 {
            return Some(BirchHangingSign {
                r#rotation: 9,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5846 {
            return Some(BirchHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 6,
            });
        }
        if state_id == 5859 {
            return Some(BirchHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 12,
            });
        }
        if state_id == 5874 {
            return Some(BirchHangingSign {
                r#rotation: 4,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 5886 {
            return Some(BirchHangingSign {
                r#rotation: 10,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5856 {
            return Some(BirchHangingSign {
                r#attached: true,
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 5841 {
            return Some(BirchHangingSign {
                r#rotation: 3,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 5851 {
            return Some(BirchHangingSign {
                r#rotation: 8,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5878 {
            return Some(BirchHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 6,
            });
        }
        if state_id == 5835 {
            return Some(BirchHangingSign {
                r#waterlogged: false,
                r#rotation: 0,
                r#attached: true,
            });
        }
        if state_id == 5866 {
            return Some(BirchHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 0,
            });
        }
        if state_id == 5889 {
            return Some(BirchHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 11,
            });
        }
        if state_id == 5875 {
            return Some(BirchHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 4,
            });
        }
        if state_id == 5877 {
            return Some(BirchHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 5,
            });
        }
        return None;
    }
}

