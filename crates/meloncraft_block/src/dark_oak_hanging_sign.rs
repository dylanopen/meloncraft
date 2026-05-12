use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakHangingSign {
    pub attached: bool,
    pub rotation: i32,
    pub waterlogged: bool,
}


impl BlockState for DarkOakHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 5 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6101; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 8 { return 6107; }
        if block_state.r#rotation == 0 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6123; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 4 && block_state.r#attached == false { return 6130; }
        if block_state.r#attached == true && block_state.r#rotation == 2 && block_state.r#waterlogged == true { return 6094; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 15 { return 6120; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == true && block_state.r#attached == false { return 6128; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 7 { return 6136; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 5 && block_state.r#attached == true { return 6100; }
        if block_state.r#attached == false && block_state.r#rotation == 13 && block_state.r#waterlogged == true { return 6148; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6139; }
        if block_state.r#attached == true && block_state.r#rotation == 7 && block_state.r#waterlogged == false { return 6105; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 11 { return 6113; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 11 && block_state.r#attached == false { return 6144; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 6 && block_state.r#attached == false { return 6135; }
        if block_state.r#rotation == 15 && block_state.r#attached == false && block_state.r#waterlogged == true { return 6152; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 12 { return 6147; }
        if block_state.r#attached == true && block_state.r#rotation == 2 && block_state.r#waterlogged == false { return 6095; }
        if block_state.r#attached == true && block_state.r#rotation == 15 && block_state.r#waterlogged == false { return 6121; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 4 && block_state.r#attached == true { return 6099; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 6 && block_state.r#attached == true { return 6103; }
        if block_state.r#attached == false && block_state.r#rotation == 2 && block_state.r#waterlogged == true { return 6126; }
        if block_state.r#rotation == 14 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6151; }
        if block_state.r#attached == true && block_state.r#rotation == 6 && block_state.r#waterlogged == true { return 6102; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6091; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 10 { return 6110; }
        if block_state.r#attached == false && block_state.r#rotation == 8 && block_state.r#waterlogged == true { return 6138; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 7 && block_state.r#attached == false { return 6137; }
        if block_state.r#rotation == 13 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6149; }
        if block_state.r#rotation == 1 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6125; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 13 { return 6117; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 14 { return 6150; }
        if block_state.r#attached == true && block_state.r#rotation == 14 && block_state.r#waterlogged == false { return 6119; }
        if block_state.r#attached == true && block_state.r#rotation == 13 && block_state.r#waterlogged == true { return 6116; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 0 { return 6090; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 9 { return 6108; }
        if block_state.r#rotation == 11 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6145; }
        if block_state.r#rotation == 6 && block_state.r#attached == false && block_state.r#waterlogged == true { return 6134; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6093; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 11 { return 6112; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 10 { return 6111; }
        if block_state.r#attached == false && block_state.r#rotation == 9 && block_state.r#waterlogged == true { return 6140; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 3 { return 6129; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6109; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 14 && block_state.r#attached == true { return 6118; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 1 { return 6092; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 4 { return 6131; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == true && block_state.r#attached == true { return 6114; }
        if block_state.r#rotation == 8 && block_state.r#attached == true && block_state.r#waterlogged == true { return 6106; }
        if block_state.r#attached == false && block_state.r#rotation == 0 && block_state.r#waterlogged == true { return 6122; }
        if block_state.r#attached == false && block_state.r#rotation == 5 && block_state.r#waterlogged == true { return 6132; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6133; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 1 { return 6124; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 2 { return 6127; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6141; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == true && block_state.r#attached == true { return 6104; }
        if block_state.r#rotation == 15 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6153; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 4 { return 6098; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6143; }
        if block_state.r#rotation == 3 && block_state.r#attached == true && block_state.r#waterlogged == true { return 6096; }
        if block_state.r#rotation == 3 && block_state.r#attached == true && block_state.r#waterlogged == false { return 6097; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == true && block_state.r#attached == false { return 6146; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6115; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == true && block_state.r#attached == false { return 6142; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6101 {
            return Some(DarkOakHangingSign {
                r#rotation: 5,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6107 {
            return Some(DarkOakHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 8,
            });
        }
        if state_id == 6123 {
            return Some(DarkOakHangingSign {
                r#rotation: 0,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6130 {
            return Some(DarkOakHangingSign {
                r#waterlogged: true,
                r#rotation: 4,
                r#attached: false,
            });
        }
        if state_id == 6094 {
            return Some(DarkOakHangingSign {
                r#attached: true,
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 6120 {
            return Some(DarkOakHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 15,
            });
        }
        if state_id == 6128 {
            return Some(DarkOakHangingSign {
                r#rotation: 3,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6136 {
            return Some(DarkOakHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 7,
            });
        }
        if state_id == 6100 {
            return Some(DarkOakHangingSign {
                r#waterlogged: true,
                r#rotation: 5,
                r#attached: true,
            });
        }
        if state_id == 6148 {
            return Some(DarkOakHangingSign {
                r#attached: false,
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 6139 {
            return Some(DarkOakHangingSign {
                r#rotation: 8,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6105 {
            return Some(DarkOakHangingSign {
                r#attached: true,
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 6113 {
            return Some(DarkOakHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 11,
            });
        }
        if state_id == 6144 {
            return Some(DarkOakHangingSign {
                r#waterlogged: true,
                r#rotation: 11,
                r#attached: false,
            });
        }
        if state_id == 6135 {
            return Some(DarkOakHangingSign {
                r#waterlogged: false,
                r#rotation: 6,
                r#attached: false,
            });
        }
        if state_id == 6152 {
            return Some(DarkOakHangingSign {
                r#rotation: 15,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6147 {
            return Some(DarkOakHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 12,
            });
        }
        if state_id == 6095 {
            return Some(DarkOakHangingSign {
                r#attached: true,
                r#rotation: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 6121 {
            return Some(DarkOakHangingSign {
                r#attached: true,
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 6099 {
            return Some(DarkOakHangingSign {
                r#waterlogged: false,
                r#rotation: 4,
                r#attached: true,
            });
        }
        if state_id == 6103 {
            return Some(DarkOakHangingSign {
                r#waterlogged: false,
                r#rotation: 6,
                r#attached: true,
            });
        }
        if state_id == 6126 {
            return Some(DarkOakHangingSign {
                r#attached: false,
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 6151 {
            return Some(DarkOakHangingSign {
                r#rotation: 14,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6102 {
            return Some(DarkOakHangingSign {
                r#attached: true,
                r#rotation: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 6091 {
            return Some(DarkOakHangingSign {
                r#rotation: 0,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6110 {
            return Some(DarkOakHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 10,
            });
        }
        if state_id == 6138 {
            return Some(DarkOakHangingSign {
                r#attached: false,
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 6137 {
            return Some(DarkOakHangingSign {
                r#waterlogged: false,
                r#rotation: 7,
                r#attached: false,
            });
        }
        if state_id == 6149 {
            return Some(DarkOakHangingSign {
                r#rotation: 13,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6125 {
            return Some(DarkOakHangingSign {
                r#rotation: 1,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6117 {
            return Some(DarkOakHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 13,
            });
        }
        if state_id == 6150 {
            return Some(DarkOakHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 14,
            });
        }
        if state_id == 6119 {
            return Some(DarkOakHangingSign {
                r#attached: true,
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 6116 {
            return Some(DarkOakHangingSign {
                r#attached: true,
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 6090 {
            return Some(DarkOakHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 0,
            });
        }
        if state_id == 6108 {
            return Some(DarkOakHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 9,
            });
        }
        if state_id == 6145 {
            return Some(DarkOakHangingSign {
                r#rotation: 11,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6134 {
            return Some(DarkOakHangingSign {
                r#rotation: 6,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6093 {
            return Some(DarkOakHangingSign {
                r#rotation: 1,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6112 {
            return Some(DarkOakHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 11,
            });
        }
        if state_id == 6111 {
            return Some(DarkOakHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 10,
            });
        }
        if state_id == 6140 {
            return Some(DarkOakHangingSign {
                r#attached: false,
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 6129 {
            return Some(DarkOakHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 3,
            });
        }
        if state_id == 6109 {
            return Some(DarkOakHangingSign {
                r#rotation: 9,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6118 {
            return Some(DarkOakHangingSign {
                r#waterlogged: true,
                r#rotation: 14,
                r#attached: true,
            });
        }
        if state_id == 6092 {
            return Some(DarkOakHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 1,
            });
        }
        if state_id == 6131 {
            return Some(DarkOakHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 4,
            });
        }
        if state_id == 6114 {
            return Some(DarkOakHangingSign {
                r#rotation: 12,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6106 {
            return Some(DarkOakHangingSign {
                r#rotation: 8,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6122 {
            return Some(DarkOakHangingSign {
                r#attached: false,
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 6132 {
            return Some(DarkOakHangingSign {
                r#attached: false,
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 6133 {
            return Some(DarkOakHangingSign {
                r#rotation: 5,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6124 {
            return Some(DarkOakHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 1,
            });
        }
        if state_id == 6127 {
            return Some(DarkOakHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 2,
            });
        }
        if state_id == 6141 {
            return Some(DarkOakHangingSign {
                r#rotation: 9,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6104 {
            return Some(DarkOakHangingSign {
                r#rotation: 7,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6153 {
            return Some(DarkOakHangingSign {
                r#rotation: 15,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6098 {
            return Some(DarkOakHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 4,
            });
        }
        if state_id == 6143 {
            return Some(DarkOakHangingSign {
                r#rotation: 10,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6096 {
            return Some(DarkOakHangingSign {
                r#rotation: 3,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6097 {
            return Some(DarkOakHangingSign {
                r#rotation: 3,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6146 {
            return Some(DarkOakHangingSign {
                r#rotation: 12,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6115 {
            return Some(DarkOakHangingSign {
                r#rotation: 12,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6142 {
            return Some(DarkOakHangingSign {
                r#rotation: 10,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        return None;
    }
}

