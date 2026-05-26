use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaTrapdoor {
    pub powered: bool,
    pub r#facing: Facing,
    pub waterlogged: bool,
    pub r#half: Half,
    pub open: bool,
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

impl BlockState for AcaciaTrapdoor {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#open == false
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 7214;
        }
        if self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#open == false
            && self.r#facing == Facing::North
        {
            return 7182;
        }
        if self.r#open == true
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#powered == true
        {
            return 7226;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#open == false
            && self.r#waterlogged == false
        {
            return 7192;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#powered == false
        {
            return 7232;
        }
        if self.r#waterlogged == false
            && self.r#open == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#powered == false
        {
            return 7184;
        }
        if self.r#waterlogged == true
            && self.r#open == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#powered == true
        {
            return 7193;
        }
        if self.r#powered == true
            && self.r#open == true
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 7225;
        }
        if self.r#powered == true
            && self.r#waterlogged == true
            && self.r#open == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
        {
            return 7221;
        }
        if self.r#open == false
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#powered == false
        {
            return 7175;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#open == false
            && self.r#half == Half::Top
        {
            return 7207;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#powered == true
        {
            return 7217;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 7205;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#open == true
        {
            return 7209;
        }
        if self.r#waterlogged == false
            && self.r#powered == false
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#facing == Facing::North
        {
            return 7172;
        }
        if self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#open == false
        {
            return 7181;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#open == false
            && self.r#facing == Facing::South
            && self.r#powered == true
        {
            return 7189;
        }
        if self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#open == true
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 7218;
        }
        if self.r#facing == Facing::East
            && self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 7229;
        }
        if self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#waterlogged == true
        {
            return 7231;
        }
        if self.r#half == Half::Top
            && self.r#open == true
            && self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 7203;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#open == false
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 7216;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#open == true
            && self.r#facing == Facing::East
            && self.r#powered == false
        {
            return 7219;
        }
        if self.r#waterlogged == false
            && self.r#open == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#powered == true
        {
            return 7198;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#open == false
        {
            return 7223;
        }
        if self.r#open == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#powered == true
        {
            return 7173;
        }
        if self.r#open == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 7206;
        }
        if self.r#powered == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#open == true
        {
            return 7188;
        }
        if self.r#open == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#half == Half::Bottom
        {
            return 7183;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#open == false
        {
            return 7222;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#open == false
            && self.r#facing == Facing::East
            && self.r#powered == false
        {
            return 7224;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 7197;
        }
        if self.r#open == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#powered == true
        {
            return 7190;
        }
        if self.r#open == true
            && self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
        {
            return 7211;
        }
        if self.r#half == Half::Top
            && self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 7174;
        }
        if self.r#half == Half::Top
            && self.r#open == true
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#powered == true
        {
            return 7185;
        }
        if self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#waterlogged == true
        {
            return 7195;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#open == true
            && self.r#powered == true
            && self.r#half == Half::Bottom
        {
            return 7194;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#open == true
        {
            return 7170;
        }
        if self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#waterlogged == false
        {
            return 7196;
        }
        if self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#open == false
        {
            return 7213;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
        {
            return 7187;
        }
        if self.r#open == false
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 7200;
        }
        if self.r#open == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#half == Half::Top
        {
            return 7202;
        }
        if self.r#facing == Facing::West
            && self.r#open == true
            && self.r#half == Half::Top
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 7204;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#open == true
        {
            return 7171;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#open == false
        {
            return 7199;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
        {
            return 7180;
        }
        if self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#open == true
            && self.r#half == Half::Top
        {
            return 7186;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#open == true
        {
            return 7179;
        }
        if self.r#powered == false
            && self.r#waterlogged == false
            && self.r#open == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
        {
            return 7176;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#open == true
        {
            return 7210;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#open == true
        {
            return 7177;
        }
        if self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 7230;
        }
        if self.r#waterlogged == false
            && self.r#open == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#powered == false
        {
            return 7208;
        }
        if self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#open == true
        {
            return 7169;
        }
        if self.r#facing == Facing::North
            && self.r#open == true
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 7178;
        }
        if self.r#open == true
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#facing == Facing::West
        {
            return 7212;
        }
        if self.r#open == false
            && self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 7215;
        }
        if self.r#open == true
            && self.r#half == Half::Top
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
        {
            return 7201;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#open == true
            && self.r#powered == false
        {
            return 7228;
        }
        if self.r#powered == false
            && self.r#open == false
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
        {
            return 7191;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#open == true
            && self.r#powered == false
        {
            return 7227;
        }
        if self.r#half == Half::Top
            && self.r#open == true
            && self.r#powered == false
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 7220;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7214 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#open: false,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 7182 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 7226 {
            return Some(AcaciaTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 7192 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: false,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7232 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 7184 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 7193 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 7225 {
            return Some(AcaciaTrapdoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7221 {
            return Some(AcaciaTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 7175 {
            return Some(AcaciaTrapdoor {
                r#open: false,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 7207 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: false,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7217 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 7205 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7209 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 7172 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 7181 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 7189 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 7218 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7229 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7231 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7203 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7216 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: false,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 7219 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 7198 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 7223 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 7173 {
            return Some(AcaciaTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 7206 {
            return Some(AcaciaTrapdoor {
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7188 {
            return Some(AcaciaTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 7183 {
            return Some(AcaciaTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7222 {
            return Some(AcaciaTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 7224 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 7197 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7190 {
            return Some(AcaciaTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 7211 {
            return Some(AcaciaTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 7174 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 7185 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 7195 {
            return Some(AcaciaTrapdoor {
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7194 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#open: true,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7170 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 7196 {
            return Some(AcaciaTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7213 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 7187 {
            return Some(AcaciaTrapdoor {
                r#open: true,
                r#powered: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 7200 {
            return Some(AcaciaTrapdoor {
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7202 {
            return Some(AcaciaTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7204 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7171 {
            return Some(AcaciaTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 7199 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 7180 {
            return Some(AcaciaTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7186 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7179 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 7176 {
            return Some(AcaciaTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 7210 {
            return Some(AcaciaTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 7177 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 7230 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 7208 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 7169 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 7178 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7212 {
            return Some(AcaciaTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 7215 {
            return Some(AcaciaTrapdoor {
                r#open: false,
                r#facing: Facing::West,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 7201 {
            return Some(AcaciaTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 7228 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 7191 {
            return Some(AcaciaTrapdoor {
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 7227 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 7220 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
