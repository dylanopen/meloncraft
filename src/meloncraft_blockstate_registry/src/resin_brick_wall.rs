use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResinBrickWall {
    pub waterlogged: bool,
    pub r#east: East,
    pub r#south: South,
    pub r#north: North,
    pub r#west: West,
    pub up: bool,
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

impl BlockState for ResinBrickWall {
    fn to_id(&self) -> i32 {
        if self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 9095;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == true
            && self.r#south == South::Low
        {
            return 9037;
        }
        if self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 9054;
        }
        if self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#east == East::Tall
        {
            return 9043;
        }
        if self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#south == South::Tall
        {
            return 9013;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::None
        {
            return 8820;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#north == North::None
        {
            return 8828;
        }
        if self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 8855;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 8963;
        }
        if self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::Low
        {
            return 8985;
        }
        if self.r#up == false
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#waterlogged == false
        {
            return 9034;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::Low
        {
            return 8878;
        }
        if self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#up == true
        {
            return 8832;
        }
        if self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#waterlogged == false
        {
            return 8998;
        }
        if self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 9004;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::Tall
        {
            return 9045;
        }
        if self.r#up == true
            && self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 8822;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#west == West::None
        {
            return 9036;
        }
        if self.r#west == West::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#south == South::None
        {
            return 9062;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#north == North::Low
        {
            return 8952;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::None
        {
            return 8899;
        }
        if self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#north == North::None
            && self.r#waterlogged == false
        {
            return 9040;
        }
        if self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#up == false
        {
            return 8830;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#east == East::Low
        {
            return 8945;
        }
        if self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#west == West::None
            && self.r#south == South::Low
        {
            return 8856;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#north == North::Low
        {
            return 8986;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 8999;
        }
        if self.r#west == West::Low
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::Tall
        {
            return 8992;
        }
        if self.r#up == false
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 8817;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#waterlogged == false
        {
            return 9003;
        }
        if self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 9070;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#east == East::Low
        {
            return 8982;
        }
        if self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Low
        {
            return 9080;
        }
        if self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 8915;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#up == true
            && self.r#north == North::Low
        {
            return 8870;
        }
        if self.r#up == true
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#north == North::Tall
        {
            return 9016;
        }
        if self.r#north == North::Tall
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 9107;
        }
        if self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#up == false
        {
            return 9009;
        }
        if self.r#west == West::None
            && self.r#up == true
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#waterlogged == false
        {
            return 8811;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 8924;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#east == East::Tall
        {
            return 9068;
        }
        if self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 8984;
        }
        if self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#up == false
        {
            return 9018;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#west == West::Tall
        {
            return 8894;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#south == South::Low
        {
            return 9109;
        }
        if self.r#up == true
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#east == East::None
        {
            return 8884;
        }
        if self.r#up == true
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 9039;
        }
        if self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 8942;
        }
        if self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#south == South::Tall
        {
            return 9012;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#east == East::None
        {
            return 8867;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#east == East::None
        {
            return 8887;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::None
        {
            return 9026;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#east == East::Tall
        {
            return 9123;
        }
        if self.r#up == true
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 8991;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#up == true
        {
            return 9124;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 8846;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 9001;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 9020;
        }
        if self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#up == true
        {
            return 8990;
        }
        if self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::Tall
        {
            return 9008;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::None
        {
            return 8845;
        }
        if self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Tall
        {
            return 9113;
        }
        if self.r#up == true
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#west == West::Tall
        {
            return 8825;
        }
        if self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#north == North::None
            && self.r#south == South::Tall
        {
            return 8840;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 9041;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 8983;
        }
        if self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 9044;
        }
        if self.r#south == South::Tall
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#west == West::Tall
        {
            return 9050;
        }
        if self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Tall
        {
            return 8987;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#waterlogged == false
        {
            return 9075;
        }
        if self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == false
        {
            return 8818;
        }
        if self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#up == true
            && self.r#west == West::None
        {
            return 9027;
        }
        if self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#west == West::None
        {
            return 8907;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#west == West::Tall
        {
            return 8834;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 8919;
        }
        if self.r#west == West::Low
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 8974;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::Tall
        {
            return 9115;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#west == West::Low
        {
            return 8956;
        }
        if self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 8993;
        }
        if self.r#east == East::None
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#west == West::Low
        {
            return 8857;
        }
        if self.r#up == true
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#waterlogged == false
        {
            return 8955;
        }
        if self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#west == West::Tall
        {
            return 8957;
        }
        if self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#waterlogged == false
        {
            return 8843;
        }
        if self.r#south == South::Tall
            && self.r#up == true
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#waterlogged == false
        {
            return 8835;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 8852;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == false
        {
            return 8931;
        }
        if self.r#west == West::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#south == South::None
        {
            return 8847;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#up == true
        {
            return 9028;
        }
        if self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::None
            && self.r#east == East::Tall
        {
            return 9057;
        }
        if self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 9114;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::None
        {
            return 8895;
        }
        if self.r#west == West::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#north == North::Low
        {
            return 8968;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 8810;
        }
        if self.r#north == North::None
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#waterlogged == true
        {
            return 8935;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#up == true
        {
            return 8940;
        }
        if self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#north == North::None
        {
            return 8950;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::None
        {
            return 9035;
        }
        if self.r#up == true
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#west == West::Low
        {
            return 8881;
        }
        if self.r#up == true
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 9060;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 8966;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#up == false
        {
            return 9116;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == true
        {
            return 8860;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Low
        {
            return 9074;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#east == East::None
        {
            return 8888;
        }
        if self.r#north == North::None
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 8951;
        }
        if self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::None
        {
            return 8936;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#south == South::Low
        {
            return 9111;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 9089;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#up == true
        {
            return 9087;
        }
        if self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#east == East::None
        {
            return 8875;
        }
        if self.r#up == false
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::Tall
        {
            return 8997;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == true
        {
            return 9096;
        }
        if self.r#south == South::None
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 8890;
        }
        if self.r#up == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Tall
        {
            return 9086;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 9002;
        }
        if self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#west == West::None
        {
            return 8889;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::None
        {
            return 8859;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 8922;
        }
        if self.r#south == South::None
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#west == West::Tall
        {
            return 8921;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::None
        {
            return 8961;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 9000;
        }
        if self.r#south == South::Low
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#west == West::Tall
        {
            return 9038;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == true
        {
            return 9077;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#south == South::Tall
        {
            return 9122;
        }
        if self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Low
        {
            return 8977;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 8823;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#waterlogged == true
        {
            return 8833;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == true
        {
            return 8882;
        }
        if self.r#up == true
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::Tall
        {
            return 8905;
        }
        if self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::None
        {
            return 8918;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#west == West::None
        {
            return 8871;
        }
        if self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 9031;
        }
        if self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == false
        {
            return 9071;
        }
        if self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#north == North::Tall
        {
            return 9125;
        }
        if self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#north == North::None
        {
            return 9055;
        }
        if self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#up == false
        {
            return 9059;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#south == South::Low
        {
            return 9112;
        }
        if self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::None
        {
            return 9066;
        }
        if self.r#up == false
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Low
        {
            return 9047;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#north == North::Low
        {
            return 9079;
        }
        if self.r#south == South::Tall
            && self.r#up == false
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Low
        {
            return 8874;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#west == West::Tall
        {
            return 8891;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 8925;
        }
        if self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 8909;
        }
        if self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 8863;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 8937;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::None
        {
            return 8812;
        }
        if self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 8981;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Tall
        {
            return 8883;
        }
        if self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 9064;
        }
        if self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::None
        {
            return 9103;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 8900;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#south == South::Tall
        {
            return 9048;
        }
        if self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#up == false
        {
            return 8971;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::None
        {
            return 8849;
        }
        if self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::Tall
        {
            return 9118;
        }
        if self.r#up == false
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::None
        {
            return 8926;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 8927;
        }
        if self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::None
        {
            return 8838;
        }
        if self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#east == East::None
        {
            return 8910;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#east == East::Tall
        {
            return 9085;
        }
        if self.r#south == South::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Low
        {
            return 8850;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#south == South::Low
        {
            return 9117;
        }
        if self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 9082;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 8854;
        }
        if self.r#up == false
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Tall
        {
            return 9128;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == true
        {
            return 8954;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 8906;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#south == South::Tall
        {
            return 9023;
        }
        if self.r#up == true
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 9051;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#up == true
            && self.r#north == North::Low
        {
            return 9063;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::Tall
        {
            return 9014;
        }
        if self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#up == true
        {
            return 8917;
        }
        if self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#south == South::None
        {
            return 8958;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#west == West::Tall
        {
            return 8864;
        }
        if self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::None
        {
            return 9106;
        }
        if self.r#up == true
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::Low
        {
            return 9076;
        }
        if self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#up == true
        {
            return 8941;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#south == South::Low
        {
            return 9007;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 8877;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#west == West::None
        {
            return 8943;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#south == South::Low
        {
            return 8858;
        }
        if self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#north == North::Tall
        {
            return 9098;
        }
        if self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#up == false
        {
            return 8841;
        }
        if self.r#up == true
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 9017;
        }
        if self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 9030;
        }
        if self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#north == North::Low
        {
            return 9090;
        }
        if self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#waterlogged == true
        {
            return 8876;
        }
        if self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#south == South::Low
        {
            return 8932;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#east == East::Low
        {
            return 8965;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Tall
        {
            return 9084;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#north == North::Tall
        {
            return 8911;
        }
        if self.r#up == true
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#east == East::Low
        {
            return 8920;
        }
        if self.r#up == true
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::Low
        {
            return 8896;
        }
        if self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 9011;
        }
        if self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::Low
        {
            return 8821;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#east == East::None
        {
            return 8809;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
        {
            return 8897;
        }
        if self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 8947;
        }
        if self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 8885;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::Low
        {
            return 9073;
        }
        if self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 8868;
        }
        if self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#east == East::Low
        {
            return 8949;
        }
        if self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#waterlogged == true
        {
            return 9110;
        }
        if self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 8842;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#east == East::Low
        {
            return 9005;
        }
        if self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#east == East::Low
        {
            return 8959;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 8960;
        }
        if self.r#up == true
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 8861;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#south == South::Low
        {
            return 8892;
        }
        if self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#east == East::Tall
        {
            return 9091;
        }
        if self.r#west == West::None
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 9021;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#north == North::Low
        {
            return 9094;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 9121;
        }
        if self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 9010;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 8902;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#south == South::None
        {
            return 8962;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::None
        {
            return 9006;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 9065;
        }
        if self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 9120;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#south == South::Low
        {
            return 8933;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 9127;
        }
        if self.r#up == true
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#east == East::None
        {
            return 8872;
        }
        if self.r#west == West::None
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#north == North::Low
        {
            return 8976;
        }
        if self.r#up == true
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 9049;
        }
        if self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#east == East::None
        {
            return 8869;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#west == West::None
        {
            return 8970;
        }
        if self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::None
        {
            return 8934;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::Tall
        {
            return 8988;
        }
        if self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 8995;
        }
        if self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#up == false
        {
            return 8814;
        }
        if self.r#up == false
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#east == East::None
        {
            return 8912;
        }
        if self.r#west == West::Low
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 8815;
        }
        if self.r#south == South::Low
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#east == East::Low
        {
            return 8928;
        }
        if self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#up == false
        {
            return 8886;
        }
        if self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
        {
            return 9032;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Low
        {
            return 8824;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#south == South::Tall
        {
            return 9052;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::Low
        {
            return 8893;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#east == East::Low
        {
            return 8939;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::Tall
        {
            return 9053;
        }
        if self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 9033;
        }
        if self.r#east == East::Tall
            && self.r#up == false
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Tall
        {
            return 9056;
        }
        if self.r#west == West::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::Low
        {
            return 9069;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#east == East::Tall
        {
            return 9083;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Tall
        {
            return 9130;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 8914;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == true
        {
            return 9072;
        }
        if self.r#up == true
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::Tall
        {
            return 8813;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 8853;
        }
        if self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#up == true
        {
            return 9015;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::None
        {
            return 8836;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#up == false
            && self.r#north == North::Low
        {
            return 9067;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Tall
        {
            return 9058;
        }
        if self.r#up == false
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 9093;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::None
        {
            return 9097;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#waterlogged == false
        {
            return 9099;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#north == North::Low
        {
            return 8953;
        }
        if self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == false
        {
            return 9129;
        }
        if self.r#north == North::Low
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Low
        {
            return 8973;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#up == false
        {
            return 8996;
        }
        if self.r#up == true
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 8969;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#north == North::Low
        {
            return 9092;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#north == North::Low
        {
            return 8879;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 9024;
        }
        if self.r#up == true
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#waterlogged == false
        {
            return 8944;
        }
        if self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#east == East::Low
        {
            return 9019;
        }
        if self.r#east == East::None
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Low
        {
            return 8865;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#waterlogged == false
        {
            return 8980;
        }
        if self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Low
        {
            return 8903;
        }
        if self.r#west == West::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#south == South::Tall
        {
            return 9022;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#south == South::Low
        {
            return 9046;
        }
        if self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#west == West::Tall
        {
            return 8837;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#north == North::Low
        {
            return 8975;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#up == false
        {
            return 8946;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#up == false
        {
            return 8913;
        }
        if self.r#south == South::None
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 8851;
        }
        if self.r#west == West::None
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 8826;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::Low
        {
            return 8827;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#up == false
            && self.r#east == East::None
        {
            return 8831;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#waterlogged == false
        {
            return 9029;
        }
        if self.r#waterlogged == true
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::Low
        {
            return 9078;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::Tall
        {
            return 8873;
        }
        if self.r#north == North::Low
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 8866;
        }
        if self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 8948;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#up == false
        {
            return 9081;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#up == false
        {
            return 9104;
        }
        if self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::Tall
        {
            return 9105;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#north == North::Low
        {
            return 8862;
        }
        if self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#west == West::None
        {
            return 8898;
        }
        if self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == true
            && self.r#east == East::None
        {
            return 8904;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#up == true
        {
            return 8964;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 9131;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::Tall
        {
            return 9126;
        }
        if self.r#south == South::None
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 8848;
        }
        if self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Low
        {
            return 8972;
        }
        if self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#up == false
            && self.r#south == South::None
        {
            return 8819;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#up == true
        {
            return 8989;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#west == West::None
        {
            return 8967;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 8978;
        }
        if self.r#up == false
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == true
        {
            return 8994;
        }
        if self.r#up == true
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#west == West::Tall
        {
            return 9101;
        }
        if self.r#up == true
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#north == North::None
        {
            return 8930;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#up == true
            && self.r#east == East::None
        {
            return 8880;
        }
        if self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 8938;
        }
        if self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 8979;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 9042;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 8839;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#south == South::Low
        {
            return 8901;
        }
        if self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 9102;
        }
        if self.r#up == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 9088;
        }
        if self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#waterlogged == true
        {
            return 9108;
        }
        if self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 9100;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#up == false
        {
            return 8829;
        }
        if self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 9025;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 8923;
        }
        if self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 8929;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#west == West::Low
        {
            return 8908;
        }
        if self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#west == West::None
        {
            return 8916;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 8808;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 9061;
        }
        if self.r#north == North::Low
            && self.r#west == West::None
            && self.r#up == true
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 8844;
        }
        if self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#east == East::Tall
        {
            return 9119;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 8816;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9095 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 9037 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 9054 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 9043 {
            return Some(ResinBrickWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 9013 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 8820 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 8828 {
            return Some(ResinBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 8855 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 8963 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 8985 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 9034 {
            return Some(ResinBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 8878 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 8832 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 8998 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 9004 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 9045 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 8822 {
            return Some(ResinBrickWall {
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 9036 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 9062 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 8952 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 8899 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 9040 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 8830 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 8945 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 8856 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 8986 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 8999 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 8992 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 8817 {
            return Some(ResinBrickWall {
                r#up: false,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 9003 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 9070 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 8982 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 9080 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 8915 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 8870 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 9016 {
            return Some(ResinBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 9107 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 9009 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 8811 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#up: true,
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 8924 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 9068 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 8984 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 9018 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 8894 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 9109 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 8884 {
            return Some(ResinBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 9039 {
            return Some(ResinBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 8942 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 9012 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 8867 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 8887 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 9026 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 9123 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 8991 {
            return Some(ResinBrickWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 9124 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 8846 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 9001 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 9020 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8990 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 9008 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 8845 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 9113 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 8825 {
            return Some(ResinBrickWall {
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 8840 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 9041 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 8983 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 9044 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 9050 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 8987 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 9075 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 8818 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 9027 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 8907 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 8834 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 8919 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 8974 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 9115 {
            return Some(ResinBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 8956 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 8993 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 8857 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 8955 {
            return Some(ResinBrickWall {
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 8957 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 8843 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 8835 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 8852 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 8931 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 8847 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 9028 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 9057 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 9114 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8895 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 8968 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 8810 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 8935 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 8940 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 8950 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 9035 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 8881 {
            return Some(ResinBrickWall {
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 9060 {
            return Some(ResinBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 8966 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 9116 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 8860 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 9074 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 8888 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 8951 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 8936 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 9111 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 9089 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 9087 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 8875 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 8997 {
            return Some(ResinBrickWall {
                r#up: false,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 9096 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 8890 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 9086 {
            return Some(ResinBrickWall {
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 9002 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 8889 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 8859 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 8922 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 8921 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 8961 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 9000 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 9038 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 9077 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 9122 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 8977 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 8823 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8833 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 8882 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 8905 {
            return Some(ResinBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 8918 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 8871 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 9031 {
            return Some(ResinBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 9071 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 9125 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 9055 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 9059 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 9112 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 9066 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 9047 {
            return Some(ResinBrickWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 9079 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 8874 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 8891 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 8925 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 8909 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 8863 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8937 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 8812 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 8981 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 8883 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 9064 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 9103 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 8900 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 9048 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 8971 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 8849 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 9118 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 8926 {
            return Some(ResinBrickWall {
                r#up: false,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 8927 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 8838 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 8910 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 9085 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 8850 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 9117 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 9082 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 8854 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 9128 {
            return Some(ResinBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 8954 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 8906 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 9023 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 9051 {
            return Some(ResinBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 9063 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 9014 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 8917 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 8958 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 8864 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 9106 {
            return Some(ResinBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 9076 {
            return Some(ResinBrickWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 8941 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 9007 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 8877 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 8943 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 8858 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 9098 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 8841 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 9017 {
            return Some(ResinBrickWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 9030 {
            return Some(ResinBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 9090 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 8876 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 8932 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 8965 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 9084 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 8911 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 8920 {
            return Some(ResinBrickWall {
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 8896 {
            return Some(ResinBrickWall {
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 9011 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 8821 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 8809 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 8897 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 8947 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8885 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 9073 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 8868 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 8949 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 9110 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 8842 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 9005 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 8959 {
            return Some(ResinBrickWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 8960 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 8861 {
            return Some(ResinBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 8892 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 9091 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 9021 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 9094 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 9121 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 9010 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 8902 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 8962 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 9006 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 9065 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 9120 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 8933 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 9127 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 8872 {
            return Some(ResinBrickWall {
                r#up: true,
                r#north: North::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 8976 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 9049 {
            return Some(ResinBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 8869 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 8970 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 8934 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 8988 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 8995 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8814 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 8912 {
            return Some(ResinBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 8815 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 8928 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 8886 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 9032 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 8824 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 9052 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 8893 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 8939 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 9053 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 9033 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 9056 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 9069 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 9083 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 9130 {
            return Some(ResinBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 8914 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 9072 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 8813 {
            return Some(ResinBrickWall {
                r#up: true,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 8853 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 9015 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 8836 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 9067 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 9058 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 9093 {
            return Some(ResinBrickWall {
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 9097 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 9099 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 8953 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 9129 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 8973 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 8996 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 8969 {
            return Some(ResinBrickWall {
                r#up: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 9092 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 8879 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 9024 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 8944 {
            return Some(ResinBrickWall {
                r#up: true,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 9019 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 8865 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 8980 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 8903 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 9022 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 9046 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 8837 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 8975 {
            return Some(ResinBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 8946 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 8913 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 8851 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8826 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8827 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 8831 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 9029 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 9078 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 8873 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 8866 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 8948 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 9081 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 9104 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 9105 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 8862 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 8898 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 8904 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 8964 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 9131 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 9126 {
            return Some(ResinBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 8848 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8972 {
            return Some(ResinBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 8819 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 8989 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 8967 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 8978 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 8994 {
            return Some(ResinBrickWall {
                r#up: false,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 9101 {
            return Some(ResinBrickWall {
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 8930 {
            return Some(ResinBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 8880 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 8938 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 8979 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 9042 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8839 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 8901 {
            return Some(ResinBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 9102 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 9088 {
            return Some(ResinBrickWall {
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 9108 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 9100 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8829 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 9025 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 8923 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 8929 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 8908 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 8916 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 8808 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 9061 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 8844 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#west: West::None,
                r#up: true,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 9119 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 8816 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
            });
        }
        return None;
    }
}
