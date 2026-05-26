use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaHangingSign {
    pub rotation: i32,
    pub waterlogged: bool,
    pub attached: bool,
}

impl BlockState for AcaciaHangingSign {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#attached == true && self.r#rotation == 3 {
            return 5904;
        }
        if self.r#rotation == 11 && self.r#waterlogged == false && self.r#attached == false {
            return 5953;
        }
        if self.r#rotation == 11 && self.r#attached == true && self.r#waterlogged == false {
            return 5921;
        }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 1 {
            return 5933;
        }
        if self.r#rotation == 8 && self.r#waterlogged == true && self.r#attached == true {
            return 5914;
        }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 13 {
            return 5956;
        }
        if self.r#attached == true && self.r#rotation == 1 && self.r#waterlogged == true {
            return 5900;
        }
        if self.r#attached == true && self.r#rotation == 5 && self.r#waterlogged == true {
            return 5908;
        }
        if self.r#attached == false && self.r#rotation == 14 && self.r#waterlogged == true {
            return 5958;
        }
        if self.r#rotation == 15 && self.r#attached == false && self.r#waterlogged == false {
            return 5961;
        }
        if self.r#attached == true && self.r#rotation == 15 && self.r#waterlogged == false {
            return 5929;
        }
        if self.r#rotation == 6 && self.r#attached == true && self.r#waterlogged == false {
            return 5911;
        }
        if self.r#attached == true && self.r#rotation == 8 && self.r#waterlogged == false {
            return 5915;
        }
        if self.r#waterlogged == false && self.r#rotation == 9 && self.r#attached == true {
            return 5917;
        }
        if self.r#rotation == 4 && self.r#attached == false && self.r#waterlogged == true {
            return 5938;
        }
        if self.r#rotation == 10 && self.r#attached == true && self.r#waterlogged == true {
            return 5918;
        }
        if self.r#rotation == 4 && self.r#waterlogged == true && self.r#attached == true {
            return 5906;
        }
        if self.r#attached == false && self.r#waterlogged == false && self.r#rotation == 7 {
            return 5945;
        }
        if self.r#attached == false && self.r#rotation == 3 && self.r#waterlogged == true {
            return 5936;
        }
        if self.r#attached == false && self.r#waterlogged == false && self.r#rotation == 6 {
            return 5943;
        }
        if self.r#attached == true && self.r#rotation == 5 && self.r#waterlogged == false {
            return 5909;
        }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 0 {
            return 5930;
        }
        if self.r#waterlogged == true && self.r#attached == true && self.r#rotation == 9 {
            return 5916;
        }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 9 {
            return 5949;
        }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 4 {
            return 5907;
        }
        if self.r#attached == false && self.r#rotation == 10 && self.r#waterlogged == true {
            return 5950;
        }
        if self.r#attached == true && self.r#rotation == 13 && self.r#waterlogged == true {
            return 5924;
        }
        if self.r#attached == false && self.r#rotation == 2 && self.r#waterlogged == false {
            return 5935;
        }
        if self.r#rotation == 5 && self.r#waterlogged == true && self.r#attached == false {
            return 5940;
        }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 12 {
            return 5923;
        }
        if self.r#rotation == 5 && self.r#attached == false && self.r#waterlogged == false {
            return 5941;
        }
        if self.r#attached == false && self.r#rotation == 13 && self.r#waterlogged == false {
            return 5957;
        }
        if self.r#attached == true && self.r#rotation == 2 && self.r#waterlogged == false {
            return 5903;
        }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 10 {
            return 5919;
        }
        if self.r#rotation == 1 && self.r#attached == false && self.r#waterlogged == true {
            return 5932;
        }
        if self.r#waterlogged == true && self.r#attached == true && self.r#rotation == 11 {
            return 5920;
        }
        if self.r#waterlogged == true && self.r#attached == true && self.r#rotation == 6 {
            return 5910;
        }
        if self.r#rotation == 8 && self.r#waterlogged == false && self.r#attached == false {
            return 5947;
        }
        if self.r#rotation == 14 && self.r#attached == true && self.r#waterlogged == false {
            return 5927;
        }
        if self.r#rotation == 15 && self.r#attached == false && self.r#waterlogged == true {
            return 5960;
        }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 8 {
            return 5946;
        }
        if self.r#attached == true && self.r#rotation == 0 && self.r#waterlogged == false {
            return 5899;
        }
        if self.r#attached == false && self.r#rotation == 2 && self.r#waterlogged == true {
            return 5934;
        }
        if self.r#rotation == 0 && self.r#attached == false && self.r#waterlogged == false {
            return 5931;
        }
        if self.r#waterlogged == true && self.r#attached == true && self.r#rotation == 0 {
            return 5898;
        }
        if self.r#attached == true && self.r#waterlogged == true && self.r#rotation == 14 {
            return 5926;
        }
        if self.r#rotation == 1 && self.r#waterlogged == false && self.r#attached == true {
            return 5901;
        }
        if self.r#rotation == 3 && self.r#waterlogged == false && self.r#attached == true {
            return 5905;
        }
        if self.r#rotation == 6 && self.r#waterlogged == true && self.r#attached == false {
            return 5942;
        }
        if self.r#attached == true && self.r#rotation == 7 && self.r#waterlogged == true {
            return 5912;
        }
        if self.r#attached == true && self.r#rotation == 13 && self.r#waterlogged == false {
            return 5925;
        }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 3 {
            return 5937;
        }
        if self.r#attached == false && self.r#rotation == 14 && self.r#waterlogged == false {
            return 5959;
        }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 7 {
            return 5913;
        }
        if self.r#waterlogged == false && self.r#rotation == 4 && self.r#attached == false {
            return 5939;
        }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 9 {
            return 5948;
        }
        if self.r#waterlogged == true && self.r#attached == true && self.r#rotation == 2 {
            return 5902;
        }
        if self.r#attached == true && self.r#waterlogged == true && self.r#rotation == 15 {
            return 5928;
        }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 7 {
            return 5944;
        }
        if self.r#rotation == 12 && self.r#waterlogged == true && self.r#attached == false {
            return 5954;
        }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 12 {
            return 5955;
        }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 11 {
            return 5952;
        }
        if self.r#rotation == 10 && self.r#waterlogged == false && self.r#attached == false {
            return 5951;
        }
        if self.r#attached == true && self.r#rotation == 12 && self.r#waterlogged == true {
            return 5922;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5904 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 3,
            });
        }
        if state_id == 5953 {
            return Some(AcaciaHangingSign {
                r#rotation: 11,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5921 {
            return Some(AcaciaHangingSign {
                r#rotation: 11,
                r#attached: true,
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
        if state_id == 5914 {
            return Some(AcaciaHangingSign {
                r#rotation: 8,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5956 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 13,
            });
        }
        if state_id == 5900 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 5908 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 5958 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 5961 {
            return Some(AcaciaHangingSign {
                r#rotation: 15,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5929 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 5911 {
            return Some(AcaciaHangingSign {
                r#rotation: 6,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 5915 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 5917 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#rotation: 9,
                r#attached: true,
            });
        }
        if state_id == 5938 {
            return Some(AcaciaHangingSign {
                r#rotation: 4,
                r#attached: false,
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
        if state_id == 5906 {
            return Some(AcaciaHangingSign {
                r#rotation: 4,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5945 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 7,
            });
        }
        if state_id == 5936 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 5943 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        if state_id == 5909 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 5930 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 0,
            });
        }
        if state_id == 5916 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 9,
            });
        }
        if state_id == 5949 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 9,
            });
        }
        if state_id == 5907 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 4,
            });
        }
        if state_id == 5950 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 5924 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 5935 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#rotation: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 5940 {
            return Some(AcaciaHangingSign {
                r#rotation: 5,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5923 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 12,
            });
        }
        if state_id == 5941 {
            return Some(AcaciaHangingSign {
                r#rotation: 5,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5957 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#rotation: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 5903 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 5919 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 10,
            });
        }
        if state_id == 5932 {
            return Some(AcaciaHangingSign {
                r#rotation: 1,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 5920 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 11,
            });
        }
        if state_id == 5910 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 6,
            });
        }
        if state_id == 5947 {
            return Some(AcaciaHangingSign {
                r#rotation: 8,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5927 {
            return Some(AcaciaHangingSign {
                r#rotation: 14,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 5960 {
            return Some(AcaciaHangingSign {
                r#rotation: 15,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 5946 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 8,
            });
        }
        if state_id == 5899 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 5934 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 5931 {
            return Some(AcaciaHangingSign {
                r#rotation: 0,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5898 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 0,
            });
        }
        if state_id == 5926 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 14,
            });
        }
        if state_id == 5901 {
            return Some(AcaciaHangingSign {
                r#rotation: 1,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5905 {
            return Some(AcaciaHangingSign {
                r#rotation: 3,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5942 {
            return Some(AcaciaHangingSign {
                r#rotation: 6,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5912 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 5925 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 5937 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 3,
            });
        }
        if state_id == 5959 {
            return Some(AcaciaHangingSign {
                r#attached: false,
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 5913 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 7,
            });
        }
        if state_id == 5939 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#rotation: 4,
                r#attached: false,
            });
        }
        if state_id == 5948 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 9,
            });
        }
        if state_id == 5902 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 2,
            });
        }
        if state_id == 5928 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 15,
            });
        }
        if state_id == 5944 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 7,
            });
        }
        if state_id == 5954 {
            return Some(AcaciaHangingSign {
                r#rotation: 12,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5955 {
            return Some(AcaciaHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 12,
            });
        }
        if state_id == 5952 {
            return Some(AcaciaHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 11,
            });
        }
        if state_id == 5951 {
            return Some(AcaciaHangingSign {
                r#rotation: 10,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5922 {
            return Some(AcaciaHangingSign {
                r#attached: true,
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
