use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaHangingSign {
    pub rotation: i32,
    pub waterlogged: bool,
    pub attached: bool,
}


impl BlockState for AcaciaHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 13 { return 5924; }
        if block_state.r#attached == false && block_state.r#rotation == 0 && block_state.r#waterlogged == false { return 5931; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == true && block_state.r#attached == true { return 5916; }
        if block_state.r#attached == false && block_state.r#rotation == 11 && block_state.r#waterlogged == false { return 5953; }
        if block_state.r#attached == false && block_state.r#rotation == 12 && block_state.r#waterlogged == true { return 5954; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 14 { return 5958; }
        if block_state.r#attached == false && block_state.r#rotation == 2 && block_state.r#waterlogged == true { return 5934; }
        if block_state.r#rotation == 10 && block_state.r#attached == true && block_state.r#waterlogged == true { return 5918; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 14 && block_state.r#attached == true { return 5926; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 2 { return 5935; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 10 && block_state.r#attached == true { return 5919; }
        if block_state.r#rotation == 8 && block_state.r#attached == false && block_state.r#waterlogged == true { return 5946; }
        if block_state.r#rotation == 3 && block_state.r#attached == false && block_state.r#waterlogged == true { return 5936; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5909; }
        if block_state.r#rotation == 5 && block_state.r#attached == false && block_state.r#waterlogged == true { return 5940; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 2 { return 5903; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 0 { return 5930; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 9 && block_state.r#attached == true { return 5917; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 1 { return 5901; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 4 { return 5938; }
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 4 { return 5939; }
        if block_state.r#attached == true && block_state.r#rotation == 12 && block_state.r#waterlogged == false { return 5923; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 8 { return 5914; }
        if block_state.r#attached == true && block_state.r#rotation == 15 && block_state.r#waterlogged == true { return 5928; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 13 && block_state.r#attached == false { return 5956; }
        if block_state.r#attached == true && block_state.r#rotation == 5 && block_state.r#waterlogged == true { return 5908; }
        if block_state.r#attached == true && block_state.r#rotation == 7 && block_state.r#waterlogged == false { return 5913; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 6 && block_state.r#attached == false { return 5942; }
        if block_state.r#attached == true && block_state.r#rotation == 0 && block_state.r#waterlogged == false { return 5899; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 1 { return 5933; }
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 8 { return 5947; }
        if block_state.r#rotation == 11 && block_state.r#attached == true && block_state.r#waterlogged == false { return 5921; }
        if block_state.r#attached == false && block_state.r#rotation == 11 && block_state.r#waterlogged == true { return 5952; }
        if block_state.r#rotation == 14 && block_state.r#attached == true && block_state.r#waterlogged == false { return 5927; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 12 && block_state.r#attached == false { return 5955; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 13 { return 5925; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == true && block_state.r#attached == false { return 5944; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == true && block_state.r#attached == true { return 5898; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 7 { return 5912; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 8 && block_state.r#attached == true { return 5915; }
        if block_state.r#attached == false && block_state.r#rotation == 7 && block_state.r#waterlogged == false { return 5945; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 9 { return 5948; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 6 { return 5911; }
        if block_state.r#attached == true && block_state.r#rotation == 3 && block_state.r#waterlogged == true { return 5904; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 4 { return 5906; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == true && block_state.r#attached == false { return 5950; }
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 5 { return 5941; }
        if block_state.r#rotation == 1 && block_state.r#attached == true && block_state.r#waterlogged == true { return 5900; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 12 { return 5922; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == false && block_state.r#attached == false { return 5943; }
        if block_state.r#attached == true && block_state.r#rotation == 11 && block_state.r#waterlogged == true { return 5920; }
        if block_state.r#attached == false && block_state.r#rotation == 14 && block_state.r#waterlogged == false { return 5959; }
        if block_state.r#rotation == 13 && block_state.r#waterlogged == false && block_state.r#attached == false { return 5957; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 6 { return 5910; }
        if block_state.r#rotation == 2 && block_state.r#attached == true && block_state.r#waterlogged == true { return 5902; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 9 && block_state.r#attached == false { return 5949; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5905; }
        if block_state.r#rotation == 1 && block_state.r#attached == false && block_state.r#waterlogged == true { return 5932; }
        if block_state.r#attached == false && block_state.r#rotation == 10 && block_state.r#waterlogged == false { return 5951; }
        if block_state.r#rotation == 15 && block_state.r#attached == true && block_state.r#waterlogged == false { return 5929; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == false && block_state.r#attached == false { return 5937; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == true && block_state.r#attached == false { return 5960; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5907; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 15 { return 5961; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5924 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 13,
            });
        }
        if state_id == 5931 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 5916 {
            return Some(AcaciaHangingSign {
                r#rotation: 9,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5953 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#rotation: 11,
                r#waterlogged: false,
            });
        }
        if state_id == 5954 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 5958 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 14,
            });
        }
        if state_id == 5934 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 5918 {
            return Some(AcaciaHangingSign {
                r#rotation: 10,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 5926 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#rotation: 14,
                r#attached: true,
            });
        }
        if state_id == 5935 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 2,
            });
        }
        if state_id == 5919 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#rotation: 10,
                r#attached: true,
            });
        }
        if state_id == 5946 {
            return Some(AcaciaHangingSign {
                r#rotation: 8,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 5936 {
            return Some(AcaciaHangingSign {
                r#rotation: 3,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 5909 {
            return Some(AcaciaHangingSign {
                r#rotation: 5,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5940 {
            return Some(AcaciaHangingSign {
                r#rotation: 5,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 5903 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 2,
            });
        }
        if state_id == 5930 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 0,
            });
        }
        if state_id == 5917 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#rotation: 9,
                r#attached: true,
            });
        }
        if state_id == 5901 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 1,
            });
        }
        if state_id == 5938 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 4,
            });
        }
        if state_id == 5939 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 4,
            });
        }
        if state_id == 5923 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 5914 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 8,
            });
        }
        if state_id == 5928 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 5956 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#rotation: 13,
                r#attached: false,
            });
        }
        if state_id == 5908 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 5913 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 5942 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#rotation: 6,
                r#attached: false,
            });
        }
        if state_id == 5899 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 5933 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 1,
            });
        }
        if state_id == 5947 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 8,
            });
        }
        if state_id == 5921 {
            return Some(AcaciaHangingSign {
                r#rotation: 11,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 5952 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 5927 {
            return Some(AcaciaHangingSign {
                r#rotation: 14,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 5955 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#rotation: 12,
                r#attached: false,
            });
        }
        if state_id == 5925 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 5944 {
            return Some(AcaciaHangingSign {
                r#rotation: 7,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5898 {
            return Some(AcaciaHangingSign {
                r#rotation: 0,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5912 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 7,
            });
        }
        if state_id == 5915 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#rotation: 8,
                r#attached: true,
            });
        }
        if state_id == 5945 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 5948 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 9,
            });
        }
        if state_id == 5911 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 6,
            });
        }
        if state_id == 5904 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 5906 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 4,
            });
        }
        if state_id == 5950 {
            return Some(AcaciaHangingSign {
                r#rotation: 10,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5941 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 5,
            });
        }
        if state_id == 5900 {
            return Some(AcaciaHangingSign {
                r#rotation: 1,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 5922 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 12,
            });
        }
        if state_id == 5943 {
            return Some(AcaciaHangingSign {
                r#rotation: 6,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5920 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 5959 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 5957 {
            return Some(AcaciaHangingSign {
                r#rotation: 13,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5910 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 6,
            });
        }
        if state_id == 5902 {
            return Some(AcaciaHangingSign {
                r#rotation: 2,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 5949 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#rotation: 9,
                r#attached: false,
            });
        }
        if state_id == 5905 {
            return Some(AcaciaHangingSign {
                r#rotation: 3,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5932 {
            return Some(AcaciaHangingSign {
                r#rotation: 1,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 5951 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 5929 {
            return Some(AcaciaHangingSign {
                r#rotation: 15,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 5937 {
            return Some(AcaciaHangingSign {
                r#rotation: 3,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5960 {
            return Some(AcaciaHangingSign {
                r#rotation: 15,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5907 {
            return Some(AcaciaHangingSign {
                r#rotation: 4,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5961 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 15,
            });
        }
        return None;
    }
}

