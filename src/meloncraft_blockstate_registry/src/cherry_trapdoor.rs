use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryTrapdoor {
    pub r#facing: Facing,
    pub r#half: Half,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
}

impl BlockState for CherryTrapdoor {
    fn to_id(&self) -> i32 {
        if self.r#open == false
            && self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 7264;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#open == true
            && self.r#half == Half::Top
        {
            return 7266;
        }
        if self.r#powered == false
            && self.r#open == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 7291;
        }
        if self.r#half == Half::Top
            && self.r#open == true
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 7268;
        }
        if self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#open == true
            && self.r#facing == Facing::East
        {
            return 7289;
        }
        if self.r#powered == true
            && self.r#open == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
        {
            return 7234;
        }
        if self.r#open == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 7235;
        }
        if self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#open == false
            && self.r#waterlogged == false
        {
            return 7280;
        }
        if self.r#waterlogged == false
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#facing == Facing::East
        {
            return 7292;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
        {
            return 7236;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#open == false
            && self.r#half == Half::Top
        {
            return 7271;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#open == true
        {
            return 7257;
        }
        if self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#open == true
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 7282;
        }
        if self.r#waterlogged == false
            && self.r#open == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#powered == false
        {
            return 7252;
        }
        if self.r#open == false
            && self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 7245;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#open == false
            && self.r#half == Half::Top
        {
            return 7238;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#open == true
            && self.r#powered == false
        {
            return 7275;
        }
        if self.r#open == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#powered == false
        {
            return 7272;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#open == false
            && self.r#powered == true
        {
            return 7278;
        }
        if self.r#open == false
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 7270;
        }
        if self.r#powered == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#open == false
            && self.r#waterlogged == true
        {
            return 7261;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#open == true
            && self.r#waterlogged == false
        {
            return 7258;
        }
        if self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 7246;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#powered == false
            && self.r#open == false
            && self.r#waterlogged == false
        {
            return 7240;
        }
        if self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 7294;
        }
        if self.r#powered == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#open == true
            && self.r#waterlogged == true
        {
            return 7281;
        }
        if self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#open == true
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 7276;
        }
        if self.r#waterlogged == false
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#open == false
        {
            return 7296;
        }
        if self.r#open == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 7247;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#open == false
            && self.r#powered == true
        {
            return 7277;
        }
        if self.r#powered == true
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#open == false
        {
            return 7293;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 7244;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#open == true
        {
            return 7250;
        }
        if self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#open == false
            && self.r#waterlogged == false
        {
            return 7254;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 7260;
        }
        if self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#facing == Facing::South
        {
            return 7263;
        }
        if self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#open == true
            && self.r#half == Half::Bottom
        {
            return 7259;
        }
        if self.r#powered == true
            && self.r#open == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
        {
            return 7290;
        }
        if self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#open == true
            && self.r#waterlogged == true
        {
            return 7273;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#open == false
        {
            return 7295;
        }
        if self.r#half == Half::Top
            && self.r#open == true
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#facing == Facing::East
        {
            return 7284;
        }
        if self.r#facing == Facing::South
            && self.r#open == false
            && self.r#powered == false
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 7256;
        }
        if self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 7262;
        }
        if self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 7249;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#open == false
            && self.r#facing == Facing::South
            && self.r#powered == true
        {
            return 7253;
        }
        if self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#open == false
        {
            return 7248;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#half == Half::Top
            && self.r#open == false
        {
            return 7239;
        }
        if self.r#facing == Facing::South
            && self.r#open == false
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#powered == false
        {
            return 7255;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#open == true
            && self.r#half == Half::Bottom
        {
            return 7274;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 7243;
        }
        if self.r#waterlogged == true
            && self.r#powered == true
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#facing == Facing::North
        {
            return 7241;
        }
        if self.r#waterlogged == true
            && self.r#open == false
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
        {
            return 7269;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#open == true
            && self.r#waterlogged == false
        {
            return 7242;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#powered == true
        {
            return 7265;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#facing == Facing::North
            && self.r#powered == true
        {
            return 7233;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#open == false
        {
            return 7286;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#open == false
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 7287;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
        {
            return 7251;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#open == false
        {
            return 7279;
        }
        if self.r#half == Half::Top
            && self.r#open == true
            && self.r#powered == false
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 7283;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#open == false
            && self.r#powered == true
        {
            return 7237;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#powered == false
        {
            return 7267;
        }
        if self.r#half == Half::Top
            && self.r#open == false
            && self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 7285;
        }
        if self.r#powered == false
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#open == false
            && self.r#facing == Facing::East
        {
            return 7288;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7264 {
            return Some(CherryTrapdoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 7266 {
            return Some(CherryTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: true,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7291 {
            return Some(CherryTrapdoor {
                r#powered: false,
                r#open: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 7268 {
            return Some(CherryTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 7289 {
            return Some(CherryTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7234 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 7235 {
            return Some(CherryTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7280 {
            return Some(CherryTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7292 {
            return Some(CherryTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7236 {
            return Some(CherryTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 7271 {
            return Some(CherryTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: false,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7257 {
            return Some(CherryTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 7282 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7252 {
            return Some(CherryTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 7245 {
            return Some(CherryTrapdoor {
                r#open: false,
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 7238 {
            return Some(CherryTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: true,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7275 {
            return Some(CherryTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 7272 {
            return Some(CherryTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 7278 {
            return Some(CherryTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 7270 {
            return Some(CherryTrapdoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 7261 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7258 {
            return Some(CherryTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7246 {
            return Some(CherryTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 7240 {
            return Some(CherryTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7294 {
            return Some(CherryTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 7281 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7276 {
            return Some(CherryTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 7296 {
            return Some(CherryTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 7247 {
            return Some(CherryTrapdoor {
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7277 {
            return Some(CherryTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 7293 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 7244 {
            return Some(CherryTrapdoor {
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 7250 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 7254 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7260 {
            return Some(CherryTrapdoor {
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7263 {
            return Some(CherryTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7259 {
            return Some(CherryTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
                r#open: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7290 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7273 {
            return Some(CherryTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::West,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7295 {
            return Some(CherryTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 7284 {
            return Some(CherryTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 7256 {
            return Some(CherryTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 7262 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 7249 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 7253 {
            return Some(CherryTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 7248 {
            return Some(CherryTrapdoor {
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 7239 {
            return Some(CherryTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: false,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 7255 {
            return Some(CherryTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 7274 {
            return Some(CherryTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7243 {
            return Some(CherryTrapdoor {
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 7241 {
            return Some(CherryTrapdoor {
                r#waterlogged: true,
                r#powered: true,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 7269 {
            return Some(CherryTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 7242 {
            return Some(CherryTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7265 {
            return Some(CherryTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 7233 {
            return Some(CherryTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 7286 {
            return Some(CherryTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 7287 {
            return Some(CherryTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7251 {
            return Some(CherryTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 7279 {
            return Some(CherryTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 7283 {
            return Some(CherryTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 7237 {
            return Some(CherryTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 7267 {
            return Some(CherryTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 7285 {
            return Some(CherryTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 7288 {
            return Some(CherryTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
