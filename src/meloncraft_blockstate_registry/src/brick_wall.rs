use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrickWall {
    pub r#north: North,
    pub up: bool,
    pub r#south: South,
    pub r#east: East,
    pub waterlogged: bool,
    pub r#west: West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum North {
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
pub enum West {
    None,
    Low,
    Tall,
}

impl BlockState for BrickWall {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#north == North::Tall
        {
            return 16605;
        }
        if self.r#south == South::Low
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 16423;
        }
        if self.r#up == true
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Tall
        {
            return 16581;
        }
        if self.r#north == North::Low
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#west == West::Tall
        {
            return 16363;
        }
        if self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#up == true
        {
            return 16460;
        }
        if self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#up == true
            && self.r#east == East::None
        {
            return 16306;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 16516;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#east == East::Tall
        {
            return 16598;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::Tall
        {
            return 16323;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::None
        {
            return 16418;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 16383;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 16570;
        }
        if self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 16333;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 16463;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#south == South::Low
        {
            return 16377;
        }
        if self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 16324;
        }
        if self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#up == true
        {
            return 16393;
        }
        if self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 16404;
        }
        if self.r#south == South::None
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#east == East::Tall
        {
            return 16554;
        }
        if self.r#west == West::None
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Tall
        {
            return 16385;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::Tall
        {
            return 16568;
        }
        if self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#up == true
        {
            return 16582;
        }
        if self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 16434;
        }
        if self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 16302;
        }
        if self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 16525;
        }
        if self.r#west == West::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#south == South::Low
        {
            return 16344;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#south == South::Tall
        {
            return 16615;
        }
        if self.r#up == true
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#west == West::Tall
        {
            return 16597;
        }
        if self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#west == West::None
            && self.r#north == North::None
        {
            return 16319;
        }
        if self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#north == North::Tall
        {
            return 16370;
        }
        if self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Low
        {
            return 16553;
        }
        if self.r#up == false
            && self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 16360;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#west == West::Tall
        {
            return 16366;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 16408;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 16481;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Low
        {
            return 16341;
        }
        if self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 16468;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#east == East::Low
        {
            return 16462;
        }
        if self.r#up == true
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#west == West::Low
        {
            return 16488;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 16608;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#south == South::Tall
        {
            return 16501;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 16524;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#west == West::Tall
        {
            return 16294;
        }
        if self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 16432;
        }
        if self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 16515;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::Low
        {
            return 16347;
        }
        if self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Tall
        {
            return 16506;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 16435;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 16400;
        }
        if self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::None
        {
            return 16367;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#east == East::None
        {
            return 16312;
        }
        if self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::None
        {
            return 16518;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#waterlogged == false
        {
            return 16320;
        }
        if self.r#up == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#south == South::Low
        {
            return 16563;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 16414;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#waterlogged == true
        {
            return 16610;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#east == East::Low
        {
            return 16493;
        }
        if self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 16339;
        }
        if self.r#up == true
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 16557;
        }
        if self.r#north == North::Low
            && self.r#west == West::None
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Tall
        {
            return 16574;
        }
        if self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#south == South::None
        {
            return 16584;
        }
        if self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::Low
        {
            return 16556;
        }
        if self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::Low
        {
            return 16455;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#north == North::Tall
        {
            return 16387;
        }
        if self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#west == West::Low
        {
            return 16542;
        }
        if self.r#up == false
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 16495;
        }
        if self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#up == false
        {
            return 16587;
        }
        if self.r#west == West::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 16559;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::None
        {
            return 16411;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 16419;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#south == South::Low
        {
            return 16522;
        }
        if self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 16496;
        }
        if self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#west == West::Tall
        {
            return 16576;
        }
        if self.r#up == true
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Low
        {
            return 16593;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == true
        {
            return 16384;
        }
        if self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 16330;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#east == East::Low
        {
            return 16466;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == false
        {
            return 16350;
        }
        if self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Tall
        {
            return 16371;
        }
        if self.r#south == South::Tall
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 16569;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 16372;
        }
        if self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#north == North::None
            && self.r#waterlogged == true
        {
            return 16540;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == false
        {
            return 16362;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 16497;
        }
        if self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 16507;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#east == East::Low
        {
            return 16490;
        }
        if self.r#west == West::None
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 16601;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#waterlogged == true
        {
            return 16329;
        }
        if self.r#south == South::None
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 16405;
        }
        if self.r#east == East::None
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::None
        {
            return 16309;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 16354;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::None
            && self.r#north == North::Tall
        {
            return 16604;
        }
        if self.r#up == true
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#south == South::Tall
        {
            return 16607;
        }
        if self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 16476;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 16511;
        }
        if self.r#up == true
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Tall
        {
            return 16465;
        }
        if self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Low
        {
            return 16470;
        }
        if self.r#west == West::None
            && self.r#up == true
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Tall
        {
            return 16532;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#north == North::Low
        {
            return 16548;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#south == South::Low
        {
            return 16564;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#up == true
        {
            return 16475;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Tall
        {
            return 16612;
        }
        if self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 16391;
        }
        if self.r#up == false
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#east == East::None
        {
            return 16298;
        }
        if self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#east == East::Tall
        {
            return 16535;
        }
        if self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 16474;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#up == false
            && self.r#north == North::Low
        {
            return 16562;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == true
        {
            return 16573;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 16311;
        }
        if self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::None
        {
            return 16337;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#waterlogged == true
        {
            return 16394;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#waterlogged == false
        {
            return 16523;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#west == West::None
            && self.r#north == North::Tall
        {
            return 16484;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::Tall
        {
            return 16430;
        }
        if self.r#up == false
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#waterlogged == true
        {
            return 16456;
        }
        if self.r#up == true
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#east == East::None
        {
            return 16318;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#east == East::Low
        {
            return 16457;
        }
        if self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 16422;
        }
        if self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#east == East::None
        {
            return 16346;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#east == East::Low
        {
            return 16487;
        }
        if self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 16321;
        }
        if self.r#up == true
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#west == West::Tall
        {
            return 16558;
        }
        if self.r#up == true
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#west == West::None
        {
            return 16592;
        }
        if self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#up == true
            && self.r#east == East::None
        {
            return 16296;
        }
        if self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 16316;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#south == South::Tall
        {
            return 16500;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#south == South::Tall
        {
            return 16609;
        }
        if self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#south == South::Tall
        {
            return 16505;
        }
        if self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::None
        {
            return 16448;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#up == true
            && self.r#north == North::Low
        {
            return 16353;
        }
        if self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::None
        {
            return 16413;
        }
        if self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 16395;
        }
        if self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::Low
        {
            return 16551;
        }
        if self.r#west == West::Low
            && self.r#up == false
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#east == East::Tall
        {
            return 16527;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == true
        {
            return 16332;
        }
        if self.r#east == East::None
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 16340;
        }
        if self.r#up == true
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
        {
            return 16317;
        }
        if self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 16483;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == false
        {
            return 16565;
        }
        if self.r#west == West::None
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 16577;
        }
        if self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 16590;
        }
        if self.r#east == East::Low
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#up == true
        {
            return 16439;
        }
        if self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#up == true
            && self.r#north == North::Low
        {
            return 16451;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 16530;
        }
        if self.r#west == West::None
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 16358;
        }
        if self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 16425;
        }
        if self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 16299;
        }
        if self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::None
        {
            return 16389;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#up == false
        {
            return 16433;
        }
        if self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 16464;
        }
        if self.r#up == true
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#north == North::None
        {
            return 16305;
        }
        if self.r#up == true
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#north == North::Low
        {
            return 16545;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 16579;
        }
        if self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#north == North::Low
        {
            return 16336;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 16426;
        }
        if self.r#east == East::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#north == North::None
        {
            return 16308;
        }
        if self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Tall
        {
            return 16502;
        }
        if self.r#north == North::Tall
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Low
        {
            return 16602;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::None
        {
            return 16374;
        }
        if self.r#west == West::Low
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
        {
            return 16386;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::None
        {
            return 16526;
        }
        if self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::None
            && self.r#south == South::Tall
        {
            return 16355;
        }
        if self.r#east == East::None
            && self.r#west == West::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#south == South::Tall
        {
            return 16397;
        }
        if self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#up == false
        {
            return 16478;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == true
        {
            return 16575;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::None
        {
            return 16373;
        }
        if self.r#up == false
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#south == South::None
        {
            return 16589;
        }
        if self.r#waterlogged == true
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 16588;
        }
        if self.r#west == West::None
            && self.r#up == false
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 16313;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#east == East::Low
        {
            return 16417;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#north == North::Tall
        {
            return 16595;
        }
        if self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#up == true
        {
            return 16403;
        }
        if self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 16429;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::None
        {
            return 16359;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#east == East::None
        {
            return 16351;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#north == North::None
        {
            return 16520;
        }
        if self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 16552;
        }
        if self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#west == West::None
        {
            return 16469;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#north == North::None
        {
            return 16519;
        }
        if self.r#up == true
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 16585;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 16446;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == true
        {
            return 16492;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#east == East::Low
        {
            return 16482;
        }
        if self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#north == North::Low
        {
            return 16566;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#west == West::None
        {
            return 16583;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::None
        {
            return 16536;
        }
        if self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#north == North::None
        {
            return 16543;
        }
        if self.r#west == West::Low
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#up == false
        {
            return 16479;
        }
        if self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#up == true
            && self.r#north == North::None
        {
            return 16401;
        }
        if self.r#north == North::None
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 16307;
        }
        if self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 16498;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == false
        {
            return 16382;
        }
        if self.r#waterlogged == false
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#east == East::Tall
        {
            return 16613;
        }
        if self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::None
        {
            return 16334;
        }
        if self.r#up == true
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#north == North::Low
        {
            return 16436;
        }
        if self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#south == South::None
            && self.r#north == North::Tall
        {
            return 16375;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 16534;
        }
        if self.r#up == true
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#south == South::Low
        {
            return 16378;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 16603;
        }
        if self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#south == South::None
        {
            return 16365;
        }
        if self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#west == West::None
        {
            return 16301;
        }
        if self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::None
        {
            return 16510;
        }
        if self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#east == East::Low
        {
            return 16458;
        }
        if self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#south == South::Low
        {
            return 16599;
        }
        if self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 16327;
        }
        if self.r#up == true
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#waterlogged == false
        {
            return 16441;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#west == West::Tall
        {
            return 16471;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::None
        {
            return 16531;
        }
        if self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#north == North::Low
        {
            return 16578;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#east == East::Low
        {
            return 16443;
        }
        if self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::None
        {
            return 16472;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#east == East::Low
        {
            return 16489;
        }
        if self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 16477;
        }
        if self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 16555;
        }
        if self.r#north == North::None
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 16402;
        }
        if self.r#west == West::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#east == East::None
            && self.r#south == South::Low
        {
            return 16343;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::None
        {
            return 16300;
        }
        if self.r#waterlogged == true
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#west == West::None
        {
            return 16454;
        }
        if self.r#west == West::Low
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 16356;
        }
        if self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == false
        {
            return 16303;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#south == South::Low
        {
            return 16421;
        }
        if self.r#up == true
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Tall
        {
            return 16461;
        }
        if self.r#south == South::None
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 16335;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#east == East::Low
        {
            return 16499;
        }
        if self.r#south == South::Low
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 16529;
        }
        if self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 16357;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
        {
            return 16549;
        }
        if self.r#south == South::Low
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 16379;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#east == East::Tall
        {
            return 16537;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#north == North::Low
        {
            return 16561;
        }
        if self.r#east == East::None
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Low
        {
            return 16314;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 16297;
        }
        if self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#north == North::None
            && self.r#west == West::Low
        {
            return 16512;
        }
        if self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#up == true
        {
            return 16508;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#up == true
        {
            return 16428;
        }
        if self.r#up == false
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Low
        {
            return 16361;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 16473;
        }
        if self.r#up == true
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#east == East::Low
        {
            return 16449;
        }
        if self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 16345;
        }
        if self.r#up == false
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#north == North::Tall
        {
            return 16591;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::None
        {
            return 16442;
        }
        if self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == true
        {
            return 16364;
        }
        if self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#north == North::None
            && self.r#waterlogged == true
        {
            return 16533;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::None
        {
            return 16550;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == true
        {
            return 16348;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::Tall
        {
            return 16538;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#west == West::Tall
        {
            return 16594;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 16407;
        }
        if self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::None
        {
            return 16331;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::Tall
        {
            return 16486;
        }
        if self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 16388;
        }
        if self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 16567;
        }
        if self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#west == West::None
        {
            return 16571;
        }
        if self.r#up == true
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == false
        {
            return 16596;
        }
        if self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 16376;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::None
        {
            return 16513;
        }
        if self.r#up == true
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == true
        {
            return 16606;
        }
        if self.r#up == false
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
        {
            return 16517;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 16420;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 16424;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#south == South::None
        {
            return 16546;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#up == false
        {
            return 16614;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 16580;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::None
        {
            return 16325;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::Tall
        {
            return 16539;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#north == North::Tall
        {
            return 16494;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 16412;
        }
        if self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#east == East::Low
        {
            return 16431;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#east == East::Low
        {
            return 16450;
        }
        if self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 16541;
        }
        if self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#north == North::Low
        {
            return 16352;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#up == false
        {
            return 16600;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Tall
        {
            return 16611;
        }
        if self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
        {
            return 16368;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#north == North::None
        {
            return 16521;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 16572;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 16437;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 16444;
        }
        if self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#east == East::None
            && self.r#west == West::Tall
        {
            return 16381;
        }
        if self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#south == South::Low
        {
            return 16415;
        }
        if self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#up == true
        {
            return 16292;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 16342;
        }
        if self.r#west == West::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#south == South::Tall
        {
            return 16322;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#south == South::None
        {
            return 16369;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#west == West::None
        {
            return 16304;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::Low
        {
            return 16440;
        }
        if self.r#up == false
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 16310;
        }
        if self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 16485;
        }
        if self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 16503;
        }
        if self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 16480;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == true
            && self.r#east == East::Tall
        {
            return 16544;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 16560;
        }
        if self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::None
        {
            return 16438;
        }
        if self.r#west == West::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#south == South::Tall
        {
            return 16390;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 16349;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#north == North::None
            && self.r#waterlogged == true
        {
            return 16406;
        }
        if self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#waterlogged == false
        {
            return 16453;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#up == true
            && self.r#south == South::None
            && self.r#east == East::Tall
        {
            return 16547;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#up == true
        {
            return 16295;
        }
        if self.r#west == West::Low
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Low
        {
            return 16338;
        }
        if self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::None
            && self.r#waterlogged == true
        {
            return 16586;
        }
        if self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#south == South::Tall
        {
            return 16396;
        }
        if self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::None
        {
            return 16326;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::None
        {
            return 16528;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#east == East::None
        {
            return 16399;
        }
        if self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#up == true
        {
            return 16380;
        }
        if self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 16392;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 16427;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#waterlogged == false
        {
            return 16416;
        }
        if self.r#north == North::None
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#south == South::None
        {
            return 16410;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#west == West::Low
        {
            return 16452;
        }
        if self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::Low
        {
            return 16459;
        }
        if self.r#south == South::None
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#up == true
        {
            return 16509;
        }
        if self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#south == South::Tall
        {
            return 16504;
        }
        if self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#up == false
        {
            return 16491;
        }
        if self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#up == false
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 16315;
        }
        if self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 16514;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 16409;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::None
        {
            return 16445;
        }
        if self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::None
        {
            return 16293;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == false
        {
            return 16447;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 16398;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 16467;
        }
        if self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#up == true
        {
            return 16328;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16605 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 16423 {
            return Some(BrickWall {
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 16581 {
            return Some(BrickWall {
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 16363 {
            return Some(BrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 16460 {
            return Some(BrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 16306 {
            return Some(BrickWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 16516 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 16598 {
            return Some(BrickWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 16323 {
            return Some(BrickWall {
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 16418 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 16383 {
            return Some(BrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 16570 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 16333 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 16463 {
            return Some(BrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 16377 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 16324 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 16393 {
            return Some(BrickWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 16404 {
            return Some(BrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 16554 {
            return Some(BrickWall {
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 16385 {
            return Some(BrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 16568 {
            return Some(BrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 16582 {
            return Some(BrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 16434 {
            return Some(BrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 16302 {
            return Some(BrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 16525 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 16344 {
            return Some(BrickWall {
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 16615 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 16597 {
            return Some(BrickWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 16319 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 16370 {
            return Some(BrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 16553 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 16360 {
            return Some(BrickWall {
                r#up: false,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 16366 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 16408 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 16481 {
            return Some(BrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 16341 {
            return Some(BrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 16468 {
            return Some(BrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16462 {
            return Some(BrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 16488 {
            return Some(BrickWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 16608 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 16501 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 16524 {
            return Some(BrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 16294 {
            return Some(BrickWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 16432 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 16515 {
            return Some(BrickWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 16347 {
            return Some(BrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 16506 {
            return Some(BrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 16435 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 16400 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 16367 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 16312 {
            return Some(BrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 16518 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 16320 {
            return Some(BrickWall {
                r#north: North::None,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16563 {
            return Some(BrickWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 16414 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16610 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 16493 {
            return Some(BrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 16339 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 16557 {
            return Some(BrickWall {
                r#up: true,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 16574 {
            return Some(BrickWall {
                r#north: North::Low,
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 16584 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 16556 {
            return Some(BrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16455 {
            return Some(BrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 16387 {
            return Some(BrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 16542 {
            return Some(BrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 16495 {
            return Some(BrickWall {
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 16587 {
            return Some(BrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 16559 {
            return Some(BrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 16411 {
            return Some(BrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 16419 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 16522 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16496 {
            return Some(BrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 16576 {
            return Some(BrickWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 16593 {
            return Some(BrickWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 16384 {
            return Some(BrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 16330 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16466 {
            return Some(BrickWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 16350 {
            return Some(BrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 16371 {
            return Some(BrickWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 16569 {
            return Some(BrickWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 16372 {
            return Some(BrickWall {
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16540 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 16362 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 16497 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 16507 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 16490 {
            return Some(BrickWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 16601 {
            return Some(BrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 16329 {
            return Some(BrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 16405 {
            return Some(BrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 16309 {
            return Some(BrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 16354 {
            return Some(BrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 16604 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 16607 {
            return Some(BrickWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 16476 {
            return Some(BrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 16511 {
            return Some(BrickWall {
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 16465 {
            return Some(BrickWall {
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 16470 {
            return Some(BrickWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 16532 {
            return Some(BrickWall {
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 16548 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 16564 {
            return Some(BrickWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16475 {
            return Some(BrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 16612 {
            return Some(BrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 16391 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 16298 {
            return Some(BrickWall {
                r#up: false,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 16535 {
            return Some(BrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 16474 {
            return Some(BrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16562 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 16573 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 16311 {
            return Some(BrickWall {
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 16337 {
            return Some(BrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 16394 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 16523 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 16484 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 16430 {
            return Some(BrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 16456 {
            return Some(BrickWall {
                r#up: false,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 16318 {
            return Some(BrickWall {
                r#up: true,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 16457 {
            return Some(BrickWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 16422 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 16346 {
            return Some(BrickWall {
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 16487 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 16321 {
            return Some(BrickWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 16558 {
            return Some(BrickWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 16592 {
            return Some(BrickWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 16296 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 16316 {
            return Some(BrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 16500 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 16609 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 16505 {
            return Some(BrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 16448 {
            return Some(BrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 16353 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 16413 {
            return Some(BrickWall {
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 16395 {
            return Some(BrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 16551 {
            return Some(BrickWall {
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 16527 {
            return Some(BrickWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 16332 {
            return Some(BrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 16340 {
            return Some(BrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 16317 {
            return Some(BrickWall {
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 16483 {
            return Some(BrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 16565 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 16577 {
            return Some(BrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 16590 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 16439 {
            return Some(BrickWall {
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 16451 {
            return Some(BrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 16530 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 16358 {
            return Some(BrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 16425 {
            return Some(BrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 16299 {
            return Some(BrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 16389 {
            return Some(BrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 16433 {
            return Some(BrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 16464 {
            return Some(BrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 16305 {
            return Some(BrickWall {
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 16545 {
            return Some(BrickWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 16579 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 16336 {
            return Some(BrickWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 16426 {
            return Some(BrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16308 {
            return Some(BrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 16502 {
            return Some(BrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 16602 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 16374 {
            return Some(BrickWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 16386 {
            return Some(BrickWall {
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 16526 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 16355 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 16397 {
            return Some(BrickWall {
                r#east: East::None,
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 16478 {
            return Some(BrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 16575 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 16373 {
            return Some(BrickWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 16589 {
            return Some(BrickWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 16588 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 16313 {
            return Some(BrickWall {
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16417 {
            return Some(BrickWall {
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 16595 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 16403 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 16429 {
            return Some(BrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 16359 {
            return Some(BrickWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 16351 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 16520 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 16552 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 16469 {
            return Some(BrickWall {
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 16519 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 16585 {
            return Some(BrickWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 16446 {
            return Some(BrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 16492 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 16482 {
            return Some(BrickWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 16566 {
            return Some(BrickWall {
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 16583 {
            return Some(BrickWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 16536 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 16543 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 16479 {
            return Some(BrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 16401 {
            return Some(BrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 16307 {
            return Some(BrickWall {
                r#north: North::None,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 16498 {
            return Some(BrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16382 {
            return Some(BrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 16613 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 16334 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 16436 {
            return Some(BrickWall {
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 16375 {
            return Some(BrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 16534 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16378 {
            return Some(BrickWall {
                r#up: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 16603 {
            return Some(BrickWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 16365 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 16301 {
            return Some(BrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 16510 {
            return Some(BrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 16458 {
            return Some(BrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 16599 {
            return Some(BrickWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16327 {
            return Some(BrickWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 16441 {
            return Some(BrickWall {
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16471 {
            return Some(BrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 16531 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 16578 {
            return Some(BrickWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 16443 {
            return Some(BrickWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 16472 {
            return Some(BrickWall {
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 16489 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 16477 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16555 {
            return Some(BrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 16402 {
            return Some(BrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 16343 {
            return Some(BrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 16300 {
            return Some(BrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 16454 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#up: false,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 16356 {
            return Some(BrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 16303 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 16421 {
            return Some(BrickWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 16461 {
            return Some(BrickWall {
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 16335 {
            return Some(BrickWall {
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 16499 {
            return Some(BrickWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 16529 {
            return Some(BrickWall {
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 16357 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16549 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 16379 {
            return Some(BrickWall {
                r#south: South::Low,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 16537 {
            return Some(BrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 16561 {
            return Some(BrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 16314 {
            return Some(BrickWall {
                r#east: East::None,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 16297 {
            return Some(BrickWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 16512 {
            return Some(BrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 16508 {
            return Some(BrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 16428 {
            return Some(BrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 16361 {
            return Some(BrickWall {
                r#up: false,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 16473 {
            return Some(BrickWall {
                r#south: South::None,
                r#up: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 16449 {
            return Some(BrickWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 16345 {
            return Some(BrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 16591 {
            return Some(BrickWall {
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 16442 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 16364 {
            return Some(BrickWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 16533 {
            return Some(BrickWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 16550 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 16348 {
            return Some(BrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 16538 {
            return Some(BrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 16594 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 16407 {
            return Some(BrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 16331 {
            return Some(BrickWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 16486 {
            return Some(BrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 16388 {
            return Some(BrickWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 16567 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16571 {
            return Some(BrickWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 16596 {
            return Some(BrickWall {
                r#up: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 16376 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 16513 {
            return Some(BrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 16606 {
            return Some(BrickWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 16517 {
            return Some(BrickWall {
                r#up: false,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 16420 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 16424 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 16546 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 16614 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 16580 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 16325 {
            return Some(BrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 16539 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 16494 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 16412 {
            return Some(BrickWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 16431 {
            return Some(BrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 16450 {
            return Some(BrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 16541 {
            return Some(BrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 16352 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 16600 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 16611 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 16368 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 16521 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 16572 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 16437 {
            return Some(BrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 16444 {
            return Some(BrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 16381 {
            return Some(BrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 16415 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 16292 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 16342 {
            return Some(BrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16322 {
            return Some(BrickWall {
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 16369 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 16304 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 16440 {
            return Some(BrickWall {
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 16310 {
            return Some(BrickWall {
                r#up: false,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 16485 {
            return Some(BrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 16503 {
            return Some(BrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 16480 {
            return Some(BrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16544 {
            return Some(BrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 16560 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 16438 {
            return Some(BrickWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 16390 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 16349 {
            return Some(BrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 16406 {
            return Some(BrickWall {
                r#south: South::None,
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 16453 {
            return Some(BrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 16547 {
            return Some(BrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 16295 {
            return Some(BrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 16338 {
            return Some(BrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 16586 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 16396 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 16326 {
            return Some(BrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 16528 {
            return Some(BrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 16399 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 16380 {
            return Some(BrickWall {
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 16392 {
            return Some(BrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 16427 {
            return Some(BrickWall {
                r#west: West::None,
                r#north: North::None,
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 16416 {
            return Some(BrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16410 {
            return Some(BrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 16452 {
            return Some(BrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 16459 {
            return Some(BrickWall {
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 16509 {
            return Some(BrickWall {
                r#south: South::None,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 16504 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 16491 {
            return Some(BrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 16315 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16514 {
            return Some(BrickWall {
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 16409 {
            return Some(BrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 16445 {
            return Some(BrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 16293 {
            return Some(BrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 16447 {
            return Some(BrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 16398 {
            return Some(BrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 16467 {
            return Some(BrickWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 16328 {
            return Some(BrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::None,
                r#up: true,
            });
        }
        return None;
    }
}
