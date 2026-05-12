use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakHangingSign {
    pub attached: bool,
    pub rotation: i32,
    pub waterlogged: bool,
}


impl BlockState for PaleOakHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 7 { return 6168; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6159; }
        if block_state.r#rotation == 5 && block_state.r#attached == false && block_state.r#waterlogged == true { return 6196; }
        if block_state.r#attached == true && block_state.r#rotation == 11 && block_state.r#waterlogged == false { return 6177; }
        if block_state.r#rotation == 3 && block_state.r#attached == true && block_state.r#waterlogged == false { return 6161; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 6 { return 6167; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6163; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 10 { return 6174; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == true && block_state.r#attached == true { return 6162; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6179; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 8 { return 6171; }
        if block_state.r#attached == true && block_state.r#rotation == 13 && block_state.r#waterlogged == true { return 6180; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 11 && block_state.r#attached == false { return 6209; }
        if block_state.r#attached == false && block_state.r#rotation == 13 && block_state.r#waterlogged == true { return 6212; }
        if block_state.r#rotation == 6 && block_state.r#attached == true && block_state.r#waterlogged == true { return 6166; }
        if block_state.r#rotation == 1 && block_state.r#attached == true && block_state.r#waterlogged == true { return 6156; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 14 && block_state.r#attached == false { return 6215; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == true && block_state.r#attached == true { return 6182; }
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 15 { return 6217; }
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 10 { return 6207; }
        if block_state.r#rotation == 9 && block_state.r#attached == true && block_state.r#waterlogged == false { return 6173; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 15 { return 6216; }
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 6 { return 6199; }
        if block_state.r#rotation == 3 && block_state.r#attached == false && block_state.r#waterlogged == true { return 6192; }
        if block_state.r#attached == true && block_state.r#rotation == 7 && block_state.r#waterlogged == false { return 6169; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6165; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 11 && block_state.r#attached == true { return 6176; }
        if block_state.r#attached == false && block_state.r#rotation == 4 && block_state.r#waterlogged == true { return 6194; }
        if block_state.r#attached == false && block_state.r#rotation == 5 && block_state.r#waterlogged == false { return 6197; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 6 { return 6198; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 0 && block_state.r#attached == false { return 6186; }
        if block_state.r#attached == false && block_state.r#rotation == 7 && block_state.r#waterlogged == false { return 6201; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 8 && block_state.r#attached == false { return 6203; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 0 { return 6154; }
        if block_state.r#rotation == 2 && block_state.r#attached == true && block_state.r#waterlogged == true { return 6158; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 10 && block_state.r#attached == true { return 6175; }
        if block_state.r#attached == true && block_state.r#rotation == 9 && block_state.r#waterlogged == true { return 6172; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 15 { return 6185; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 2 && block_state.r#attached == false { return 6191; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6193; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 10 { return 6206; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 5 { return 6164; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == true && block_state.r#attached == true { return 6178; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 3 { return 6160; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 2 { return 6190; }
        if block_state.r#rotation == 14 && block_state.r#attached == false && block_state.r#waterlogged == true { return 6214; }
        if block_state.r#attached == false && block_state.r#rotation == 7 && block_state.r#waterlogged == true { return 6200; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6205; }
        if block_state.r#rotation == 8 && block_state.r#attached == true && block_state.r#waterlogged == true { return 6170; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 0 { return 6155; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 13 && block_state.r#attached == true { return 6181; }
        if block_state.r#attached == false && block_state.r#rotation == 9 && block_state.r#waterlogged == true { return 6204; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == true && block_state.r#attached == false { return 6188; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 11 { return 6208; }
        if block_state.r#attached == true && block_state.r#rotation == 1 && block_state.r#waterlogged == false { return 6157; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 12 { return 6211; }
        if block_state.r#attached == true && block_state.r#rotation == 15 && block_state.r#waterlogged == true { return 6184; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 0 { return 6187; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 4 { return 6195; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 8 { return 6202; }
        if block_state.r#attached == false && block_state.r#rotation == 12 && block_state.r#waterlogged == true { return 6210; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 13 { return 6213; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6189; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 14 && block_state.r#attached == true { return 6183; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6168 {
            return Some(PaleOakHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 7,
            });
        }
        if state_id == 6159 {
            return Some(PaleOakHangingSign {
                r#rotation: 2,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6196 {
            return Some(PaleOakHangingSign {
                r#rotation: 5,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6177 {
            return Some(PaleOakHangingSign {
                r#attached: true,
                r#rotation: 11,
                r#waterlogged: false,
            });
        }
        if state_id == 6161 {
            return Some(PaleOakHangingSign {
                r#rotation: 3,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6167 {
            return Some(PaleOakHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 6,
            });
        }
        if state_id == 6163 {
            return Some(PaleOakHangingSign {
                r#rotation: 4,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6174 {
            return Some(PaleOakHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 10,
            });
        }
        if state_id == 6162 {
            return Some(PaleOakHangingSign {
                r#rotation: 4,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6179 {
            return Some(PaleOakHangingSign {
                r#rotation: 12,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6171 {
            return Some(PaleOakHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 8,
            });
        }
        if state_id == 6180 {
            return Some(PaleOakHangingSign {
                r#attached: true,
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 6209 {
            return Some(PaleOakHangingSign {
                r#waterlogged: false,
                r#rotation: 11,
                r#attached: false,
            });
        }
        if state_id == 6212 {
            return Some(PaleOakHangingSign {
                r#attached: false,
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 6166 {
            return Some(PaleOakHangingSign {
                r#rotation: 6,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6156 {
            return Some(PaleOakHangingSign {
                r#rotation: 1,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6215 {
            return Some(PaleOakHangingSign {
                r#waterlogged: false,
                r#rotation: 14,
                r#attached: false,
            });
        }
        if state_id == 6182 {
            return Some(PaleOakHangingSign {
                r#rotation: 14,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6217 {
            return Some(PaleOakHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 15,
            });
        }
        if state_id == 6207 {
            return Some(PaleOakHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 10,
            });
        }
        if state_id == 6173 {
            return Some(PaleOakHangingSign {
                r#rotation: 9,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6216 {
            return Some(PaleOakHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 15,
            });
        }
        if state_id == 6199 {
            return Some(PaleOakHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        if state_id == 6192 {
            return Some(PaleOakHangingSign {
                r#rotation: 3,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6169 {
            return Some(PaleOakHangingSign {
                r#attached: true,
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 6165 {
            return Some(PaleOakHangingSign {
                r#rotation: 5,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6176 {
            return Some(PaleOakHangingSign {
                r#waterlogged: true,
                r#rotation: 11,
                r#attached: true,
            });
        }
        if state_id == 6194 {
            return Some(PaleOakHangingSign {
                r#attached: false,
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 6197 {
            return Some(PaleOakHangingSign {
                r#attached: false,
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 6198 {
            return Some(PaleOakHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 6,
            });
        }
        if state_id == 6186 {
            return Some(PaleOakHangingSign {
                r#waterlogged: true,
                r#rotation: 0,
                r#attached: false,
            });
        }
        if state_id == 6201 {
            return Some(PaleOakHangingSign {
                r#attached: false,
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 6203 {
            return Some(PaleOakHangingSign {
                r#waterlogged: false,
                r#rotation: 8,
                r#attached: false,
            });
        }
        if state_id == 6154 {
            return Some(PaleOakHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 0,
            });
        }
        if state_id == 6158 {
            return Some(PaleOakHangingSign {
                r#rotation: 2,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6175 {
            return Some(PaleOakHangingSign {
                r#waterlogged: false,
                r#rotation: 10,
                r#attached: true,
            });
        }
        if state_id == 6172 {
            return Some(PaleOakHangingSign {
                r#attached: true,
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 6185 {
            return Some(PaleOakHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 15,
            });
        }
        if state_id == 6191 {
            return Some(PaleOakHangingSign {
                r#waterlogged: false,
                r#rotation: 2,
                r#attached: false,
            });
        }
        if state_id == 6193 {
            return Some(PaleOakHangingSign {
                r#rotation: 3,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6206 {
            return Some(PaleOakHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 10,
            });
        }
        if state_id == 6164 {
            return Some(PaleOakHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 5,
            });
        }
        if state_id == 6178 {
            return Some(PaleOakHangingSign {
                r#rotation: 12,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6160 {
            return Some(PaleOakHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 3,
            });
        }
        if state_id == 6190 {
            return Some(PaleOakHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 2,
            });
        }
        if state_id == 6214 {
            return Some(PaleOakHangingSign {
                r#rotation: 14,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6200 {
            return Some(PaleOakHangingSign {
                r#attached: false,
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 6205 {
            return Some(PaleOakHangingSign {
                r#rotation: 9,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6170 {
            return Some(PaleOakHangingSign {
                r#rotation: 8,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6155 {
            return Some(PaleOakHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 0,
            });
        }
        if state_id == 6181 {
            return Some(PaleOakHangingSign {
                r#waterlogged: false,
                r#rotation: 13,
                r#attached: true,
            });
        }
        if state_id == 6204 {
            return Some(PaleOakHangingSign {
                r#attached: false,
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 6188 {
            return Some(PaleOakHangingSign {
                r#rotation: 1,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6208 {
            return Some(PaleOakHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 11,
            });
        }
        if state_id == 6157 {
            return Some(PaleOakHangingSign {
                r#attached: true,
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 6211 {
            return Some(PaleOakHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 12,
            });
        }
        if state_id == 6184 {
            return Some(PaleOakHangingSign {
                r#attached: true,
                r#rotation: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 6187 {
            return Some(PaleOakHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 0,
            });
        }
        if state_id == 6195 {
            return Some(PaleOakHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 4,
            });
        }
        if state_id == 6202 {
            return Some(PaleOakHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 8,
            });
        }
        if state_id == 6210 {
            return Some(PaleOakHangingSign {
                r#attached: false,
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 6213 {
            return Some(PaleOakHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 13,
            });
        }
        if state_id == 6189 {
            return Some(PaleOakHangingSign {
                r#rotation: 1,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6183 {
            return Some(PaleOakHangingSign {
                r#waterlogged: false,
                r#rotation: 14,
                r#attached: true,
            });
        }
        return None;
    }
}

