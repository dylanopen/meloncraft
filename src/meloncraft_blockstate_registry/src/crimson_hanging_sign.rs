use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonHangingSign {
    pub attached: bool,
    pub waterlogged: bool,
    pub rotation: i32,
}

impl BlockState for CrimsonHangingSign {
    fn to_id(&self) -> i32 {
        if self.r#rotation == 12 && self.r#waterlogged == true && self.r#attached == true {
            return 6242;
        }
        if self.r#waterlogged == false && self.r#rotation == 4 && self.r#attached == true {
            return 6227;
        }
        if self.r#rotation == 1 && self.r#attached == true && self.r#waterlogged == false {
            return 6221;
        }
        if self.r#waterlogged == false && self.r#rotation == 7 && self.r#attached == true {
            return 6233;
        }
        if self.r#attached == true && self.r#rotation == 9 && self.r#waterlogged == true {
            return 6236;
        }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 13 {
            return 6245;
        }
        if self.r#attached == false && self.r#rotation == 11 && self.r#waterlogged == true {
            return 6272;
        }
        if self.r#attached == false && self.r#rotation == 9 && self.r#waterlogged == true {
            return 6268;
        }
        if self.r#waterlogged == true && self.r#rotation == 10 && self.r#attached == true {
            return 6238;
        }
        if self.r#rotation == 5 && self.r#waterlogged == false && self.r#attached == false {
            return 6261;
        }
        if self.r#rotation == 12 && self.r#attached == false && self.r#waterlogged == true {
            return 6274;
        }
        if self.r#waterlogged == false && self.r#rotation == 10 && self.r#attached == false {
            return 6271;
        }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 15 {
            return 6280;
        }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 5 {
            return 6229;
        }
        if self.r#attached == true && self.r#waterlogged == true && self.r#rotation == 13 {
            return 6244;
        }
        if self.r#waterlogged == false && self.r#rotation == 3 && self.r#attached == true {
            return 6225;
        }
        if self.r#rotation == 3 && self.r#waterlogged == true && self.r#attached == true {
            return 6224;
        }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 8 {
            return 6267;
        }
        if self.r#attached == false && self.r#rotation == 4 && self.r#waterlogged == true {
            return 6258;
        }
        if self.r#attached == false && self.r#rotation == 11 && self.r#waterlogged == false {
            return 6273;
        }
        if self.r#rotation == 11 && self.r#waterlogged == false && self.r#attached == true {
            return 6241;
        }
        if self.r#attached == false && self.r#rotation == 7 && self.r#waterlogged == false {
            return 6265;
        }
        if self.r#waterlogged == false && self.r#rotation == 4 && self.r#attached == false {
            return 6259;
        }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 2 {
            return 6223;
        }
        if self.r#waterlogged == true && self.r#rotation == 0 && self.r#attached == true {
            return 6218;
        }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 12 {
            return 6243;
        }
        if self.r#attached == false && self.r#rotation == 0 && self.r#waterlogged == true {
            return 6250;
        }
        if self.r#attached == false && self.r#waterlogged == false && self.r#rotation == 13 {
            return 6277;
        }
        if self.r#waterlogged == true && self.r#rotation == 8 && self.r#attached == false {
            return 6266;
        }
        if self.r#rotation == 5 && self.r#attached == false && self.r#waterlogged == true {
            return 6260;
        }
        if self.r#rotation == 6 && self.r#attached == false && self.r#waterlogged == false {
            return 6263;
        }
        if self.r#attached == false && self.r#rotation == 10 && self.r#waterlogged == true {
            return 6270;
        }
        if self.r#attached == false && self.r#waterlogged == true && self.r#rotation == 7 {
            return 6264;
        }
        if self.r#waterlogged == true && self.r#rotation == 2 && self.r#attached == true {
            return 6222;
        }
        if self.r#attached == false && self.r#waterlogged == false && self.r#rotation == 0 {
            return 6251;
        }
        if self.r#rotation == 6 && self.r#waterlogged == true && self.r#attached == true {
            return 6230;
        }
        if self.r#attached == true && self.r#waterlogged == false && self.r#rotation == 6 {
            return 6231;
        }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 14 {
            return 6247;
        }
        if self.r#waterlogged == true && self.r#rotation == 1 && self.r#attached == false {
            return 6252;
        }
        if self.r#attached == true && self.r#rotation == 11 && self.r#waterlogged == true {
            return 6240;
        }
        if self.r#rotation == 8 && self.r#waterlogged == true && self.r#attached == true {
            return 6234;
        }
        if self.r#rotation == 7 && self.r#waterlogged == true && self.r#attached == true {
            return 6232;
        }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 14 {
            return 6279;
        }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 9 {
            return 6269;
        }
        if self.r#attached == false && self.r#rotation == 14 && self.r#waterlogged == true {
            return 6278;
        }
        if self.r#rotation == 15 && self.r#attached == false && self.r#waterlogged == false {
            return 6281;
        }
        if self.r#rotation == 4 && self.r#attached == true && self.r#waterlogged == true {
            return 6226;
        }
        if self.r#attached == true && self.r#rotation == 14 && self.r#waterlogged == true {
            return 6246;
        }
        if self.r#attached == true && self.r#rotation == 15 && self.r#waterlogged == true {
            return 6248;
        }
        if self.r#rotation == 9 && self.r#attached == true && self.r#waterlogged == false {
            return 6237;
        }
        if self.r#rotation == 5 && self.r#attached == true && self.r#waterlogged == true {
            return 6228;
        }
        if self.r#rotation == 0 && self.r#waterlogged == false && self.r#attached == true {
            return 6219;
        }
        if self.r#rotation == 13 && self.r#waterlogged == true && self.r#attached == false {
            return 6276;
        }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 2 {
            return 6254;
        }
        if self.r#waterlogged == false && self.r#rotation == 1 && self.r#attached == false {
            return 6253;
        }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 3 {
            return 6256;
        }
        if self.r#rotation == 3 && self.r#attached == false && self.r#waterlogged == false {
            return 6257;
        }
        if self.r#attached == true && self.r#rotation == 10 && self.r#waterlogged == false {
            return 6239;
        }
        if self.r#rotation == 6 && self.r#attached == false && self.r#waterlogged == true {
            return 6262;
        }
        if self.r#attached == true && self.r#rotation == 15 && self.r#waterlogged == false {
            return 6249;
        }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 2 {
            return 6255;
        }
        if self.r#attached == true && self.r#rotation == 8 && self.r#waterlogged == false {
            return 6235;
        }
        if self.r#rotation == 12 && self.r#attached == false && self.r#waterlogged == false {
            return 6275;
        }
        if self.r#waterlogged == true && self.r#attached == true && self.r#rotation == 1 {
            return 6220;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6242 {
            return Some(CrimsonHangingSign {
                r#rotation: 12,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6227 {
            return Some(CrimsonHangingSign {
                r#waterlogged: false,
                r#rotation: 4,
                r#attached: true,
            });
        }
        if state_id == 6221 {
            return Some(CrimsonHangingSign {
                r#rotation: 1,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6233 {
            return Some(CrimsonHangingSign {
                r#waterlogged: false,
                r#rotation: 7,
                r#attached: true,
            });
        }
        if state_id == 6236 {
            return Some(CrimsonHangingSign {
                r#attached: true,
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 6245 {
            return Some(CrimsonHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 13,
            });
        }
        if state_id == 6272 {
            return Some(CrimsonHangingSign {
                r#attached: false,
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 6268 {
            return Some(CrimsonHangingSign {
                r#attached: false,
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 6238 {
            return Some(CrimsonHangingSign {
                r#waterlogged: true,
                r#rotation: 10,
                r#attached: true,
            });
        }
        if state_id == 6261 {
            return Some(CrimsonHangingSign {
                r#rotation: 5,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6274 {
            return Some(CrimsonHangingSign {
                r#rotation: 12,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6271 {
            return Some(CrimsonHangingSign {
                r#waterlogged: false,
                r#rotation: 10,
                r#attached: false,
            });
        }
        if state_id == 6280 {
            return Some(CrimsonHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 15,
            });
        }
        if state_id == 6229 {
            return Some(CrimsonHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 5,
            });
        }
        if state_id == 6244 {
            return Some(CrimsonHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 13,
            });
        }
        if state_id == 6225 {
            return Some(CrimsonHangingSign {
                r#waterlogged: false,
                r#rotation: 3,
                r#attached: true,
            });
        }
        if state_id == 6224 {
            return Some(CrimsonHangingSign {
                r#rotation: 3,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6267 {
            return Some(CrimsonHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 8,
            });
        }
        if state_id == 6258 {
            return Some(CrimsonHangingSign {
                r#attached: false,
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 6273 {
            return Some(CrimsonHangingSign {
                r#attached: false,
                r#rotation: 11,
                r#waterlogged: false,
            });
        }
        if state_id == 6241 {
            return Some(CrimsonHangingSign {
                r#rotation: 11,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6265 {
            return Some(CrimsonHangingSign {
                r#attached: false,
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 6259 {
            return Some(CrimsonHangingSign {
                r#waterlogged: false,
                r#rotation: 4,
                r#attached: false,
            });
        }
        if state_id == 6223 {
            return Some(CrimsonHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 2,
            });
        }
        if state_id == 6218 {
            return Some(CrimsonHangingSign {
                r#waterlogged: true,
                r#rotation: 0,
                r#attached: true,
            });
        }
        if state_id == 6243 {
            return Some(CrimsonHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 12,
            });
        }
        if state_id == 6250 {
            return Some(CrimsonHangingSign {
                r#attached: false,
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 6277 {
            return Some(CrimsonHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 6266 {
            return Some(CrimsonHangingSign {
                r#waterlogged: true,
                r#rotation: 8,
                r#attached: false,
            });
        }
        if state_id == 6260 {
            return Some(CrimsonHangingSign {
                r#rotation: 5,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6263 {
            return Some(CrimsonHangingSign {
                r#rotation: 6,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6270 {
            return Some(CrimsonHangingSign {
                r#attached: false,
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 6264 {
            return Some(CrimsonHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 7,
            });
        }
        if state_id == 6222 {
            return Some(CrimsonHangingSign {
                r#waterlogged: true,
                r#rotation: 2,
                r#attached: true,
            });
        }
        if state_id == 6251 {
            return Some(CrimsonHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 0,
            });
        }
        if state_id == 6230 {
            return Some(CrimsonHangingSign {
                r#rotation: 6,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6231 {
            return Some(CrimsonHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        if state_id == 6247 {
            return Some(CrimsonHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 14,
            });
        }
        if state_id == 6252 {
            return Some(CrimsonHangingSign {
                r#waterlogged: true,
                r#rotation: 1,
                r#attached: false,
            });
        }
        if state_id == 6240 {
            return Some(CrimsonHangingSign {
                r#attached: true,
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 6234 {
            return Some(CrimsonHangingSign {
                r#rotation: 8,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6232 {
            return Some(CrimsonHangingSign {
                r#rotation: 7,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6279 {
            return Some(CrimsonHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 14,
            });
        }
        if state_id == 6269 {
            return Some(CrimsonHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 9,
            });
        }
        if state_id == 6278 {
            return Some(CrimsonHangingSign {
                r#attached: false,
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 6281 {
            return Some(CrimsonHangingSign {
                r#rotation: 15,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6226 {
            return Some(CrimsonHangingSign {
                r#rotation: 4,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6246 {
            return Some(CrimsonHangingSign {
                r#attached: true,
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 6248 {
            return Some(CrimsonHangingSign {
                r#attached: true,
                r#rotation: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 6237 {
            return Some(CrimsonHangingSign {
                r#rotation: 9,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6228 {
            return Some(CrimsonHangingSign {
                r#rotation: 5,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6219 {
            return Some(CrimsonHangingSign {
                r#rotation: 0,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6276 {
            return Some(CrimsonHangingSign {
                r#rotation: 13,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6254 {
            return Some(CrimsonHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 2,
            });
        }
        if state_id == 6253 {
            return Some(CrimsonHangingSign {
                r#waterlogged: false,
                r#rotation: 1,
                r#attached: false,
            });
        }
        if state_id == 6256 {
            return Some(CrimsonHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 3,
            });
        }
        if state_id == 6257 {
            return Some(CrimsonHangingSign {
                r#rotation: 3,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6239 {
            return Some(CrimsonHangingSign {
                r#attached: true,
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 6262 {
            return Some(CrimsonHangingSign {
                r#rotation: 6,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6249 {
            return Some(CrimsonHangingSign {
                r#attached: true,
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 6255 {
            return Some(CrimsonHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 2,
            });
        }
        if state_id == 6235 {
            return Some(CrimsonHangingSign {
                r#attached: true,
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 6275 {
            return Some(CrimsonHangingSign {
                r#rotation: 12,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6220 {
            return Some(CrimsonHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 1,
            });
        }
        return None;
    }
}
