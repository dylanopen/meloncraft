use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuffBrickWall {
    pub r#north: North,
    pub waterlogged: bool,
    pub up: bool,
    pub r#west: West,
    pub r#east: East,
    pub r#south: South,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum North {
    None,
    Low,
    Tall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum West {
    None,
    Low,
    Tall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum East {
    None,
    Low,
    Tall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum South {
    None,
    Low,
    Tall,
}

impl BlockState for TuffBrickWall {
    fn to_id(&self) -> i32 {
        if self.r#west == West::None
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#south == South::None
        {
            return 24451;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::None
        {
            return 24252;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#waterlogged == false
        {
            return 24272;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#north == North::None
        {
            return 24194;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 24176;
        }
        if self.r#south == South::None
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 24306;
        }
        if self.r#south == South::Low
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 24424;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#west == West::None
        {
            return 24460;
        }
        if self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#up == true
        {
            return 24378;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 24216;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 24464;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#waterlogged == true
        {
            return 24342;
        }
        if self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#up == false
        {
            return 24241;
        }
        if self.r#up == true
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#east == East::Tall
        {
            return 24380;
        }
        if self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#south == South::Low
        {
            return 24320;
        }
        if self.r#up == false
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#south == South::Low
        {
            return 24363;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#north == North::None
        {
            return 24191;
        }
        if self.r#north == North::Low
            && self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 24439;
        }
        if self.r#up == false
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#east == East::None
        {
            return 24230;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#south == South::None
        {
            return 24273;
        }
        if self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Low
        {
            return 24297;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 24400;
        }
        if self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#east == East::Tall
        {
            return 24450;
        }
        if self.r#up == true
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#west == West::None
        {
            return 24415;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#west == West::Tall
        {
            return 24174;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#east == East::None
        {
            return 24235;
        }
        if self.r#up == true
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#waterlogged == false
        {
            return 24331;
        }
        if self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
        {
            return 24307;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::None
            && self.r#south == South::Tall
        {
            return 24259;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::None
        {
            return 24175;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#up == false
        {
            return 24298;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 24257;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Tall
        {
            return 24359;
        }
        if self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::Tall
        {
            return 24185;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#north == North::None
        {
            return 24398;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 24212;
        }
        if self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#south == South::None
        {
            return 24169;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#south == South::None
            && self.r#north == North::Low
        {
            return 24413;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Low
        {
            return 24296;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#north == North::Low
        {
            return 24314;
        }
        if self.r#south == South::Low
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 24391;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#east == East::Tall
        {
            return 24416;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#north == North::None
        {
            return 24303;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#waterlogged == true
        {
            return 24388;
        }
        if self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::None
        {
            return 24394;
        }
        if self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#west == West::Tall
        {
            return 24417;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == false
        {
            return 24477;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 24357;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#north == North::Low
        {
            return 24412;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 24458;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#west == West::None
        {
            return 24229;
        }
        if self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#east == East::None
            && self.r#west == West::Low
        {
            return 24242;
        }
        if self.r#up == false
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 24301;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#north == North::None
        {
            return 24285;
        }
        if self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Low
        {
            return 24218;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#south == South::Tall
        {
            return 24368;
        }
        if self.r#north == North::Low
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == true
        {
            return 24200;
        }
        if self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#north == North::Tall
        {
            return 24254;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#north == North::Tall
        {
            return 24374;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 24449;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#up == false
            && self.r#east == East::None
            && self.r#north == North::Low
        {
            return 24206;
        }
        if self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#up == false
        {
            return 24481;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 24326;
        }
        if self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Low
        {
            return 24421;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#up == true
        {
            return 24292;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#west == West::Tall
        {
            return 24336;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == false
        {
            return 24470;
        }
        if self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 24195;
        }
        if self.r#south == South::None
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 24207;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#up == true
        {
            return 24340;
        }
        if self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 24471;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#north == North::None
        {
            return 24294;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#up == false
            && self.r#south == South::None
        {
            return 24205;
        }
        if self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == true
        {
            return 24347;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 24457;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Low
        {
            return 24219;
        }
        if self.r#up == true
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Low
        {
            return 24304;
        }
        if self.r#west == West::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#east == East::None
        {
            return 24248;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#west == West::None
        {
            return 24427;
        }
        if self.r#up == true
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#south == South::Tall
        {
            return 24256;
        }
        if self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Low
        {
            return 24353;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == false
        {
            return 24190;
        }
        if self.r#north == North::None
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == false
        {
            return 24271;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#west == West::None
        {
            return 24430;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#west == West::Low
        {
            return 24428;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#south == South::None
        {
            return 24168;
        }
        if self.r#north == North::Low
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 24224;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#east == East::Tall
        {
            return 24483;
        }
        if self.r#up == true
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#west == West::Tall
        {
            return 24234;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#south == South::Tall
        {
            return 24261;
        }
        if self.r#west == West::None
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#north == North::None
        {
            return 24397;
        }
        if self.r#waterlogged == true
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#north == North::Low
        {
            return 24334;
        }
        if self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 24258;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#west == West::Tall
        {
            return 24315;
        }
        if self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#waterlogged == false
        {
            return 24387;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#south == South::None
            && self.r#west == West::Tall
        {
            return 24312;
        }
        if self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 24180;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#north == North::None
        {
            return 24268;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#east == East::Low
        {
            return 24332;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 24352;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 24193;
        }
        if self.r#up == false
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#south == South::Low
        {
            return 24324;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::None
        {
            return 24377;
        }
        if self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#south == South::None
        {
            return 24311;
        }
        if self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#east == East::Tall
        {
            return 24448;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 24386;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 24399;
        }
        if self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 24382;
        }
        if self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#north == North::Tall
        {
            return 24455;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#up == false
        {
            return 24459;
        }
        if self.r#up == true
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 24379;
        }
        if self.r#up == false
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Tall
        {
            return 24446;
        }
        if self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#south == South::None
            && self.r#north == North::Low
        {
            return 24201;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#waterlogged == true
        {
            return 24160;
        }
        if self.r#north == North::Low
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 24228;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#north == North::None
        {
            return 24291;
        }
        if self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#up == true
            && self.r#north == North::None
        {
            return 24187;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
        {
            return 24465;
        }
        if self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#west == West::Low
        {
            return 24365;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#west == West::None
        {
            return 24436;
        }
        if self.r#up == true
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#west == West::Low
        {
            return 24404;
        }
        if self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#up == false
        {
            return 24443;
        }
        if self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#waterlogged == true
        {
            return 24192;
        }
        if self.r#up == false
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 24227;
        }
        if self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#up == true
        {
            return 24476;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::None
        {
            return 24288;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Low
        {
            return 24321;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == false
        {
            return 24348;
        }
        if self.r#up == true
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#waterlogged == true
        {
            return 24328;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#up == false
        {
            return 24251;
        }
        if self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#east == East::Low
        {
            return 24349;
        }
        if self.r#west == West::Low
            && self.r#south == South::None
            && self.r#up == false
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#waterlogged == true
        {
            return 24167;
        }
        if self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == false
        {
            return 24346;
        }
        if self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#up == true
        {
            return 24317;
        }
        if self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#waterlogged == false
        {
            return 24440;
        }
        if self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 24453;
        }
        if self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::Tall
        {
            return 24263;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Tall
        {
            return 24461;
        }
        if self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#up == true
        {
            return 24197;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#north == North::None
        {
            return 24173;
        }
        if self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 24264;
        }
        if self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 24166;
        }
        if self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#up == false
        {
            return 24373;
        }
        if self.r#south == South::Tall
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 24438;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 24199;
        }
        if self.r#up == false
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#west == West::Low
        {
            return 24422;
        }
        if self.r#west == West::Low
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#east == East::Low
        {
            return 24284;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#east == East::None
        {
            return 24179;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 24209;
        }
        if self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 24240;
        }
        if self.r#up == false
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#west == West::Tall
        {
            return 24255;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Tall
        {
            return 24482;
        }
        if self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#up == true
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#waterlogged == true
        {
            return 24198;
        }
        if self.r#west == West::None
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 24418;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#up == true
        {
            return 24330;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#up == true
        {
            return 24188;
        }
        if self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 24361;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::None
        {
            return 24162;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 24366;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#west == West::None
        {
            return 24211;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == true
        {
            return 24177;
        }
        if self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#east == East::Low
        {
            return 24325;
        }
        if self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#up == false
            && self.r#east == East::None
            && self.r#waterlogged == false
        {
            return 24217;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#east == East::Tall
        {
            return 24408;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::Tall
        {
            return 24335;
        }
        if self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 24351;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#up == true
        {
            return 24319;
        }
        if self.r#south == South::None
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 24350;
        }
        if self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 24215;
        }
        if self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 24323;
        }
        if self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::Tall
        {
            return 24409;
        }
        if self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#south == South::Low
        {
            return 24322;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#west == West::Tall
        {
            return 24426;
        }
        if self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::None
        {
            return 24202;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::None
        {
            return 24163;
        }
        if self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#west == West::Tall
        {
            return 24213;
        }
        if self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == true
        {
            return 24208;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#west == West::Low
        {
            return 24266;
        }
        if self.r#up == true
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#west == West::Low
        {
            return 24260;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Low
        {
            return 24281;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 24437;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 24435;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::None
        {
            return 24406;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::Tall
        {
            return 24472;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#up == false
        {
            return 24275;
        }
        if self.r#up == true
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::Tall
        {
            return 24184;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Low
        {
            return 24327;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#east == East::Tall
        {
            return 24479;
        }
        if self.r#west == West::None
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 24277;
        }
        if self.r#up == true
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#west == West::None
        {
            return 24232;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::None
        {
            return 24237;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#up == false
        {
            return 24250;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::None
            && self.r#waterlogged == true
        {
            return 24310;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#west == West::None
        {
            return 24358;
        }
        if self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 24189;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::None
        {
            return 24308;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 24265;
        }
        if self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#waterlogged == false
        {
            return 24339;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#south == South::None
        {
            return 24345;
        }
        if self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#south == South::None
        {
            return 24164;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 24210;
        }
        if self.r#up == false
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 24226;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#east == East::Low
        {
            return 24313;
        }
        if self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 24390;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#south == South::Tall
        {
            return 24403;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#up == false
            && self.r#south == South::Tall
        {
            return 24407;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 24456;
        }
        if self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#north == North::None
        {
            return 24183;
        }
        if self.r#up == true
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 24233;
        }
        if self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#waterlogged == true
        {
            return 24384;
        }
        if self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 24381;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#north == North::Tall
        {
            return 24236;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Low
        {
            return 24280;
        }
        if self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Low
        {
            return 24246;
        }
        if self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#north == North::None
        {
            return 24276;
        }
        if self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#up == false
        {
            return 24480;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == false
        {
            return 24447;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#south == South::Low
        {
            return 24395;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 24423;
        }
        if self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::Low
        {
            return 24287;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#north == North::Tall
        {
            return 24355;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#waterlogged == true
        {
            return 24473;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 24474;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#east == East::Low
        {
            return 24278;
        }
        if self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#south == South::Tall
        {
            return 24401;
        }
        if self.r#west == West::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
        {
            return 24253;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#south == South::Low
        {
            return 24425;
        }
        if self.r#west == West::Tall
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == true
        {
            return 24444;
        }
        if self.r#up == false
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == true
        {
            return 24442;
        }
        if self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::Low
        {
            return 24290;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#west == West::None
            && self.r#north == North::Tall
        {
            return 24364;
        }
        if self.r#up == false
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#waterlogged == true
        {
            return 24299;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Tall
        {
            return 24369;
        }
        if self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 24182;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == true
        {
            return 24161;
        }
        if self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#waterlogged == true
        {
            return 24383;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#up == true
        {
            return 24305;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 24181;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == false
        {
            return 24178;
        }
        if self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#east == East::None
        {
            return 24262;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#east == East::None
        {
            return 24165;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#west == West::Low
        {
            return 24245;
        }
        if self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#east == East::Tall
        {
            return 24467;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#south == South::Tall
        {
            return 24410;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#east == East::Low
        {
            return 24329;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 24172;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == true
        {
            return 24452;
        }
        if self.r#north == North::Low
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == false
        {
            return 24231;
        }
        if self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#up == true
        {
            return 24392;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::None
        {
            return 24402;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 24466;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#east == East::Tall
        {
            return 24469;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Low
        {
            return 24337;
        }
        if self.r#west == West::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::None
        {
            return 24309;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::None
            && self.r#south == South::None
        {
            return 24196;
        }
        if self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == true
        {
            return 24203;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#up == false
        {
            return 24286;
        }
        if self.r#south == South::Tall
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#waterlogged == false
        {
            return 24445;
        }
        if self.r#up == false
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#west == West::None
        {
            return 24478;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#south == South::Low
        {
            return 24247;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#west == West::None
        {
            return 24223;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 24270;
        }
        if self.r#west == West::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 24283;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 24333;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#up == false
        {
            return 24454;
        }
        if self.r#west == West::None
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 24370;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#west == West::Tall
        {
            return 24222;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#up == true
        {
            return 24376;
        }
        if self.r#west == West::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 24385;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#south == South::Low
        {
            return 24468;
        }
        if self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == false
        {
            return 24338;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 24282;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
        {
            return 24393;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#west == West::None
        {
            return 24463;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::None
        {
            return 24419;
        }
        if self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#east == East::Low
        {
            return 24360;
        }
        if self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 24316;
        }
        if self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::Tall
        {
            return 24302;
        }
        if self.r#up == false
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 24243;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 24420;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#up == false
        {
            return 24267;
        }
        if self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#waterlogged == true
        {
            return 24186;
        }
        if self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 24341;
        }
        if self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#south == South::Low
        {
            return 24318;
        }
        if self.r#up == false
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Low
        {
            return 24431;
        }
        if self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#waterlogged == false
        {
            return 24356;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#east == East::Tall
        {
            return 24405;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 24414;
        }
        if self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 24295;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 24375;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::Low
        {
            return 24221;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#south == South::Tall
        {
            return 24293;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#waterlogged == true
        {
            return 24396;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 24462;
        }
        if self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#up == true
        {
            return 24475;
        }
        if self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#west == West::None
        {
            return 24244;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#up == false
        {
            return 24372;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#up == false
            && self.r#east == East::None
            && self.r#south == South::None
        {
            return 24170;
        }
        if self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == false
        {
            return 24434;
        }
        if self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::Tall
        {
            return 24441;
        }
        if self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#waterlogged == false
        {
            return 24429;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#up == false
        {
            return 24214;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 24269;
        }
        if self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#up == true
        {
            return 24343;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#east == East::Low
        {
            return 24362;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#west == West::None
        {
            return 24289;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#south == South::Tall
        {
            return 24225;
        }
        if self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#west == West::None
            && self.r#east == East::None
        {
            return 24238;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#west == West::Low
        {
            return 24239;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 24204;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 24367;
        }
        if self.r#east == East::None
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 24220;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#south == South::Low
        {
            return 24389;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#waterlogged == true
        {
            return 24274;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 24300;
        }
        if self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 24411;
        }
        if self.r#west == West::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == false
        {
            return 24433;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 24432;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#south == South::Low
        {
            return 24354;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#south == South::Tall
        {
            return 24371;
        }
        if self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#north == North::Tall
        {
            return 24249;
        }
        if self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#north == North::None
        {
            return 24279;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#south == South::None
        {
            return 24344;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#south == South::None
        {
            return 24171;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 24451 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 24252 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 24272 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 24194 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 24176 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 24306 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 24424 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 24460 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 24378 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 24216 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 24464 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 24342 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 24241 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 24380 {
            return Some(TuffBrickWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 24320 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 24363 {
            return Some(TuffBrickWall {
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 24191 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 24439 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 24230 {
            return Some(TuffBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 24273 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 24297 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 24400 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 24450 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 24415 {
            return Some(TuffBrickWall {
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 24174 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 24235 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 24331 {
            return Some(TuffBrickWall {
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 24307 {
            return Some(TuffBrickWall {
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 24259 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 24175 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 24298 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 24257 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 24359 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 24185 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 24398 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 24212 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 24169 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 24413 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 24296 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 24314 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 24391 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 24416 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 24303 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 24388 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24394 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 24417 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 24477 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24357 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 24412 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 24458 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 24229 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 24242 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 24301 {
            return Some(TuffBrickWall {
                r#up: false,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 24285 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 24218 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 24368 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 24200 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 24254 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 24374 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 24449 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 24206 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 24481 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 24326 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 24421 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 24292 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 24336 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 24470 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 24195 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 24207 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 24340 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 24471 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 24294 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 24205 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 24347 {
            return Some(TuffBrickWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 24457 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 24219 {
            return Some(TuffBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 24304 {
            return Some(TuffBrickWall {
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 24248 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 24427 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 24256 {
            return Some(TuffBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 24353 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 24190 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 24271 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 24430 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 24428 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 24168 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 24224 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24483 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 24234 {
            return Some(TuffBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 24261 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 24397 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 24334 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 24258 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 24315 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 24387 {
            return Some(TuffBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24312 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 24180 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 24268 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 24332 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 24352 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 24193 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 24324 {
            return Some(TuffBrickWall {
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 24377 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 24311 {
            return Some(TuffBrickWall {
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 24448 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 24386 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 24399 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 24382 {
            return Some(TuffBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 24455 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 24459 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 24379 {
            return Some(TuffBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 24446 {
            return Some(TuffBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 24201 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 24160 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24228 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24291 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 24187 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 24465 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 24365 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 24436 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 24404 {
            return Some(TuffBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 24443 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 24192 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24227 {
            return Some(TuffBrickWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 24476 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 24288 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 24321 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 24348 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 24328 {
            return Some(TuffBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 24251 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 24349 {
            return Some(TuffBrickWall {
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 24167 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24346 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 24317 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 24440 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 24453 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 24263 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 24461 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 24197 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 24173 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 24264 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 24166 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 24373 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 24438 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24199 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 24422 {
            return Some(TuffBrickWall {
                r#up: false,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 24284 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 24179 {
            return Some(TuffBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 24209 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 24240 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24255 {
            return Some(TuffBrickWall {
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 24482 {
            return Some(TuffBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 24198 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 24418 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24330 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 24188 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 24361 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 24162 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 24366 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 24211 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 24177 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 24325 {
            return Some(TuffBrickWall {
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 24217 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 24408 {
            return Some(TuffBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 24335 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 24351 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 24319 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 24350 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 24215 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 24323 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 24409 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 24322 {
            return Some(TuffBrickWall {
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 24426 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 24202 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 24163 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 24213 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 24208 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24266 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 24260 {
            return Some(TuffBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 24281 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 24437 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 24435 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 24406 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 24472 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 24275 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 24184 {
            return Some(TuffBrickWall {
                r#up: true,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 24327 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 24479 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 24277 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 24232 {
            return Some(TuffBrickWall {
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 24237 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 24250 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 24310 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24358 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 24189 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24308 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 24265 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 24339 {
            return Some(TuffBrickWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24345 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 24164 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 24210 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 24226 {
            return Some(TuffBrickWall {
                r#up: false,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 24313 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 24390 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24403 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 24407 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 24456 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 24183 {
            return Some(TuffBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 24233 {
            return Some(TuffBrickWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24384 {
            return Some(TuffBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24381 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24236 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 24280 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 24246 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 24276 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 24480 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 24447 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24395 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 24423 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 24287 {
            return Some(TuffBrickWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 24355 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 24473 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 24474 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 24278 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 24401 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 24253 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 24425 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 24444 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 24442 {
            return Some(TuffBrickWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 24290 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 24364 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 24299 {
            return Some(TuffBrickWall {
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24369 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 24182 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 24161 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 24383 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24305 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 24181 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 24178 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 24262 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 24165 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 24245 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 24467 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 24410 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 24329 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 24172 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 24452 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 24231 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24392 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 24402 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 24466 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 24469 {
            return Some(TuffBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 24337 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 24309 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 24196 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 24203 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 24286 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 24445 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 24478 {
            return Some(TuffBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 24247 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 24223 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 24270 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24283 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 24333 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24454 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 24370 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 24222 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 24376 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 24385 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 24468 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 24338 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 24282 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 24393 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 24463 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 24419 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 24360 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 24316 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 24302 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 24243 {
            return Some(TuffBrickWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 24420 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 24267 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 24186 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24341 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24318 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 24431 {
            return Some(TuffBrickWall {
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 24356 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 24405 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 24414 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24295 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 24375 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 24221 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 24293 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 24396 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 24462 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24475 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 24244 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 24372 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 24170 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 24434 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24441 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 24429 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24214 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 24269 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 24343 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 24362 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 24289 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 24225 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 24238 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 24239 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 24204 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 24367 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 24220 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 24389 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 24274 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24300 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 24411 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 24433 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24432 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 24354 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 24371 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 24249 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 24279 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 24344 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 24171 {
            return Some(TuffBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
            });
        }
        return None;
    }
}
