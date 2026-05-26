use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DioriteWall {
    pub up: bool,
    pub waterlogged: bool,
    pub r#west: West,
    pub r#south: South,
    pub r#east: East,
    pub r#north: North,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum West {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum East {
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

impl BlockState for DioriteWall {
    fn to_id(&self) -> i32 {
        if self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#north == North::Tall
        {
            return 20376;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::Tall
        {
            return 20360;
        }
        if self.r#waterlogged == false
            && self.r#up == false
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#west == West::Low
        {
            return 20202;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#up == true
        {
            return 20207;
        }
        if self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 20315;
        }
        if self.r#north == North::Low
            && self.r#west == West::None
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Low
        {
            return 20351;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 20419;
        }
        if self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Tall
        {
            return 20452;
        }
        if self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#waterlogged == false
        {
            return 20399;
        }
        if self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 20209;
        }
        if self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 20279;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#waterlogged == true
        {
            return 20192;
        }
        if self.r#west == West::Tall
            && self.r#up == false
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#waterlogged == true
        {
            return 20188;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 20233;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#east == East::None
        {
            return 20286;
        }
        if self.r#up == true
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
        {
            return 20435;
        }
        if self.r#up == true
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 20458;
        }
        if self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 20194;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 20468;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 20330;
        }
        if self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 20350;
        }
        if self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
        {
            return 20197;
        }
        if self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 20255;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 20185;
        }
        if self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 20340;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#south == South::Tall
        {
            return 20322;
        }
        if self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::None
        {
            return 20183;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#up == true
        {
            return 20280;
        }
        if self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#south == South::Low
        {
            return 20200;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#south == South::None
        {
            return 20289;
        }
        if self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#up == false
        {
            return 20345;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#up == true
            && self.r#east == East::Low
        {
            return 20288;
        }
        if self.r#up == true
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#east == East::Low
        {
            return 20361;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#up == true
        {
            return 20230;
        }
        if self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 20276;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#south == South::Tall
        {
            return 20384;
        }
        if self.r#up == false
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == false
        {
            return 20358;
        }
        if self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#north == North::None
        {
            return 20415;
        }
        if self.r#west == West::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#south == South::Tall
        {
            return 20278;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::None
            && self.r#west == West::Low
        {
            return 20475;
        }
        if self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#up == true
        {
            return 20293;
        }
        if self.r#south == South::None
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::None
            && self.r#waterlogged == true
        {
            return 20217;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#north == North::Low
        {
            return 20347;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Low
        {
            return 20373;
        }
        if self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#south == South::Low
        {
            return 20480;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 20488;
        }
        if self.r#up == true
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#west == West::Tall
        {
            return 20398;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 20411;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#east == East::Low
        {
            return 20299;
        }
        if self.r#west == West::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#north == North::Tall
        {
            return 20268;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Low
        {
            return 20344;
        }
        if self.r#up == true
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 20245;
        }
        if self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#north == North::None
        {
            return 20417;
        }
        if self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 20433;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#north == North::Tall
        {
            return 20474;
        }
        if self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 20250;
        }
        if self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#up == true
        {
            return 20471;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#east == East::Tall
        {
            return 20441;
        }
        if self.r#east == East::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#up == false
        {
            return 20275;
        }
        if self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::None
        {
            return 20370;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#west == West::Tall
        {
            return 20383;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#south == South::None
        {
            return 20297;
        }
        if self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::None
        {
            return 20420;
        }
        if self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::None
        {
            return 20320;
        }
        if self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 20364;
        }
        if self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 20391;
        }
        if self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
        {
            return 20430;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 20444;
        }
        if self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 20325;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::Low
        {
            return 20375;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Tall
        {
            return 20389;
        }
        if self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#north == North::None
        {
            return 20404;
        }
        if self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#up == true
            && self.r#west == West::None
            && self.r#north == North::None
        {
            return 20180;
        }
        if self.r#up == true
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 20193;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#west == West::Low
        {
            return 20451;
        }
        if self.r#up == false
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 20381;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::Tall
        {
            return 20473;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::None
        {
            return 20324;
        }
        if self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#up == true
            && self.r#south == South::None
        {
            return 20184;
        }
        if self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#east == East::None
        {
            return 20273;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::Low
        {
            return 20303;
        }
        if self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 20285;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::None
        {
            return 20424;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Low
        {
            return 20343;
        }
        if self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#up == false
            && self.r#south == South::None
            && self.r#west == West::Low
        {
            return 20226;
        }
        if self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 20287;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Tall
        {
            return 20323;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 20493;
        }
        if self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Tall
        {
            return 20499;
        }
        if self.r#up == true
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#east == East::Low
        {
            return 20329;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#east == East::None
        {
            return 20256;
        }
        if self.r#up == true
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 20460;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#north == North::Tall
        {
            return 20498;
        }
        if self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 20357;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#east == East::Low
        {
            return 20321;
        }
        if self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#waterlogged == true
        {
            return 20218;
        }
        if self.r#up == false
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#north == North::Low
        {
            return 20246;
        }
        if self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#south == South::Tall
        {
            return 20431;
        }
        if self.r#up == true
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 20305;
        }
        if self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#east == East::None
        {
            return 20262;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::None
        {
            return 20336;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::Low
        {
            return 20348;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::Tall
        {
            return 20390;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 20405;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 20463;
        }
        if self.r#west == West::Tall
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 20377;
        }
        if self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 20407;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Tall
        {
            return 20482;
        }
        if self.r#west == West::None
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Tall
        {
            return 20492;
        }
        if self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::Low
        {
            return 20243;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#north == North::Low
        {
            return 20338;
        }
        if self.r#west == West::Tall
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#waterlogged == false
        {
            return 20215;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#north == North::Tall
        {
            return 20490;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#up == true
            && self.r#north == North::Tall
        {
            return 20269;
        }
        if self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 20428;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#east == East::None
        {
            return 20248;
        }
        if self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 20380;
        }
        if self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#north == North::Tall
        {
            return 20470;
        }
        if self.r#west == West::None
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#up == false
        {
            return 20270;
        }
        if self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#up == true
        {
            return 20495;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 20446;
        }
        if self.r#waterlogged == false
            && self.r#up == false
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#south == South::Low
        {
            return 20203;
        }
        if self.r#up == true
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#south == South::Low
        {
            return 20413;
        }
        if self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#up == false
        {
            return 20296;
        }
        if self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#east == East::None
        {
            return 20242;
        }
        if self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#west == West::None
        {
            return 20429;
        }
        if self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#north == North::Low
        {
            return 20223;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::None
        {
            return 20272;
        }
        if self.r#up == false
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#waterlogged == false
        {
            return 20501;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Tall
        {
            return 20461;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 20252;
        }
        if self.r#north == North::None
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 20290;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#south == South::None
        {
            return 20335;
        }
        if self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#west == West::Low
        {
            return 20403;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#waterlogged == false
        {
            return 20363;
        }
        if self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 20258;
        }
        if self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#east == East::Low
        {
            return 20316;
        }
        if self.r#up == true
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Low
        {
            return 20196;
        }
        if self.r#south == South::Tall
            && self.r#up == true
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 20206;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Tall
        {
            return 20204;
        }
        if self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#up == false
        {
            return 20274;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 20304;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#north == North::Tall
        {
            return 20392;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Tall
        {
            return 20395;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == true
            && self.r#west == West::Low
        {
            return 20436;
        }
        if self.r#up == true
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Low
        {
            return 20449;
        }
        if self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == false
        {
            return 20259;
        }
        if self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 20457;
        }
        if self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::Tall
        {
            return 20467;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 20500;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#east == East::Low
        {
            return 20378;
        }
        if self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == true
        {
            return 20427;
        }
        if self.r#south == South::None
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#waterlogged == true
        {
            return 20295;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#up == false
        {
            return 20466;
        }
        if self.r#south == South::None
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Tall
        {
            return 20365;
        }
        if self.r#north == North::Low
            && self.r#west == West::None
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 20432;
        }
        if self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#east == East::Low
        {
            return 20382;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#west == West::None
        {
            return 20450;
        }
        if self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 20437;
        }
        if self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 20283;
        }
        if self.r#west == West::Low
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#waterlogged == true
        {
            return 20313;
        }
        if self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Low
        {
            return 20489;
        }
        if self.r#up == false
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::None
        {
            return 20462;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 20465;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
        {
            return 20356;
        }
        if self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#west == West::Low
        {
            return 20244;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#waterlogged == true
        {
            return 20301;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#east == East::None
            && self.r#west == West::Low
        {
            return 20187;
        }
        if self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#east == East::Low
        {
            return 20393;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#east == East::Tall
        {
            return 20396;
        }
        if self.r#west == West::None
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == true
        {
            return 20456;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 20314;
        }
        if self.r#up == false
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#waterlogged == true
        {
            return 20487;
        }
        if self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#south == South::Tall
        {
            return 20352;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#north == North::Tall
        {
            return 20481;
        }
        if self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 20220;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 20312;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 20284;
        }
        if self.r#west == West::Tall
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 20425;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 20219;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 20263;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#waterlogged == false
        {
            return 20311;
        }
        if self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::Tall
        {
            return 20448;
        }
        if self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#west == West::None
        {
            return 20216;
        }
        if self.r#up == false
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#north == North::Tall
        {
            return 20369;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#south == South::Tall
        {
            return 20394;
        }
        if self.r#up == false
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 20186;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#west == West::Tall
        {
            return 20224;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == true
        {
            return 20228;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 20362;
        }
        if self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#up == true
        {
            return 20484;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#waterlogged == false
        {
            return 20261;
        }
        if self.r#up == true
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Tall
        {
            return 20485;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 20374;
        }
        if self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::None
        {
            return 20265;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::Tall
        {
            return 20302;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#west == West::Tall
        {
            return 20401;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 20503;
        }
        if self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Low
        {
            return 20469;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#south == South::Tall
        {
            return 20353;
        }
        if self.r#west == West::Low
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 20292;
        }
        if self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#east == East::None
        {
            return 20198;
        }
        if self.r#west == West::Tall
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 20317;
        }
        if self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
        {
            return 20281;
        }
        if self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 20318;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#south == South::Tall
        {
            return 20277;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 20334;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#west == West::Low
        {
            return 20355;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == false
        {
            return 20438;
        }
        if self.r#waterlogged == true
            && self.r#up == false
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#west == West::Tall
        {
            return 20260;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#waterlogged == false
        {
            return 20387;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 20326;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == true
            && self.r#south == South::Low
        {
            return 20264;
        }
        if self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::None
        {
            return 20421;
        }
        if self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#west == West::Low
        {
            return 20439;
        }
        if self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#south == South::Low
        {
            return 20454;
        }
        if self.r#north == North::None
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#east == East::None
            && self.r#south == South::Low
        {
            return 20195;
        }
        if self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#up == true
        {
            return 20257;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#west == West::Low
        {
            return 20307;
        }
        if self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 20453;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 20341;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#east == East::Tall
        {
            return 20464;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#south == South::None
        {
            return 20397;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#east == East::Low
        {
            return 20385;
        }
        if self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#north == North::Tall
        {
            return 20372;
        }
        if self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#up == true
        {
            return 20472;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 20331;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#south == South::None
        {
            return 20479;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 20478;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == false
        {
            return 20234;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 20412;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 20497;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#north == North::Low
        {
            return 20445;
        }
        if self.r#west == West::None
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
        {
            return 20267;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::Low
        {
            return 20211;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::None
        {
            return 20222;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 20332;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#waterlogged == true
        {
            return 20342;
        }
        if self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 20502;
        }
        if self.r#up == false
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::None
        {
            return 20426;
        }
        if self.r#west == West::None
            && self.r#up == false
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
        {
            return 20225;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#up == false
        {
            return 20486;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#north == North::None
        {
            return 20208;
        }
        if self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 20346;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::None
            && self.r#waterlogged == false
        {
            return 20221;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#up == false
        {
            return 20354;
        }
        if self.r#up == false
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#north == North::Tall
        {
            return 20371;
        }
        if self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::Tall
        {
            return 20459;
        }
        if self.r#west == West::Tall
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#waterlogged == true
        {
            return 20368;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 20254;
        }
        if self.r#west == West::Low
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#waterlogged == true
        {
            return 20337;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 20251;
        }
        if self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == false
        {
            return 20402;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#north == North::Tall
        {
            return 20253;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::None
        {
            return 20189;
        }
        if self.r#up == true
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Tall
        {
            return 20386;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#north == North::None
        {
            return 20319;
        }
        if self.r#west == West::Low
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 20214;
        }
        if self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 20349;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#west == West::Tall
        {
            return 20455;
        }
        if self.r#south == South::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
        {
            return 20400;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#up == false
        {
            return 20308;
        }
        if self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#up == true
        {
            return 20291;
        }
        if self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#east == East::None
        {
            return 20235;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#east == East::Low
        {
            return 20328;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 20294;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Low
        {
            return 20310;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 20408;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 20476;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 20238;
        }
        if self.r#west == West::Low
            && self.r#north == North::None
            && self.r#up == true
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#east == East::None
        {
            return 20181;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#south == South::None
        {
            return 20434;
        }
        if self.r#up == false
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::Tall
        {
            return 20213;
        }
        if self.r#west == West::None
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#south == South::Low
        {
            return 20339;
        }
        if self.r#west == West::Low
            && self.r#up == false
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#east == East::None
        {
            return 20190;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == false
        {
            return 20236;
        }
        if self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#up == true
        {
            return 20205;
        }
        if self.r#waterlogged == true
            && self.r#up == false
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#south == South::Tall
        {
            return 20210;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#east == East::None
        {
            return 20271;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#north == North::Low
        {
            return 20247;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#waterlogged == true
        {
            return 20306;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 20182;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#west == West::Tall
        {
            return 20227;
        }
        if self.r#east == East::None
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#north == North::None
            && self.r#west == West::Low
        {
            return 20199;
        }
        if self.r#north == North::Low
            && self.r#east == East::None
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 20249;
        }
        if self.r#north == North::None
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#south == South::None
        {
            return 20298;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#up == true
        {
            return 20327;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::None
        {
            return 20406;
        }
        if self.r#south == South::Low
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 20414;
        }
        if self.r#up == true
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#west == West::None
        {
            return 20447;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 20191;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 20496;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#up == false
        {
            return 20239;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#north == North::Tall
        {
            return 20494;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == false
        {
            return 20477;
        }
        if self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::Low
        {
            return 20379;
        }
        if self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#south == South::Low
        {
            return 20266;
        }
        if self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 20410;
        }
        if self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 20212;
        }
        if self.r#east == East::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
        {
            return 20237;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#west == West::Tall
        {
            return 20440;
        }
        if self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#east == East::Low
        {
            return 20367;
        }
        if self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#south == South::Tall
        {
            return 20241;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::Low
        {
            return 20309;
        }
        if self.r#up == true
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#east == East::None
        {
            return 20229;
        }
        if self.r#up == true
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::Low
        {
            return 20232;
        }
        if self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 20418;
        }
        if self.r#up == true
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#south == South::Tall
        {
            return 20422;
        }
        if self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#up == false
        {
            return 20442;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#west == West::None
        {
            return 20300;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#up == true
        {
            return 20409;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#up == true
        {
            return 20388;
        }
        if self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Low
        {
            return 20416;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#east == East::Tall
        {
            return 20491;
        }
        if self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 20359;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#north == North::None
        {
            return 20201;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#west == West::None
        {
            return 20282;
        }
        if self.r#west == West::None
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#up == false
        {
            return 20366;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#south == South::Tall
        {
            return 20240;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 20231;
        }
        if self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == false
        {
            return 20443;
        }
        if self.r#west == West::None
            && self.r#up == false
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#waterlogged == false
        {
            return 20333;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#north == North::None
        {
            return 20423;
        }
        if self.r#up == true
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 20483;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20376 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 20360 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 20202 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#up: false,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 20207 {
            return Some(DioriteWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 20315 {
            return Some(DioriteWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 20351 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#west: West::None,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 20419 {
            return Some(DioriteWall {
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 20452 {
            return Some(DioriteWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 20399 {
            return Some(DioriteWall {
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20209 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 20279 {
            return Some(DioriteWall {
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20192 {
            return Some(DioriteWall {
                r#east: East::None,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 20188 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 20233 {
            return Some(DioriteWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 20286 {
            return Some(DioriteWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 20435 {
            return Some(DioriteWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 20458 {
            return Some(DioriteWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 20194 {
            return Some(DioriteWall {
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 20468 {
            return Some(DioriteWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 20330 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 20350 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20197 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 20255 {
            return Some(DioriteWall {
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20185 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 20340 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 20322 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 20183 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 20280 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 20200 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 20289 {
            return Some(DioriteWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 20345 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 20288 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 20361 {
            return Some(DioriteWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 20230 {
            return Some(DioriteWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 20276 {
            return Some(DioriteWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 20384 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 20358 {
            return Some(DioriteWall {
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 20415 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 20278 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 20475 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 20293 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 20217 {
            return Some(DioriteWall {
                r#south: South::None,
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 20347 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 20373 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 20480 {
            return Some(DioriteWall {
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 20488 {
            return Some(DioriteWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 20398 {
            return Some(DioriteWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 20411 {
            return Some(DioriteWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 20299 {
            return Some(DioriteWall {
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 20268 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 20344 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 20245 {
            return Some(DioriteWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 20417 {
            return Some(DioriteWall {
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 20433 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 20474 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 20250 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 20471 {
            return Some(DioriteWall {
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 20441 {
            return Some(DioriteWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 20275 {
            return Some(DioriteWall {
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 20370 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 20383 {
            return Some(DioriteWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 20297 {
            return Some(DioriteWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 20420 {
            return Some(DioriteWall {
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 20320 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 20364 {
            return Some(DioriteWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 20391 {
            return Some(DioriteWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20430 {
            return Some(DioriteWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 20444 {
            return Some(DioriteWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 20325 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 20375 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 20389 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 20404 {
            return Some(DioriteWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 20180 {
            return Some(DioriteWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 20193 {
            return Some(DioriteWall {
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 20451 {
            return Some(DioriteWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 20381 {
            return Some(DioriteWall {
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 20473 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 20324 {
            return Some(DioriteWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 20184 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 20273 {
            return Some(DioriteWall {
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 20303 {
            return Some(DioriteWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 20285 {
            return Some(DioriteWall {
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 20424 {
            return Some(DioriteWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 20343 {
            return Some(DioriteWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 20226 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 20287 {
            return Some(DioriteWall {
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 20323 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 20493 {
            return Some(DioriteWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 20499 {
            return Some(DioriteWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 20329 {
            return Some(DioriteWall {
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 20256 {
            return Some(DioriteWall {
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 20460 {
            return Some(DioriteWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 20498 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 20357 {
            return Some(DioriteWall {
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 20321 {
            return Some(DioriteWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 20218 {
            return Some(DioriteWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 20246 {
            return Some(DioriteWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 20431 {
            return Some(DioriteWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 20305 {
            return Some(DioriteWall {
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 20262 {
            return Some(DioriteWall {
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 20336 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 20348 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 20390 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 20405 {
            return Some(DioriteWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 20463 {
            return Some(DioriteWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20377 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#up: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 20407 {
            return Some(DioriteWall {
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 20482 {
            return Some(DioriteWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 20492 {
            return Some(DioriteWall {
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 20243 {
            return Some(DioriteWall {
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 20338 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 20215 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#up: false,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20490 {
            return Some(DioriteWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 20269 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 20428 {
            return Some(DioriteWall {
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20248 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 20380 {
            return Some(DioriteWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 20470 {
            return Some(DioriteWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 20270 {
            return Some(DioriteWall {
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 20495 {
            return Some(DioriteWall {
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 20446 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20203 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 20413 {
            return Some(DioriteWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 20296 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 20242 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 20429 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 20223 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 20272 {
            return Some(DioriteWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 20501 {
            return Some(DioriteWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20461 {
            return Some(DioriteWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 20252 {
            return Some(DioriteWall {
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20290 {
            return Some(DioriteWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 20335 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 20403 {
            return Some(DioriteWall {
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 20363 {
            return Some(DioriteWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20258 {
            return Some(DioriteWall {
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 20316 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 20196 {
            return Some(DioriteWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 20206 {
            return Some(DioriteWall {
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 20204 {
            return Some(DioriteWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 20274 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 20304 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 20392 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 20395 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 20436 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 20449 {
            return Some(DioriteWall {
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 20259 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 20457 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 20467 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 20500 {
            return Some(DioriteWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 20378 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 20427 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 20295 {
            return Some(DioriteWall {
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 20466 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 20365 {
            return Some(DioriteWall {
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 20432 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 20382 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 20450 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 20437 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20283 {
            return Some(DioriteWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 20313 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 20489 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 20462 {
            return Some(DioriteWall {
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 20465 {
            return Some(DioriteWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 20356 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 20244 {
            return Some(DioriteWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 20301 {
            return Some(DioriteWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 20187 {
            return Some(DioriteWall {
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 20393 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 20396 {
            return Some(DioriteWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 20456 {
            return Some(DioriteWall {
                r#west: West::None,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 20314 {
            return Some(DioriteWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 20487 {
            return Some(DioriteWall {
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 20352 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 20481 {
            return Some(DioriteWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 20220 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20312 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 20284 {
            return Some(DioriteWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 20425 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 20219 {
            return Some(DioriteWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 20263 {
            return Some(DioriteWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 20311 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 20448 {
            return Some(DioriteWall {
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 20216 {
            return Some(DioriteWall {
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 20369 {
            return Some(DioriteWall {
                r#up: false,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 20394 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 20186 {
            return Some(DioriteWall {
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 20224 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 20228 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 20362 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 20484 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 20261 {
            return Some(DioriteWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20485 {
            return Some(DioriteWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 20374 {
            return Some(DioriteWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 20265 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 20302 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 20401 {
            return Some(DioriteWall {
                r#south: South::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 20503 {
            return Some(DioriteWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20469 {
            return Some(DioriteWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 20353 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 20292 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 20198 {
            return Some(DioriteWall {
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 20317 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 20281 {
            return Some(DioriteWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 20318 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 20277 {
            return Some(DioriteWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 20334 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 20355 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 20438 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 20260 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 20387 {
            return Some(DioriteWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 20326 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 20264 {
            return Some(DioriteWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 20421 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 20439 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 20454 {
            return Some(DioriteWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 20195 {
            return Some(DioriteWall {
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 20257 {
            return Some(DioriteWall {
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 20307 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 20453 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 20341 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 20464 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 20397 {
            return Some(DioriteWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 20385 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 20372 {
            return Some(DioriteWall {
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 20472 {
            return Some(DioriteWall {
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 20331 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 20479 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 20478 {
            return Some(DioriteWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 20234 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 20412 {
            return Some(DioriteWall {
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20497 {
            return Some(DioriteWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20445 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 20267 {
            return Some(DioriteWall {
                r#west: West::None,
                r#up: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20211 {
            return Some(DioriteWall {
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 20222 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 20332 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 20342 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 20502 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20426 {
            return Some(DioriteWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 20225 {
            return Some(DioriteWall {
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 20486 {
            return Some(DioriteWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 20208 {
            return Some(DioriteWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 20346 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 20221 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20354 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 20371 {
            return Some(DioriteWall {
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 20459 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 20368 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 20254 {
            return Some(DioriteWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 20337 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 20251 {
            return Some(DioriteWall {
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20402 {
            return Some(DioriteWall {
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 20253 {
            return Some(DioriteWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 20189 {
            return Some(DioriteWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 20386 {
            return Some(DioriteWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 20319 {
            return Some(DioriteWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 20214 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 20349 {
            return Some(DioriteWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 20455 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 20400 {
            return Some(DioriteWall {
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 20308 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 20291 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 20235 {
            return Some(DioriteWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 20328 {
            return Some(DioriteWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 20294 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 20310 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 20408 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 20476 {
            return Some(DioriteWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 20238 {
            return Some(DioriteWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 20181 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 20434 {
            return Some(DioriteWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 20213 {
            return Some(DioriteWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 20339 {
            return Some(DioriteWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 20190 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 20236 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 20205 {
            return Some(DioriteWall {
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 20210 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#up: false,
                r#north: North::None,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 20271 {
            return Some(DioriteWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 20247 {
            return Some(DioriteWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 20306 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 20182 {
            return Some(DioriteWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 20227 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 20199 {
            return Some(DioriteWall {
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 20249 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 20298 {
            return Some(DioriteWall {
                r#north: North::None,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 20327 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 20406 {
            return Some(DioriteWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 20414 {
            return Some(DioriteWall {
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 20447 {
            return Some(DioriteWall {
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 20191 {
            return Some(DioriteWall {
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 20496 {
            return Some(DioriteWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 20239 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 20494 {
            return Some(DioriteWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 20477 {
            return Some(DioriteWall {
                r#south: South::None,
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 20379 {
            return Some(DioriteWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 20266 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 20410 {
            return Some(DioriteWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 20212 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20237 {
            return Some(DioriteWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 20440 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 20367 {
            return Some(DioriteWall {
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 20241 {
            return Some(DioriteWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 20309 {
            return Some(DioriteWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 20229 {
            return Some(DioriteWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 20232 {
            return Some(DioriteWall {
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 20418 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 20422 {
            return Some(DioriteWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 20442 {
            return Some(DioriteWall {
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 20300 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 20409 {
            return Some(DioriteWall {
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 20388 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 20416 {
            return Some(DioriteWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 20491 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 20359 {
            return Some(DioriteWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 20201 {
            return Some(DioriteWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 20282 {
            return Some(DioriteWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 20366 {
            return Some(DioriteWall {
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 20240 {
            return Some(DioriteWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 20231 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 20443 {
            return Some(DioriteWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20333 {
            return Some(DioriteWall {
                r#west: West::None,
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 20423 {
            return Some(DioriteWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 20483 {
            return Some(DioriteWall {
                r#up: true,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
