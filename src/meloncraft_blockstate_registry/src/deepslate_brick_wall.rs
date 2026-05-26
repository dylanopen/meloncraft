use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateBrickWall {
    pub up: bool,
    pub r#south: South,
    pub waterlogged: bool,
    pub r#north: North,
    pub r#west: West,
    pub r#east: East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum South {
    None,
    Low,
    Tall,
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

impl BlockState for DeepslateBrickWall {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#south == South::Tall
        {
            return 29217;
        }
        if self.r#up == false
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::Low
        {
            return 29330;
        }
        if self.r#west == West::Low
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 29180;
        }
        if self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::None
        {
            return 29302;
        }
        if self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#up == true
        {
            return 29152;
        }
        if self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 29342;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 29048;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#up == true
            && self.r#east == East::None
            && self.r#west == West::Tall
        {
            return 29085;
        }
        if self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 29252;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#south == South::None
        {
            return 29154;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#south == South::Tall
        {
            return 29257;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#south == South::Low
        {
            return 29344;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#east == East::None
            && self.r#north == North::Low
        {
            return 29105;
        }
        if self.r#up == true
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#east == East::None
        {
            return 29094;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == true
        {
            return 29104;
        }
        if self.r#east == East::Low
            && self.r#west == West::None
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#north == North::None
        {
            return 29167;
        }
        if self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#up == false
        {
            return 29221;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 29310;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::Tall
        {
            return 29357;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 29155;
        }
        if self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#up == false
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Tall
        {
            return 29077;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#up == false
        {
            return 29125;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#up == false
        {
            return 29364;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 29191;
        }
        if self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#south == South::Tall
        {
            return 29293;
        }
        if self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#up == true
            && self.r#north == North::Tall
        {
            return 29224;
        }
        if self.r#up == false
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#south == South::Low
        {
            return 29175;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#west == West::None
        {
            return 29365;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::Tall
        {
            return 29366;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::Low
        {
            return 29096;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#waterlogged == false
        {
            return 29198;
        }
        if self.r#up == true
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == true
        {
            return 29140;
        }
        if self.r#up == true
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 29263;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Tall
        {
            return 29280;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#up == true
            && self.r#south == South::None
            && self.r#north == North::Tall
        {
            return 29335;
        }
        if self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#west == West::None
        {
            return 29098;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 29119;
        }
        if self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 29112;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 29109;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 29075;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#waterlogged == true
        {
            return 29237;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 29157;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 29207;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 29069;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::None
        {
            return 29050;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 29367;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 29145;
        }
        if self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 29156;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 29045;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#north == North::Tall
        {
            return 29239;
        }
        if self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#east == East::None
        {
            return 29091;
        }
        if self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#up == true
        {
            return 29299;
        }
        if self.r#up == false
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#waterlogged == true
        {
            return 29242;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 29090;
        }
        if self.r#up == true
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#south == South::Low
        {
            return 29166;
        }
        if self.r#up == false
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 29187;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 29331;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 29052;
        }
        if self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 29100;
        }
        if self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#north == North::Low
        {
            return 29106;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 29107;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
        {
            return 29149;
        }
        if self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#north == North::Low
        {
            return 29201;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::Low
        {
            return 29211;
        }
        if self.r#east == East::None
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
        {
            return 29138;
        }
        if self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
        {
            return 29181;
        }
        if self.r#south == South::None
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 29054;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#south == South::None
        {
            return 29124;
        }
        if self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#west == West::Low
        {
            return 29267;
        }
        if self.r#west == West::Tall
            && self.r#up == false
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == false
        {
            return 29235;
        }
        if self.r#south == South::None
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#up == true
            && self.r#west == West::None
        {
            return 29044;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 29328;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#east == East::Low
        {
            return 29258;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::None
            && self.r#south == South::Tall
        {
            return 29254;
        }
        if self.r#up == false
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#north == North::None
        {
            return 29074;
        }
        if self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#north == North::Low
        {
            return 29193;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#west == West::Low
        {
            return 29327;
        }
        if self.r#up == true
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#east == East::Tall
        {
            return 29320;
        }
        if self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#north == North::Tall
        {
            return 29341;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#west == West::Tall
        {
            return 29295;
        }
        if self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 29313;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 29062;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#waterlogged == true
        {
            return 29172;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#up == false
        {
            return 29230;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Low
        {
            return 29276;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 29057;
        }
        if self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 29132;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#waterlogged == true
        {
            return 29296;
        }
        if self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 29189;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#south == South::Low
        {
            return 29210;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 29161;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Low
        {
            return 29346;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#north == North::None
        {
            return 29056;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 29070;
        }
        if self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#east == East::None
            && self.r#south == South::Low
        {
            return 29060;
        }
        if self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#up == true
        {
            return 29348;
        }
        if self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::Low
        {
            return 29084;
        }
        if self.r#up == false
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 29171;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 29159;
        }
        if self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 29123;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 29283;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 29092;
        }
        if self.r#west == West::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 29101;
        }
        if self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#north == North::None
            && self.r#west == West::Tall
        {
            return 29265;
        }
        if self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 29318;
        }
        if self.r#north == North::Low
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
        {
            return 29095;
        }
        if self.r#up == true
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#south == South::None
        {
            return 29297;
        }
        if self.r#waterlogged == false
            && self.r#up == false
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::Low
        {
            return 29113;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#up == false
        {
            return 29339;
        }
        if self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::None
        {
            return 29110;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#up == true
        {
            return 29261;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
        {
            return 29337;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#east == East::Low
        {
            return 29248;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 29133;
        }
        if self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#up == true
        {
            return 29272;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#up == false
        {
            return 29066;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#east == East::Tall
        {
            return 29334;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == true
        {
            return 29322;
        }
        if self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 29323;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#north == North::None
        {
            return 29286;
        }
        if self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#west == West::Low
        {
            return 29114;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 29061;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 29205;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == true
        {
            return 29284;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#east == East::Low
        {
            return 29249;
        }
        if self.r#west == West::None
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#north == North::None
        {
            return 29068;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#west == West::None
        {
            return 29137;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 29146;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
        {
            return 29311;
        }
        if self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#up == false
            && self.r#north == North::Low
        {
            return 29087;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#up == true
        {
            return 29080;
        }
        if self.r#south == South::Tall
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 29250;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#east == East::None
        {
            return 29071;
        }
        if self.r#north == North::None
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 29178;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#up == false
        {
            return 29127;
        }
        if self.r#west == West::None
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#up == false
        {
            return 29314;
        }
        if self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 29332;
        }
        if self.r#up == true
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::Tall
        {
            return 29128;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 29349;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 29111;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#north == North::None
        {
            return 29079;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::None
        {
            return 29227;
        }
        if self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 29231;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#up == false
        {
            return 29102;
        }
        if self.r#waterlogged == false
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#north == North::Tall
        {
            return 29150;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 29047;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#up == false
        {
            return 29281;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::Low
        {
            return 29244;
        }
        if self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 29073;
        }
        if self.r#up == false
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::None
        {
            return 29136;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == true
        {
            return 29345;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 29143;
        }
        if self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::None
        {
            return 29053;
        }
        if self.r#north == North::None
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#waterlogged == false
        {
            return 29169;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Low
        {
            return 29202;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::Tall
        {
            return 29288;
        }
        if self.r#west == West::Low
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#waterlogged == true
        {
            return 29135;
        }
        if self.r#west == West::Low
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#east == East::Low
        {
            return 29192;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Low
        {
            return 29165;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 29195;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 29260;
        }
        if self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#north == North::None
        {
            return 29184;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 29097;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#west == West::Low
        {
            return 29051;
        }
        if self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 29117;
        }
        if self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#up == false
        {
            return 29158;
        }
        if self.r#up == false
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == true
        {
            return 29183;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::None
        {
            return 29046;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#up == false
        {
            return 29271;
        }
        if self.r#east == East::Tall
            && self.r#up == false
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#waterlogged == true
        {
            return 29279;
        }
        if self.r#up == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 29301;
        }
        if self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 29308;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#west == West::Low
        {
            return 29309;
        }
        if self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#up == false
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#south == South::Low
        {
            return 29134;
        }
        if self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#up == true
        {
            return 29285;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#west == West::Low
        {
            return 29351;
        }
        if self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 29251;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#east == East::Tall
        {
            return 29362;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 29298;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == true
        {
            return 29118;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::Low
        {
            return 29188;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == true
        {
            return 29228;
        }
        if self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#south == South::Low
        {
            return 29353;
        }
        if self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#south == South::None
        {
            return 29194;
        }
        if self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#east == East::Tall
        {
            return 29294;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#west == West::Low
        {
            return 29255;
        }
        if self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#up == false
            && self.r#north == North::Tall
        {
            return 29122;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#up == true
        {
            return 29226;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#east == East::Tall
        {
            return 29355;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 29067;
        }
        if self.r#west == West::Low
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#waterlogged == false
        {
            return 29336;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#up == true
        {
            return 29325;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 29321;
        }
        if self.r#west == West::Low
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Low
        {
            return 29225;
        }
        if self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::Tall
        {
            return 29282;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 29199;
        }
        if self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#up == false
        {
            return 29065;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#up == true
        {
            return 29275;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#west == West::Tall
        {
            return 29142;
        }
        if self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 29139;
        }
        if self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#south == South::None
        {
            return 29116;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#west == West::None
        {
            return 29203;
        }
        if self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 29222;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 29266;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#west == West::Tall
        {
            return 29055;
        }
        if self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 29274;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#west == West::None
        {
            return 29212;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 29219;
        }
        if self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Low
        {
            return 29315;
        }
        if self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#west == West::Low
        {
            return 29078;
        }
        if self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#north == North::None
        {
            return 29290;
        }
        if self.r#east == East::Low
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 29200;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#west == West::Tall
        {
            return 29238;
        }
        if self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#waterlogged == false
        {
            return 29241;
        }
        if self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::Tall
        {
            return 29287;
        }
        if self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#up == true
        {
            return 29356;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 29115;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == false
        {
            return 29304;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 29086;
        }
        if self.r#up == false
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 29209;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#east == East::Low
        {
            return 29214;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == true
        {
            return 29312;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#south == South::Low
        {
            return 29240;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#north == North::Low
        {
            return 29317;
        }
        if self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::None
            && self.r#waterlogged == false
        {
            return 29103;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#up == false
        {
            return 29170;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 29243;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#north == North::Tall
        {
            return 29141;
        }
        if self.r#up == true
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::Tall
        {
            return 29289;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#south == South::None
        {
            return 29307;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#up == false
        {
            return 29185;
        }
        if self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 29305;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#waterlogged == false
        {
            return 29319;
        }
        if self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#up == true
        {
            return 29361;
        }
        if self.r#north == North::None
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#south == South::Tall
        {
            return 29177;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#south == South::None
        {
            return 29190;
        }
        if self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#west == West::Tall
        {
            return 29196;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 29179;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 29360;
        }
        if self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 29277;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#west == West::Tall
        {
            return 29130;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::Low
        {
            return 29354;
        }
        if self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 29072;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#south == South::None
        {
            return 29121;
        }
        if self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#north == North::Tall
        {
            return 29229;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 29268;
        }
        if self.r#north == North::None
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 29163;
        }
        if self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 29126;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#north == North::None
        {
            return 29153;
        }
        if self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 29329;
        }
        if self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#west == West::None
        {
            return 29350;
        }
        if self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == true
            && self.r#north == North::Low
        {
            return 29300;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Tall
        {
            return 29147;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 29278;
        }
        if self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == true
        {
            return 29291;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 29148;
        }
        if self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 29324;
        }
        if self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 29340;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::None
        {
            return 29234;
        }
        if self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::Low
        {
            return 29063;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#south == South::Tall
        {
            return 29151;
        }
        if self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::Low
        {
            return 29232;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == false
        {
            return 29363;
        }
        if self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#south == South::Tall
        {
            return 29359;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 29247;
        }
        if self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::None
        {
            return 29059;
        }
        if self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#north == North::None
        {
            return 29270;
        }
        if self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#up == true
        {
            return 29129;
        }
        if self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#north == North::Tall
        {
            return 29131;
        }
        if self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#waterlogged == true
        {
            return 29081;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#up == false
        {
            return 29220;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 29253;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::None
        {
            return 29120;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 29162;
        }
        if self.r#west == West::Low
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#east == East::Tall
        {
            return 29333;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 29168;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#west == West::None
        {
            return 29236;
        }
        if self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#north == North::Tall
        {
            return 29233;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#waterlogged == true
        {
            return 29088;
        }
        if self.r#north == North::None
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 29174;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 29058;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::Low
        {
            return 29108;
        }
        if self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Tall
        {
            return 29256;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 29173;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#east == East::None
        {
            return 29083;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 29182;
        }
        if self.r#up == true
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Low
        {
            return 29176;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#south == South::None
        {
            return 29264;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#north == North::None
        {
            return 29064;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Low
        {
            return 29273;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::None
        {
            return 29306;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#east == East::Tall
        {
            return 29316;
        }
        if self.r#east == East::Tall
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 29338;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#south == South::None
        {
            return 29089;
        }
        if self.r#west == West::Tall
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 29259;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#west == West::None
        {
            return 29215;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#north == North::Tall
        {
            return 29245;
        }
        if self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#up == false
        {
            return 29269;
        }
        if self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::Tall
        {
            return 29292;
        }
        if self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#north == North::None
            && self.r#west == West::Tall
        {
            return 29076;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#waterlogged == false
        {
            return 29197;
        }
        if self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Low
        {
            return 29246;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#west == West::Low
        {
            return 29144;
        }
        if self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#north == North::Low
        {
            return 29223;
        }
        if self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 29082;
        }
        if self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == false
            && self.r#east == East::Low
        {
            return 29218;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#west == West::None
            && self.r#south == South::Low
        {
            return 29347;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#east == East::None
        {
            return 29049;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#up == false
        {
            return 29343;
        }
        if self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#south == South::Tall
        {
            return 29326;
        }
        if self.r#north == North::None
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 29186;
        }
        if self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 29093;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#up == false
        {
            return 29352;
        }
        if self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 29262;
        }
        if self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#north == North::Low
        {
            return 29099;
        }
        if self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 29303;
        }
        if self.r#west == West::None
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#waterlogged == true
        {
            return 29206;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#south == South::Low
        {
            return 29208;
        }
        if self.r#south == South::Tall
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 29358;
        }
        if self.r#up == true
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#north == North::None
        {
            return 29164;
        }
        if self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 29204;
        }
        if self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#south == South::Tall
        {
            return 29216;
        }
        if self.r#north == North::None
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 29160;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#south == South::Tall
        {
            return 29213;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29217 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 29330 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 29180 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 29302 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 29152 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 29342 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 29048 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 29085 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 29252 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 29154 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 29257 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 29344 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 29105 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 29094 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 29104 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 29167 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 29221 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 29310 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 29357 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 29155 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 29077 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 29125 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 29364 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 29191 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 29293 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 29224 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 29175 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 29365 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 29366 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 29096 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29198 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 29140 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 29263 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 29280 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 29335 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 29098 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 29119 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 29112 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29109 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 29075 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 29237 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 29157 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 29207 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 29069 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 29050 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 29367 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 29145 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 29156 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 29045 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 29239 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 29091 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 29299 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 29242 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 29090 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 29166 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 29187 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 29331 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 29052 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29100 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 29106 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 29107 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 29149 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29201 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29211 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 29138 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29181 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 29054 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 29124 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 29267 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 29235 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 29044 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 29328 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 29258 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 29254 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 29074 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 29193 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29327 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 29320 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 29341 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 29295 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 29313 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29062 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 29172 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 29230 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 29276 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 29057 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 29132 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 29296 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#up: true,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 29189 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 29210 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 29161 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 29346 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 29056 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 29070 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 29060 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 29348 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 29084 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 29171 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 29159 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 29123 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 29283 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 29092 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 29101 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 29265 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 29318 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 29095 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 29297 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 29113 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 29339 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 29110 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 29261 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 29337 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 29248 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 29133 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 29272 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 29066 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 29334 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 29322 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 29323 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 29286 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 29114 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 29061 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 29205 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 29284 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 29249 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 29068 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 29137 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 29146 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 29311 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 29087 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 29080 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 29250 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 29071 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 29178 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29127 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 29314 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 29332 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 29128 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 29349 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29111 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 29079 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 29227 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 29231 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 29102 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 29150 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 29047 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 29281 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 29244 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 29073 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 29136 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 29345 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 29143 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 29053 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 29169 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29202 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 29288 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 29135 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 29192 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 29165 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 29195 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 29260 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 29184 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 29097 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29051 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 29117 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 29158 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 29183 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 29046 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 29271 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 29279 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 29301 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 29308 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 29309 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 29134 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 29285 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 29351 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 29251 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 29362 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 29298 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 29118 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 29188 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 29228 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 29353 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 29194 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 29294 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 29255 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 29122 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 29226 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 29355 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 29067 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 29336 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 29325 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 29321 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 29225 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 29282 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 29199 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 29065 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 29275 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 29142 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 29139 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 29116 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 29203 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 29222 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 29266 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 29055 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 29274 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29212 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 29219 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 29315 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29078 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 29290 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 29200 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 29238 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 29241 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29287 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 29356 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 29115 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 29304 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 29086 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 29209 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 29214 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 29312 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 29240 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 29317 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 29103 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 29170 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 29243 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 29141 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 29289 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 29307 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 29185 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 29305 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 29319 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29361 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 29177 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 29190 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 29196 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 29179 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 29360 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29277 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 29130 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 29354 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 29072 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 29121 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 29229 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 29268 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29163 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 29126 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 29153 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 29329 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 29350 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 29300 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 29147 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 29278 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 29291 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 29148 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 29324 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 29340 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 29234 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 29063 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 29151 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 29232 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 29363 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 29359 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 29247 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 29059 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 29270 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 29129 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 29131 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 29081 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 29220 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 29253 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29120 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 29162 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 29333 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 29168 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 29236 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 29233 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 29088 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 29174 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 29058 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29108 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29256 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 29173 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 29083 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 29182 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 29176 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 29264 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 29064 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 29273 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 29306 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 29316 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 29338 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 29089 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 29259 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29215 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 29245 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 29269 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 29292 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 29076 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 29197 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 29246 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 29144 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 29223 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 29082 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 29218 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 29347 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 29049 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 29343 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 29326 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 29186 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 29093 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 29352 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 29262 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29099 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29303 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 29206 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#up: false,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 29208 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 29358 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 29164 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 29204 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 29216 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 29160 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29213 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        return None;
    }
}
