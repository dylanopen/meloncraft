use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedstoneWire {
    pub r#south: South,
    pub r#east: East,
    pub r#west: West,
    pub power: i32,
    pub r#north: North,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum South {
    Up,
    Side,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum East {
    Up,
    Side,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum West {
    Up,
    Side,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum North {
    Up,
    Side,
    None,
}

impl BlockState for RedstoneWire {
    fn to_id(&self) -> i32 {
        if self.r#east == East::Side
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#power == 4
        {
            return 4572;
        }
        if self.r#power == 1
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#south == South::Up
        {
            return 4252;
        }
        if self.r#west == West::None
            && self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#power == 0
        {
            return 4247;
        }
        if self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#power == 7
        {
            return 4739;
        }
        if self.r#north == North::Up
            && self.r#power == 5
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::Side
        {
            return 4295;
        }
        if self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#power == 4
            && self.r#south == South::Side
        {
            return 4281;
        }
        if self.r#west == West::None
            && self.r#south == South::Side
            && self.r#power == 0
            && self.r#east == East::Side
            && self.r#north == North::Side
        {
            return 4391;
        }
        if self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#east == East::None
            && self.r#power == 9
        {
            return 4759;
        }
        if self.r#west == West::Up
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 1
            && self.r#south == South::Side
        {
            return 4830;
        }
        if self.r#east == East::Up
            && self.r#south == South::None
            && self.r#power == 0
            && self.r#west == West::Side
            && self.r#north == North::Up
        {
            return 3817;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#power == 11
            && self.r#west == West::Side
        {
            return 5068;
        }
        if self.r#east == East::Up
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#south == South::None
            && self.r#power == 3
        {
            return 4131;
        }
        if self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 12
            && self.r#south == South::None
            && self.r#west == West::Up
        {
            return 4932;
        }
        if self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#power == 14
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 4231;
        }
        if self.r#power == 3
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#north == North::None
        {
            return 4129;
        }
        if self.r#power == 11
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Up
        {
            return 4781;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#power == 0
            && self.r#south == South::Side
            && self.r#west == West::Side
        {
            return 4966;
        }
        if self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#power == 10
        {
            return 4623;
        }
        if self.r#west == West::Side
            && self.r#south == South::Side
            && self.r#east == East::None
            && self.r#power == 4
            && self.r#north == North::Side
        {
            return 4858;
        }
        if self.r#power == 4
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#east == East::None
        {
            return 5004;
        }
        if self.r#power == 12
            && self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#west == West::Side
        {
            return 3925;
        }
        if self.r#south == South::None
            && self.r#power == 14
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 5094;
        }
        if self.r#power == 15
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#east == East::None
        {
            return 5100;
        }
        if self.r#east == East::Side
            && self.r#power == 1
            && self.r#north == North::Side
            && self.r#west == West::Side
            && self.r#south == South::None
        {
            return 4402;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::Up
            && self.r#power == 5
            && self.r#west == West::Side
        {
            return 4150;
        }
        if self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#power == 2
            && self.r#east == East::Side
            && self.r#west == West::Side
        {
            return 4408;
        }
        if self.r#south == South::Up
            && self.r#power == 10
            && self.r#west == West::None
            && self.r#east == East::Side
            && self.r#north == North::None
        {
            return 4622;
        }
        if self.r#west == West::Side
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#power == 15
            && self.r#south == South::Side
        {
            return 4237;
        }
        if self.r#power == 0
            && self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 4820;
        }
        if self.r#north == North::Side
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#power == 5
            && self.r#east == East::Side
        {
            return 4439;
        }
        if self.r#power == 5
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#south == South::Side
        {
            return 4866;
        }
        if self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#west == West::None
            && self.r#power == 13
            && self.r#south == South::None
        {
            return 4511;
        }
        if self.r#south == South::Side
            && self.r#east == East::None
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#power == 5
        {
            return 4723;
        }
        if self.r#south == South::None
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#east == East::Up
            && self.r#power == 2
        {
            return 3980;
        }
        if self.r#east == East::Up
            && self.r#south == South::None
            && self.r#west == West::Side
            && self.r#power == 2
            && self.r#north == North::None
        {
            return 4123;
        }
        if self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#power == 9
            && self.r#west == West::Side
            && self.r#south == South::Side
        {
            return 4471;
        }
        if self.r#west == West::Side
            && self.r#power == 5
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 4582;
        }
        if self.r#east == East::None
            && self.r#south == South::Side
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#power == 10
        {
            return 4913;
        }
        if self.r#north == North::None
            && self.r#power == 8
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#west == West::Side
        {
            return 5038;
        }
        if self.r#south == South::Side
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 12
            && self.r#west == West::None
        {
            return 4931;
        }
        if self.r#power == 11
            && self.r#south == South::None
            && self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#west == West::Side
        {
            return 3916;
        }
        if self.r#north == North::Side
            && self.r#power == 1
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#west == West::Side
        {
            return 3967;
        }
        if self.r#south == South::None
            && self.r#east == East::Up
            && self.r#power == 0
            && self.r#west == West::Up
            && self.r#north == North::Up
        {
            return 3816;
        }
        if self.r#east == East::Up
            && self.r#power == 1
            && self.r#north == North::Up
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 3820;
        }
        if self.r#power == 4
            && self.r#east == East::None
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#north == North::Up
        {
            return 4712;
        }
        if self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#power == 10
        {
            return 4191;
        }
        if self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#power == 15
        {
            return 3949;
        }
        if self.r#south == South::None
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#power == 15
        {
            return 5103;
        }
        if self.r#east == East::Up
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#power == 10
        {
            return 3904;
        }
        if self.r#west == West::None
            && self.r#east == East::Up
            && self.r#north == North::None
            && self.r#power == 4
            && self.r#south == South::Up
        {
            return 4136;
        }
        if self.r#north == North::Side
            && self.r#power == 10
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#west == West::Side
        {
            return 4915;
        }
        if self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#east == East::None
            && self.r#power == 1
            && self.r#south == South::Up
        {
            return 4827;
        }
        if self.r#power == 8
            && self.r#east == East::Side
            && self.r#west == West::None
            && self.r#south == South::Side
            && self.r#north == North::Up
        {
            return 4319;
        }
        if self.r#east == East::None
            && self.r#west == West::Side
            && self.r#power == 2
            && self.r#north == North::Up
            && self.r#south == South::Up
        {
            return 4693;
        }
        if self.r#east == East::Up
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#power == 1
        {
            return 4113;
        }
        if self.r#power == 1
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#north == North::Up
        {
            return 4683;
        }
        if self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#power == 5
            && self.r#south == South::Up
        {
            return 3856;
        }
        if self.r#power == 3
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#east == East::Up
            && self.r#south == South::None
        {
            return 3845;
        }
        if self.r#power == 7
            && self.r#west == West::None
            && self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#south == South::None
        {
            return 4313;
        }
        if self.r#east == East::Up
            && self.r#power == 4
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 3849;
        }
        if self.r#west == West::Up
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#power == 0
            && self.r#east == East::Side
        {
            return 4536;
        }
        if self.r#power == 2
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#west == West::Up
        {
            return 4548;
        }
        if self.r#east == East::Up
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#power == 9
            && self.r#north == North::Up
        {
            return 3897;
        }
        if self.r#power == 5
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#north == North::Up
        {
            return 3855;
        }
        if self.r#east == East::None
            && self.r#west == West::Up
            && self.r#power == 14
            && self.r#north == North::Side
            && self.r#south == South::Side
        {
            return 4947;
        }
        if self.r#north == North::None
            && self.r#south == South::Up
            && self.r#power == 1
            && self.r#east == East::Up
            && self.r#west == West::Side
        {
            return 4108;
        }
        if self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 3
            && self.r#south == South::None
            && self.r#west == West::Up
        {
            return 4419;
        }
        if self.r#east == East::Up
            && self.r#west == West::Up
            && self.r#south == South::Up
            && self.r#power == 12
            && self.r#north == North::Up
        {
            return 3918;
        }
        if self.r#north == North::Side
            && self.r#power == 9
            && self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#south == South::None
        {
            return 4041;
        }
        if self.r#power == 6
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#north == North::None
        {
            return 4154;
        }
        if self.r#north == North::None
            && self.r#south == South::Side
            && self.r#power == 2
            && self.r#east == East::Up
            && self.r#west == West::Up
        {
            return 4119;
        }
        if self.r#north == North::None
            && self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#power == 14
            && self.r#west == West::None
        {
            return 4658;
        }
        if self.r#south == South::None
            && self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#power == 6
            && self.r#west == West::Side
        {
            return 3871;
        }
        if self.r#west == West::None
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 9
            && self.r#south == South::Up
        {
            return 4469;
        }
        if self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#power == 12
            && self.r#north == North::None
            && self.r#south == South::Up
        {
            return 4639;
        }
        if self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 1
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 4397;
        }
        if self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 3
            && self.r#south == South::Up
        {
            return 4847;
        }
        if self.r#east == East::Side
            && self.r#south == South::None
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#power == 2
        {
            return 4267;
        }
        if self.r#power == 15
            && self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#north == North::Side
        {
            return 4096;
        }
        if self.r#north == North::Side
            && self.r#power == 11
            && self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 4486;
        }
        if self.r#north == North::Up
            && self.r#power == 0
            && self.r#south == South::None
            && self.r#east == East::Side
            && self.r#west == West::None
        {
            return 4250;
        }
        if self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#power == 5
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 4432;
        }
        if self.r#north == North::Side
            && self.r#west == West::None
            && self.r#east == East::Up
            && self.r#power == 9
            && self.r#south == South::Up
        {
            return 4037;
        }
        if self.r#west == West::Side
            && self.r#power == 1
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#south == South::Side
        {
            return 4543;
        }
        if self.r#south == South::None
            && self.r#power == 3
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#west == West::None
        {
            return 4421;
        }
        if self.r#power == 3
            && self.r#south == South::Up
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#west == West::Side
        {
            return 4702;
        }
        if self.r#power == 5
            && self.r#east == East::Up
            && self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#south == South::None
        {
            return 3861;
        }
        if self.r#power == 7
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 4169;
        }
        if self.r#south == South::Side
            && self.r#west == West::None
            && self.r#power == 2
            && self.r#north == North::Side
            && self.r#east == East::None
        {
            return 4841;
        }
        if self.r#south == South::Up
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 7
        {
            return 4883;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#power == 5
            && self.r#south == South::Side
            && self.r#west == West::Side
        {
            return 5011;
        }
        if self.r#north == North::None
            && self.r#south == South::Up
            && self.r#power == 7
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 5027;
        }
        if self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#power == 2
            && self.r#south == South::None
        {
            return 4268;
        }
        if self.r#north == North::None
            && self.r#west == West::None
            && self.r#power == 14
            && self.r#south == South::Side
            && self.r#east == East::None
        {
            return 5093;
        }
        if self.r#east == East::None
            && self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#power == 10
            && self.r#west == West::Side
        {
            return 4768;
        }
        if self.r#north == North::None
            && self.r#power == 12
            && self.r#east == East::None
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 5072;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#power == 14
            && self.r#east == East::None
        {
            return 5096;
        }
        if self.r#west == West::Up
            && self.r#power == 12
            && self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#south == South::Side
        {
            return 3921;
        }
        if self.r#east == East::Side
            && self.r#south == South::None
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#power == 12
        {
            return 4358;
        }
        if self.r#west == West::None
            && self.r#power == 10
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#south == South::None
        {
            return 4628;
        }
        if self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#power == 7
        {
            return 4452;
        }
        if self.r#south == South::Up
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#power == 8
            && self.r#west == West::Side
        {
            return 5035;
        }
        if self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#power == 8
            && self.r#north == North::Side
            && self.r#west == West::Side
        {
            return 4030;
        }
        if self.r#west == West::Side
            && self.r#south == South::Up
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#power == 7
        {
            return 4162;
        }
        if self.r#power == 5
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#north == North::Side
        {
            return 4437;
        }
        if self.r#west == West::Up
            && self.r#power == 9
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::None
        {
            return 5049;
        }
        if self.r#east == East::None
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#power == 4
            && self.r#west == West::None
        {
            return 4856;
        }
        if self.r#south == South::Side
            && self.r#west == West::None
            && self.r#north == North::Up
            && self.r#east == East::None
            && self.r#power == 3
        {
            return 4706;
        }
        if self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#west == West::None
            && self.r#power == 0
            && self.r#north == North::None
        {
            return 4103;
        }
        if self.r#power == 6
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#west == West::Up
            && self.r#south == South::Side
        {
            return 4587;
        }
        if self.r#east == East::Up
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#power == 1
            && self.r#south == South::Side
        {
            return 3823;
        }
        if self.r#power == 4
            && self.r#west == West::Up
            && self.r#south == South::Side
            && self.r#north == North::Side
            && self.r#east == East::Up
        {
            return 3993;
        }
        if self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#power == 9
        {
            return 4184;
        }
        if self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#power == 15
        {
            return 4238;
        }
        if self.r#power == 5
            && self.r#north == North::None
            && self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#west == West::Side
        {
            return 4147;
        }
        if self.r#east == East::Side
            && self.r#west == West::None
            && self.r#north == North::Side
            && self.r#power == 14
            && self.r#south == South::None
        {
            return 4520;
        }
        if self.r#east == East::None
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#power == 6
        {
            return 4729;
        }
        if self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#power == 8
            && self.r#east == East::None
        {
            return 4891;
        }
        if self.r#north == North::Up
            && self.r#power == 8
            && self.r#west == West::Up
            && self.r#south == South::Up
            && self.r#east == East::None
        {
            return 4746;
        }
        if self.r#power == 12
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#east == East::None
        {
            return 4929;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#power == 12
            && self.r#north == North::Side
            && self.r#east == East::None
        {
            return 4934;
        }
        if self.r#south == South::Up
            && self.r#power == 15
            && self.r#north == North::Side
            && self.r#east == East::None
            && self.r#west == West::Side
        {
            return 4954;
        }
        if self.r#north == North::Up
            && self.r#east == East::None
            && self.r#power == 8
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 4754;
        }
        if self.r#power == 8
            && self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 4175;
        }
        if self.r#north == North::Side
            && self.r#power == 5
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::Up
        {
            return 4007;
        }
        if self.r#west == West::None
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#power == 5
            && self.r#south == South::Side
        {
            return 3860;
        }
        if self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#south == South::None
            && self.r#power == 11
            && self.r#east == East::Side
        {
            return 4347;
        }
        if self.r#south == South::None
            && self.r#power == 11
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#north == North::Side
        {
            return 4492;
        }
        if self.r#power == 15
            && self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#east == East::None
        {
            return 4953;
        }
        if self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#power == 5
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 4433;
        }
        if self.r#west == West::Side
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#power == 3
        {
            return 4990;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#power == 10
        {
            return 4625;
        }
        if self.r#north == North::Side
            && self.r#power == 8
            && self.r#south == South::Up
            && self.r#east == East::Up
            && self.r#west == West::Side
        {
            return 4027;
        }
        if self.r#north == North::None
            && self.r#power == 6
            && self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#west == West::None
        {
            return 4589;
        }
        if self.r#north == North::Up
            && self.r#power == 5
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#east == East::Side
        {
            return 4289;
        }
        if self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#power == 13
            && self.r#east == East::Up
            && self.r#west == West::None
        {
            return 3929;
        }
        if self.r#east == East::Side
            && self.r#power == 14
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::Side
        {
            return 4661;
        }
        if self.r#power == 4
            && self.r#north == North::Up
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 4713;
        }
        if self.r#west == West::Side
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#power == 13
        {
            return 4654;
        }
        if self.r#power == 14
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#north == North::Side
        {
            return 4950;
        }
        if self.r#north == North::Side
            && self.r#power == 0
            && self.r#south == South::Side
            && self.r#west == West::None
            && self.r#east == East::Up
        {
            return 3959;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#south == South::Side
            && self.r#power == 8
        {
            return 5037;
        }
        if self.r#north == North::None
            && self.r#south == South::Side
            && self.r#east == East::None
            && self.r#power == 8
            && self.r#west == West::None
        {
            return 5039;
        }
        if self.r#east == East::Side
            && self.r#power == 2
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::Side
        {
            return 4555;
        }
        if self.r#south == South::None
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#power == 13
        {
            return 4943;
        }
        if self.r#power == 10
            && self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 4045;
        }
        if self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#power == 3
        {
            return 4993;
        }
        if self.r#power == 3
            && self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#north == North::Side
        {
            return 3984;
        }
        if self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#south == South::None
            && self.r#power == 0
            && self.r#north == North::Side
        {
            return 4393;
        }
        if self.r#power == 7
            && self.r#west == West::Side
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::Side
        {
            return 4888;
        }
        if self.r#power == 11
            && self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#south == South::None
        {
            return 4059;
        }
        if self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#power == 4
            && self.r#west == West::Side
            && self.r#south == South::Side
        {
            return 3994;
        }
        if self.r#west == West::Side
            && self.r#south == South::Up
            && self.r#power == 12
            && self.r#north == North::Side
            && self.r#east == East::Up
        {
            return 4063;
        }
        if self.r#power == 1
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#north == North::Side
        {
            return 3963;
        }
        if self.r#east == East::None
            && self.r#west == West::Side
            && self.r#power == 0
            && self.r#north == North::Side
            && self.r#south == South::Up
        {
            return 4819;
        }
        if self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#power == 15
        {
            return 4810;
        }
        if self.r#east == East::None
            && self.r#north == North::Up
            && self.r#power == 14
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 4803;
        }
        if self.r#south == South::None
            && self.r#power == 8
            && self.r#north == North::Up
            && self.r#west == West::Side
            && self.r#east == East::Up
        {
            return 3889;
        }
        if self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#power == 0
            && self.r#east == East::Up
            && self.r#south == South::Side
        {
            return 3957;
        }
        if self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#power == 15
            && self.r#east == East::Up
        {
            return 4091;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#power == 1
            && self.r#south == South::None
            && self.r#east == East::Up
        {
            return 4115;
        }
        if self.r#power == 7
            && self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 4595;
        }
        if self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#power == 14
            && self.r#west == West::None
            && self.r#south == South::Up
        {
            return 3938;
        }
        if self.r#power == 9
            && self.r#north == North::Side
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::Side
        {
            return 4904;
        }
        if self.r#east == East::Side
            && self.r#power == 7
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 4601;
        }
        if self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#power == 15
            && self.r#south == South::Side
            && self.r#east == East::None
        {
            return 4957;
        }
        if self.r#south == South::Side
            && self.r#west == West::None
            && self.r#power == 6
            && self.r#east == East::Up
            && self.r#north == North::Side
        {
            return 4013;
        }
        if self.r#power == 2
            && self.r#south == South::None
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#west == West::None
        {
            return 3836;
        }
        if self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#power == 5
            && self.r#west == West::Side
            && self.r#south == South::None
        {
            return 3862;
        }
        if self.r#east == East::None
            && self.r#power == 12
            && self.r#south == South::Side
            && self.r#north == North::None
            && self.r#west == West::Up
        {
            return 5073;
        }
        if self.r#power == 6
            && self.r#south == South::Side
            && self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#west == West::Side
        {
            return 4444;
        }
        if self.r#west == West::None
            && self.r#power == 7
            && self.r#north == North::Up
            && self.r#south == South::None
            && self.r#east == East::Up
        {
            return 3881;
        }
        if self.r#west == West::None
            && self.r#power == 2
            && self.r#east == East::Side
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 4556;
        }
        if self.r#north == North::Side
            && self.r#power == 4
            && self.r#west == West::Side
            && self.r#south == South::Up
            && self.r#east == East::Up
        {
            return 3991;
        }
        if self.r#north == North::Up
            && self.r#power == 13
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#east == East::None
        {
            return 4797;
        }
        if self.r#south == South::None
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 6
        {
            return 4447;
        }
        if self.r#south == South::None
            && self.r#east == East::Side
            && self.r#west == West::Up
            && self.r#power == 2
            && self.r#north == North::Up
        {
            return 4266;
        }
        if self.r#north == North::None
            && self.r#power == 1
            && self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#west == West::None
        {
            return 4112;
        }
        if self.r#power == 15
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#south == South::Side
        {
            return 4814;
        }
        if self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#power == 8
        {
            return 4314;
        }
        if self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 8
            && self.r#south == South::Up
            && self.r#west == West::Up
        {
            return 4458;
        }
        if self.r#east == East::None
            && self.r#power == 4
            && self.r#west == West::Up
            && self.r#south == South::Up
            && self.r#north == North::Side
        {
            return 4854;
        }
        if self.r#power == 9
            && self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#west == West::Up
        {
            return 4323;
        }
        if self.r#south == South::None
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#power == 7
            && self.r#west == West::Up
        {
            return 4311;
        }
        if self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#west == West::Side
            && self.r#power == 3
        {
            return 3841;
        }
        if self.r#west == West::None
            && self.r#power == 2
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#south == South::Side
        {
            return 4553;
        }
        if self.r#south == South::Up
            && self.r#east == East::Up
            && self.r#north == North::None
            && self.r#power == 12
            && self.r#west == West::Side
        {
            return 4207;
        }
        if self.r#east == East::Side
            && self.r#power == 13
            && self.r#north == North::Side
            && self.r#west == West::Side
            && self.r#south == South::Side
        {
            return 4507;
        }
        if self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#power == 5
        {
            return 4434;
        }
        if self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#power == 6
            && self.r#east == East::None
        {
            return 4873;
        }
        if self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#north == North::None
            && self.r#power == 3
        {
            return 4560;
        }
        if self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#power == 13
        {
            return 4649;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#power == 9
            && self.r#west == West::Side
            && self.r#south == South::Side
        {
            return 5047;
        }
        if self.r#south == South::Side
            && self.r#north == North::None
            && self.r#power == 13
            && self.r#east == East::Up
            && self.r#west == West::None
        {
            return 4220;
        }
        if self.r#power == 12
            && self.r#south == South::None
            && self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#west == West::Up
        {
            return 4068;
        }
        if self.r#power == 2
            && self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 3973;
        }
        if self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#power == 2
            && self.r#north == North::Up
        {
            return 3828;
        }
        if self.r#north == North::Side
            && self.r#power == 6
            && self.r#south == South::None
            && self.r#east == East::Up
            && self.r#west == West::Up
        {
            return 4014;
        }
        if self.r#south == South::Side
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#power == 0
        {
            return 4677;
        }
        if self.r#east == East::None
            && self.r#north == North::Up
            && self.r#power == 5
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 4720;
        }
        if self.r#north == North::Up
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::Up
            && self.r#power == 10
        {
            return 3908;
        }
        if self.r#east == East::Up
            && self.r#power == 4
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 4142;
        }
        if self.r#north == North::None
            && self.r#power == 1
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#east == East::Side
        {
            return 4540;
        }
        if self.r#north == North::Up
            && self.r#west == West::Side
            && self.r#south == South::Side
            && self.r#east == East::None
            && self.r#power == 13
        {
            return 4795;
        }
        if self.r#east == East::None
            && self.r#west == West::Up
            && self.r#power == 12
            && self.r#north == North::Up
            && self.r#south == South::Up
        {
            return 4782;
        }
        if self.r#north == North::None
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#power == 15
            && self.r#east == East::None
        {
            return 5101;
        }
        if self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#east == East::None
            && self.r#power == 14
            && self.r#south == South::None
        {
            return 4807;
        }
        if self.r#south == South::Side
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#power == 15
            && self.r#east == East::None
        {
            return 5102;
        }
        if self.r#west == West::None
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#power == 11
            && self.r#south == South::None
        {
            return 3917;
        }
        if self.r#south == South::None
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#east == East::None
            && self.r#power == 4
        {
            return 4717;
        }
        if self.r#east == East::Side
            && self.r#power == 11
            && self.r#west == West::None
            && self.r#north == North::Up
            && self.r#south == South::None
        {
            return 4349;
        }
        if self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#power == 13
        {
            return 4650;
        }
        if self.r#south == South::None
            && self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#power == 6
        {
            return 4446;
        }
        if self.r#power == 2
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#north == North::None
        {
            return 4988;
        }
        if self.r#south == South::Side
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#power == 7
            && self.r#east == East::None
        {
            return 5030;
        }
        if self.r#power == 7
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#north == North::Up
        {
            return 3873;
        }
        if self.r#east == East::None
            && self.r#power == 4
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::Side
        {
            return 5005;
        }
        if self.r#west == West::None
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#power == 5
            && self.r#south == South::None
        {
            return 3863;
        }
        if self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#power == 10
            && self.r#north == North::Up
        {
            return 4333;
        }
        if self.r#power == 0
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#north == North::Side
        {
            return 3954;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::Side
            && self.r#power == 9
            && self.r#north == North::None
        {
            return 4619;
        }
        if self.r#north == North::Side
            && self.r#west == West::None
            && self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#power == 15
        {
            return 4523;
        }
        if self.r#west == West::Up
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 3
            && self.r#south == South::Side
        {
            return 4848;
        }
        if self.r#south == South::Up
            && self.r#west == West::None
            && self.r#power == 9
            && self.r#north == North::None
            && self.r#east == East::Side
        {
            return 4613;
        }
        if self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#power == 7
            && self.r#east == East::None
            && self.r#north == North::Side
        {
            return 4882;
        }
        if self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#north == North::Up
            && self.r#power == 12
        {
            return 3924;
        }
        if self.r#west == West::None
            && self.r#north == North::Side
            && self.r#power == 7
            && self.r#south == South::Up
            && self.r#east == East::Up
        {
            return 4019;
        }
        if self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#power == 1
            && self.r#west == West::None
            && self.r#east == East::Side
        {
            return 4256;
        }
        if self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#power == 1
        {
            return 4972;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#power == 13
            && self.r#west == West::Up
            && self.r#east == East::None
        {
            return 5085;
        }
        if self.r#east == East::Up
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#power == 6
            && self.r#west == West::Side
        {
            return 4153;
        }
        if self.r#power == 10
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#east == East::Up
            && self.r#south == South::Side
        {
            return 3905;
        }
        if self.r#power == 2
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::Up
        {
            return 4700;
        }
        if self.r#power == 7
            && self.r#south == South::None
            && self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#west == West::Side
        {
            return 3880;
        }
        if self.r#power == 9
            && self.r#south == South::Up
            && self.r#east == East::Up
            && self.r#west == West::Side
            && self.r#north == North::None
        {
            return 4180;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::Side
            && self.r#west == West::Up
            && self.r#power == 5
        {
            return 4869;
        }
        if self.r#south == South::Side
            && self.r#power == 12
            && self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#east == East::Side
        {
            return 4353;
        }
        if self.r#west == West::Side
            && self.r#power == 10
            && self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#south == South::None
        {
            return 4051;
        }
        if self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#power == 15
            && self.r#west == West::Up
            && self.r#east == East::None
        {
            return 4812;
        }
        if self.r#east == East::Up
            && self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#power == 9
            && self.r#south == South::Up
        {
            return 4035;
        }
        if self.r#power == 4
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#south == South::Up
        {
            return 4567;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#west == West::Side
            && self.r#power == 3
            && self.r#south == South::None
        {
            return 4996;
        }
        if self.r#east == East::Side
            && self.r#power == 6
            && self.r#north == North::Up
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 4298;
        }
        if self.r#power == 9
            && self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#west == West::Side
            && self.r#east == East::Up
        {
            return 4036;
        }
        if self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#power == 10
            && self.r#south == South::Side
        {
            return 4480;
        }
        if self.r#west == West::Up
            && self.r#power == 3
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#east == East::None
        {
            return 4704;
        }
        if self.r#north == North::None
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#power == 3
        {
            return 4125;
        }
        if self.r#south == South::Side
            && self.r#north == North::None
            && self.r#power == 1
            && self.r#west == West::Up
            && self.r#east == East::None
        {
            return 4974;
        }
        if self.r#east == East::Up
            && self.r#power == 6
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#south == South::Side
        {
            return 4155;
        }
        if self.r#south == South::Up
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#power == 5
            && self.r#east == East::None
        {
            return 5007;
        }
        if self.r#north == North::Side
            && self.r#power == 0
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 3956;
        }
        if self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#power == 7
        {
            return 4018;
        }
        if self.r#east == East::Side
            && self.r#power == 11
            && self.r#west == West::Up
            && self.r#south == South::None
            && self.r#north == North::Side
        {
            return 4491;
        }
        if self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#west == West::None
            && self.r#power == 12
        {
            return 4496;
        }
        if self.r#south == South::Side
            && self.r#power == 11
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#west == West::Side
        {
            return 4633;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#power == 11
        {
            return 5064;
        }
        if self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#south == South::None
            && self.r#east == East::Up
            && self.r#power == 8
        {
            return 3888;
        }
        if self.r#power == 5
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::Up
        {
            return 4581;
        }
        if self.r#south == South::None
            && self.r#west == West::Side
            && self.r#power == 11
            && self.r#east == East::Up
            && self.r#north == North::None
        {
            return 4204;
        }
        if self.r#west == West::Side
            && self.r#south == South::None
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 7
        {
            return 4456;
        }
        if self.r#east == East::Up
            && self.r#north == North::None
            && self.r#power == 2
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 4124;
        }
        if self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#power == 4
            && self.r#west == West::Up
        {
            return 4278;
        }
        if self.r#east == East::None
            && self.r#power == 5
            && self.r#west == West::Side
            && self.r#south == South::Up
            && self.r#north == North::None
        {
            return 5008;
        }
        if self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 12
        {
            return 4498;
        }
        if self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#power == 11
        {
            return 4054;
        }
        if self.r#east == East::Side
            && self.r#power == 4
            && self.r#west == West::Side
            && self.r#south == South::Side
            && self.r#north == North::None
        {
            return 4570;
        }
        if self.r#north == North::Side
            && self.r#power == 10
            && self.r#east == East::Side
            && self.r#south == South::None
            && self.r#west == West::Up
        {
            return 4482;
        }
        if self.r#east == East::Up
            && self.r#power == 14
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#north == North::None
        {
            return 4228;
        }
        if self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#power == 13
            && self.r#west == West::None
            && self.r#south == South::Side
        {
            return 3932;
        }
        if self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#power == 8
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 4751;
        }
        if self.r#power == 5
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 4290;
        }
        if self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#south == South::None
            && self.r#power == 12
            && self.r#north == North::Up
        {
            return 4356;
        }
        if self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#power == 9
            && self.r#east == East::None
        {
            return 4756;
        }
        if self.r#south == South::Side
            && self.r#power == 14
            && self.r#north == North::Up
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 4805;
        }
        if self.r#west == West::Side
            && self.r#power == 8
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::Up
        {
            return 4753;
        }
        if self.r#power == 1
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#south == South::None
        {
            return 4977;
        }
        if self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#power == 0
            && self.r#east == East::Up
            && self.r#south == South::Up
        {
            return 3811;
        }
        if self.r#power == 6
            && self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#west == West::None
            && self.r#south == South::Up
        {
            return 3866;
        }
        if self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#power == 8
            && self.r#west == West::None
            && self.r#north == North::Side
        {
            return 4463;
        }
        if self.r#north == North::Up
            && self.r#west == West::Side
            && self.r#power == 6
            && self.r#east == East::None
            && self.r#south == South::Side
        {
            return 4732;
        }
        if self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#power == 15
            && self.r#south == South::Up
        {
            return 4378;
        }
        if self.r#north == North::Side
            && self.r#south == South::None
            && self.r#east == East::Side
            && self.r#west == West::None
            && self.r#power == 0
        {
            return 4394;
        }
        if self.r#power == 2
            && self.r#north == North::None
            && self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#west == West::Up
        {
            return 4551;
        }
        if self.r#north == North::Up
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#power == 9
            && self.r#south == South::Side
        {
            return 4327;
        }
        if self.r#power == 3
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::Side
        {
            return 4994;
        }
        if self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#power == 3
            && self.r#east == East::None
        {
            return 4701;
        }
        if self.r#power == 11
            && self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#west == West::Up
        {
            return 4056;
        }
        if self.r#south == South::None
            && self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#west == West::Up
            && self.r#power == 15
        {
            return 4095;
        }
        if self.r#east == East::Up
            && self.r#power == 7
            && self.r#west == West::Side
            && self.r#north == North::None
            && self.r#south == South::Side
        {
            return 4165;
        }
        if self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#north == North::None
            && self.r#west == West::Side
            && self.r#power == 8
        {
            return 4174;
        }
        if self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#power == 8
            && self.r#south == South::Side
            && self.r#north == North::Up
        {
            return 4317;
        }
        if self.r#east == East::None
            && self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#power == 8
        {
            return 4749;
        }
        if self.r#west == West::Side
            && self.r#south == South::Up
            && self.r#east == East::None
            && self.r#power == 14
            && self.r#north == North::Up
        {
            return 4801;
        }
        if self.r#west == West::Up
            && self.r#south == South::Up
            && self.r#power == 5
            && self.r#east == East::None
            && self.r#north == North::Side
        {
            return 4863;
        }
        if self.r#north == North::Side
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#power == 15
        {
            return 4959;
        }
        if self.r#north == North::None
            && self.r#power == 4
            && self.r#east == East::None
            && self.r#south == South::Up
            && self.r#west == West::Up
        {
            return 4998;
        }
        if self.r#north == North::Up
            && self.r#power == 8
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#east == East::None
        {
            return 4747;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#power == 7
            && self.r#west == West::None
            && self.r#north == North::Side
        {
            return 4889;
        }
        if self.r#west == West::Side
            && self.r#power == 13
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#north == North::Up
        {
            return 3928;
        }
        if self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#power == 0
            && self.r#west == West::None
            && self.r#north == North::Side
        {
            return 4388;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#power == 9
        {
            return 5051;
        }
        if self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#power == 3
        {
            return 4558;
        }
        if self.r#north == North::None
            && self.r#west == West::None
            && self.r#east == East::Up
            && self.r#power == 0
            && self.r#south == South::Up
        {
            return 4100;
        }
        if self.r#power == 8
            && self.r#north == North::Side
            && self.r#west == West::Up
            && self.r#south == South::None
            && self.r#east == East::Side
        {
            return 4464;
        }
        if self.r#power == 0
            && self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#east == East::None
        {
            return 4679;
        }
        if self.r#power == 4
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#west == West::None
        {
            return 4715;
        }
        if self.r#power == 6
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#north == North::Side
        {
            return 4880;
        }
        if self.r#south == South::None
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#power == 14
        {
            return 3943;
        }
        if self.r#south == South::None
            && self.r#north == North::Side
            && self.r#power == 12
            && self.r#west == West::Side
            && self.r#east == East::Up
        {
            return 4069;
        }
        if self.r#west == West::None
            && self.r#north == North::Up
            && self.r#power == 13
            && self.r#east == East::Side
            && self.r#south == South::None
        {
            return 4367;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#power == 0
            && self.r#east == East::Side
            && self.r#south == South::Up
        {
            return 4532;
        }
        if self.r#east == East::Side
            && self.r#power == 1
            && self.r#north == North::Up
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 4253;
        }
        if self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#power == 1
        {
            return 4254;
        }
        if self.r#power == 12
            && self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#west == West::Up
        {
            return 4350;
        }
        if self.r#south == South::Side
            && self.r#power == 13
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#west == West::None
        {
            return 4652;
        }
        if self.r#power == 0
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 4823;
        }
        if self.r#north == North::Up
            && self.r#power == 6
            && self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 4297;
        }
        if self.r#power == 5
            && self.r#north == North::Up
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#east == East::Side
        {
            return 4287;
        }
        if self.r#south == South::Up
            && self.r#west == West::None
            && self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#power == 13
        {
            return 4361;
        }
        if self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#power == 2
            && self.r#west == West::Side
            && self.r#south == South::Up
        {
            return 4405;
        }
        if self.r#power == 2
            && self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#south == South::Side
        {
            return 4407;
        }
        if self.r#east == East::Up
            && self.r#power == 11
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#north == North::Side
        {
            return 4055;
        }
        if self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#power == 7
        {
            return 4161;
        }
        if self.r#south == South::Side
            && self.r#west == West::None
            && self.r#power == 5
            && self.r#east == East::Side
            && self.r#north == North::Up
        {
            return 4292;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#power == 1
            && self.r#south == South::Side
            && self.r#east == East::Side
        {
            return 4544;
        }
        if self.r#north == North::None
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#south == South::Side
            && self.r#power == 7
        {
            return 4597;
        }
        if self.r#power == 0
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#south == South::Up
            && self.r#east == East::None
        {
            return 4675;
        }
        if self.r#north == North::Up
            && self.r#power == 12
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#east == East::Side
        {
            return 4351;
        }
        if self.r#power == 2
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#west == West::Side
        {
            return 4984;
        }
        if self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#power == 1
            && self.r#south == South::None
        {
            return 4401;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#power == 9
            && self.r#north == North::Up
            && self.r#east == East::None
        {
            return 4763;
        }
        if self.r#west == West::None
            && self.r#east == East::None
            && self.r#power == 9
            && self.r#south == South::Up
            && self.r#north == North::Up
        {
            return 4757;
        }
        if self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#power == 1
            && self.r#north == North::Side
            && self.r#east == East::Side
        {
            return 4398;
        }
        if self.r#power == 7
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#west == West::Side
            && self.r#south == South::Up
        {
            return 4738;
        }
        if self.r#power == 9
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#west == West::Side
            && self.r#north == North::Up
        {
            return 4762;
        }
        if self.r#east == East::Up
            && self.r#power == 15
            && self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#west == West::Up
        {
            return 3945;
        }
        if self.r#power == 8
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#north == North::Side
        {
            return 4029;
        }
        if self.r#east == East::Up
            && self.r#power == 8
            && self.r#north == North::Side
            && self.r#south == South::None
            && self.r#west == West::Side
        {
            return 4033;
        }
        if self.r#north == North::Up
            && self.r#south == South::Up
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#power == 6
        {
            return 4730;
        }
        if self.r#west == West::None
            && self.r#power == 6
            && self.r#south == South::Side
            && self.r#north == North::None
            && self.r#east == East::Up
        {
            return 4157;
        }
        if self.r#south == South::Up
            && self.r#east == East::Up
            && self.r#north == North::None
            && self.r#power == 1
            && self.r#west == West::Up
        {
            return 4107;
        }
        if self.r#power == 15
            && self.r#east == East::Up
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::Side
        {
            return 4240;
        }
        if self.r#north == North::None
            && self.r#south == South::Side
            && self.r#west == West::None
            && self.r#power == 8
            && self.r#east == East::Side
        {
            return 4607;
        }
        if self.r#east == East::None
            && self.r#west == West::Side
            && self.r#north == North::None
            && self.r#power == 0
            && self.r#south == South::None
        {
            return 4969;
        }
        if self.r#power == 1
            && self.r#east == East::None
            && self.r#west == West::Up
            && self.r#south == South::None
            && self.r#north == North::Side
        {
            return 4833;
        }
        if self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#power == 8
            && self.r#south == South::None
        {
            return 3890;
        }
        if self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#power == 6
            && self.r#west == West::Side
        {
            return 4015;
        }
        if self.r#north == North::None
            && self.r#power == 14
            && self.r#west == West::Up
            && self.r#south == South::Side
            && self.r#east == East::Side
        {
            return 4659;
        }
        if self.r#south == South::None
            && self.r#north == North::Side
            && self.r#west == West::Side
            && self.r#east == East::None
            && self.r#power == 9
        {
            return 4906;
        }
        if self.r#west == West::Up
            && self.r#power == 1
            && self.r#south == South::Up
            && self.r#north == North::None
            && self.r#east == East::None
        {
            return 4971;
        }
        if self.r#south == South::None
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#power == 7
        {
            return 5031;
        }
        if self.r#east == East::Up
            && self.r#power == 3
            && self.r#north == North::Up
            && self.r#south == South::None
            && self.r#west == West::Side
        {
            return 3844;
        }
        if self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#power == 15
        {
            return 3948;
        }
        if self.r#south == South::None
            && self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#power == 9
            && self.r#west == West::Up
        {
            return 4473;
        }
        if self.r#west == West::Side
            && self.r#east == East::None
            && self.r#south == South::Up
            && self.r#power == 4
            && self.r#north == North::Up
        {
            return 4711;
        }
        if self.r#south == South::None
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#power == 2
            && self.r#east == East::None
        {
            return 4986;
        }
        if self.r#power == 3
            && self.r#west == West::Side
            && self.r#south == South::None
            && self.r#north == North::Side
            && self.r#east == East::None
        {
            return 4852;
        }
        if self.r#west == West::None
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#power == 11
            && self.r#east == East::None
        {
            return 4922;
        }
        if self.r#west == West::Side
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#north == North::Side
            && self.r#power == 5
        {
            return 4867;
        }
        if self.r#south == South::Side
            && self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#west == West::Side
            && self.r#power == 2
        {
            return 3976;
        }
        if self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#power == 8
            && self.r#south == South::Up
            && self.r#east == East::Up
        {
            return 3882;
        }
        if self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#power == 14
            && self.r#west == West::Up
        {
            return 4080;
        }
        if self.r#west == West::Up
            && self.r#power == 3
            && self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#north == North::Side
        {
            return 4416;
        }
        if self.r#south == South::None
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#west == West::Side
            && self.r#power == 13
        {
            return 3934;
        }
        if self.r#west == West::Up
            && self.r#power == 13
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#east == East::None
        {
            return 5079;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#power == 5
        {
            return 4151;
        }
        if self.r#west == West::None
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 14
            && self.r#south == South::Side
        {
            return 4517;
        }
        if self.r#north == North::Side
            && self.r#west == West::None
            && self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#power == 2
        {
            return 4406;
        }
        if self.r#south == South::None
            && self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#power == 4
            && self.r#west == West::None
        {
            return 4430;
        }
        if self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#east == East::Up
            && self.r#west == West::None
            && self.r#power == 5
        {
            return 4001;
        }
        if self.r#power == 15
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::Up
        {
            return 4521;
        }
        if self.r#east == East::Side
            && self.r#power == 2
            && self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#west == West::None
        {
            return 4262;
        }
        if self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#power == 10
        {
            return 4767;
        }
        if self.r#west == West::Up
            && self.r#power == 13
            && self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#north == North::Side
        {
            return 4506;
        }
        if self.r#west == West::None
            && self.r#south == South::Side
            && self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#power == 10
        {
            return 4481;
        }
        if self.r#south == South::None
            && self.r#power == 1
            && self.r#east == East::Side
            && self.r#west == West::Up
            && self.r#north == North::None
        {
            return 4545;
        }
        if self.r#east == East::Side
            && self.r#west == West::None
            && self.r#north == North::Up
            && self.r#power == 3
            && self.r#south == South::None
        {
            return 4277;
        }
        if self.r#south == South::Side
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#power == 9
            && self.r#east == East::Side
        {
            return 4472;
        }
        if self.r#south == South::Side
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#power == 4
            && self.r#west == West::None
        {
            return 5003;
        }
        if self.r#east == East::Up
            && self.r#power == 11
            && self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#west == West::None
        {
            return 3914;
        }
        if self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#power == 2
            && self.r#east == East::None
        {
            return 4837;
        }
        if self.r#power == 4
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 4859;
        }
        if self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#power == 1
            && self.r#west == West::Side
            && self.r#south == South::None
        {
            return 3970;
        }
        if self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#power == 5
            && self.r#west == West::None
        {
            return 4436;
        }
        if self.r#north == North::Up
            && self.r#power == 12
            && self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#west == West::None
        {
            return 4352;
        }
        if self.r#south == South::None
            && self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#power == 2
            && self.r#west == West::Up
        {
            return 3834;
        }
        if self.r#power == 6
            && self.r#south == South::None
            && self.r#north == North::Up
            && self.r#east == East::None
            && self.r#west == West::Up
        {
            return 4734;
        }
        if self.r#north == North::Up
            && self.r#power == 8
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#east == East::Side
        {
            return 4316;
        }
        if self.r#north == North::None
            && self.r#west == West::Side
            && self.r#south == South::Up
            && self.r#power == 4
            && self.r#east == East::None
        {
            return 4999;
        }
        if self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#power == 4
            && self.r#west == West::None
        {
            return 4283;
        }
        if self.r#north == North::None
            && self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#power == 13
        {
            return 4219;
        }
        if self.r#north == North::None
            && self.r#west == West::None
            && self.r#power == 11
            && self.r#south == South::None
            && self.r#east == East::Up
        {
            return 4205;
        }
        if self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#power == 10
            && self.r#west == West::Up
        {
            return 4476;
        }
        if self.r#power == 8
            && self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#west == West::Side
        {
            return 4318;
        }
        if self.r#south == South::Up
            && self.r#power == 13
            && self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#west == West::Up
        {
            return 4503;
        }
        if self.r#power == 14
            && self.r#south == South::None
            && self.r#east == East::Side
            && self.r#west == West::Up
            && self.r#north == North::Side
        {
            return 4518;
        }
        if self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#power == 10
        {
            return 4764;
        }
        if self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 1
            && self.r#west == West::Up
            && self.r#south == South::Up
        {
            return 4395;
        }
        if self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 0
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 4826;
        }
        if self.r#east == East::Up
            && self.r#power == 0
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#north == North::Up
        {
            return 3812;
        }
        if self.r#power == 11
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#west == West::None
        {
            return 4202;
        }
        if self.r#power == 6
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 5019;
        }
        if self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#power == 2
            && self.r#north == North::Up
        {
            return 4260;
        }
        if self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#power == 10
            && self.r#north == North::Up
            && self.r#west == West::None
        {
            return 4334;
        }
        if self.r#power == 11
            && self.r#east == East::Side
            && self.r#west == West::Up
            && self.r#south == South::Up
            && self.r#north == North::None
        {
            return 4629;
        }
        if self.r#power == 9
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#east == East::Side
        {
            return 4611;
        }
        if self.r#power == 13
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 5084;
        }
        if self.r#east == East::None
            && self.r#power == 12
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#west == West::Side
        {
            return 4930;
        }
        if self.r#north == North::None
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#power == 13
            && self.r#west == West::Up
        {
            return 4221;
        }
        if self.r#south == South::None
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#power == 11
            && self.r#north == North::Up
        {
            return 4779;
        }
        if self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#power == 3
            && self.r#east == East::Up
            && self.r#west == West::None
        {
            return 3983;
        }
        if self.r#power == 11
            && self.r#north == North::Side
            && self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#south == South::Side
        {
            return 4488;
        }
        if self.r#power == 1
            && self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#north == North::Up
        {
            return 4255;
        }
        if self.r#east == East::None
            && self.r#power == 11
            && self.r#north == North::Side
            && self.r#west == West::Side
            && self.r#south == South::Side
        {
            return 4921;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#power == 11
            && self.r#south == South::Side
            && self.r#west == West::Side
        {
            return 5065;
        }
        if self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#power == 15
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 4379;
        }
        if self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#north == North::None
            && self.r#power == 1
        {
            return 4111;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#power == 5
            && self.r#south == South::Up
            && self.r#east == East::Side
        {
            return 4577;
        }
        if self.r#power == 6
            && self.r#north == North::Side
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 4875;
        }
        if self.r#north == North::None
            && self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#power == 2
            && self.r#west == West::Side
        {
            return 4120;
        }
        if self.r#south == South::Up
            && self.r#power == 14
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#east == East::Side
        {
            return 4369;
        }
        if self.r#east == East::Side
            && self.r#north == North::None
            && self.r#power == 8
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 4605;
        }
        if self.r#power == 4
            && self.r#east == East::Side
            && self.r#south == South::None
            && self.r#north == North::Side
            && self.r#west == West::Up
        {
            return 4428;
        }
        if self.r#east == East::Side
            && self.r#power == 11
            && self.r#west == West::Up
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 4635;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#west == West::Side
            && self.r#power == 5
        {
            return 5014;
        }
        if self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#power == 8
            && self.r#north == North::Side
            && self.r#east == East::None
        {
            return 4894;
        }
        if self.r#north == North::Side
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#west == West::Side
            && self.r#power == 6
        {
            return 4879;
        }
        if self.r#east == East::Side
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#power == 3
            && self.r#west == West::None
        {
            return 4559;
        }
        if self.r#power == 12
            && self.r#west == West::Side
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::Up
        {
            return 4213;
        }
        if self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#power == 6
        {
            return 4728;
        }
        if self.r#north == North::Up
            && self.r#power == 9
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 3893;
        }
        if self.r#power == 8
            && self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#east == East::Up
        {
            return 3887;
        }
        if self.r#west == West::Up
            && self.r#east == East::None
            && self.r#power == 9
            && self.r#south == South::None
            && self.r#north == North::Side
        {
            return 4905;
        }
        if self.r#south == South::Up
            && self.r#power == 0
            && self.r#west == West::Side
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 4963;
        }
        if self.r#east == East::Side
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#power == 5
        {
            return 4575;
        }
        if self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#power == 8
            && self.r#north == North::Up
        {
            return 3886;
        }
        if self.r#west == West::Up
            && self.r#power == 10
            && self.r#east == East::Up
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 4194;
        }
        if self.r#east == East::Side
            && self.r#west == West::Up
            && self.r#south == South::Up
            && self.r#north == North::None
            && self.r#power == 6
        {
            return 4584;
        }
        if self.r#east == East::None
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#power == 9
        {
            return 4901;
        }
        if self.r#power == 9
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#south == South::Up
            && self.r#north == North::Up
        {
            return 4755;
        }
        if self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#power == 1
            && self.r#east == East::None
        {
            return 4831;
        }
        if self.r#south == South::Up
            && self.r#power == 15
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 4667;
        }
        if self.r#power == 14
            && self.r#north == North::Side
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#east == East::Up
        {
            return 4086;
        }
        if self.r#power == 13
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#west == West::Side
            && self.r#north == North::Side
        {
            return 4942;
        }
        if self.r#east == East::Side
            && self.r#power == 5
            && self.r#west == West::Up
            && self.r#south == South::Up
            && self.r#north == North::Side
        {
            return 4431;
        }
        if self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#power == 4
            && self.r#south == South::Up
        {
            return 3848;
        }
        if self.r#south == South::Side
            && self.r#power == 2
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Up
        {
            return 4121;
        }
        if self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#power == 2
        {
            return 4263;
        }
        if self.r#power == 4
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::Up
        {
            return 3854;
        }
        if self.r#west == West::Up
            && self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#power == 5
            && self.r#east == East::Up
        {
            return 3858;
        }
        if self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#power == 6
            && self.r#south == South::Side
        {
            return 4299;
        }
        if self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#power == 7
            && self.r#east == East::Up
            && self.r#west == West::None
        {
            return 3878;
        }
        if self.r#power == 13
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#south == South::Up
            && self.r#north == North::Side
        {
            return 4504;
        }
        if self.r#south == South::Up
            && self.r#power == 3
            && self.r#west == West::None
            && self.r#north == North::Up
            && self.r#east == East::Side
        {
            return 4271;
        }
        if self.r#power == 2
            && self.r#west == West::Side
            && self.r#south == South::Up
            && self.r#north == North::None
            && self.r#east == East::Side
        {
            return 4549;
        }
        if self.r#south == South::Side
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#power == 1
        {
            return 4686;
        }
        if self.r#east == East::None
            && self.r#south == South::Up
            && self.r#power == 1
            && self.r#west == West::None
            && self.r#north == North::Side
        {
            return 4829;
        }
        if self.r#north == North::None
            && self.r#west == West::None
            && self.r#power == 9
            && self.r#east == East::Up
            && self.r#south == South::Up
        {
            return 4181;
        }
        if self.r#south == South::Side
            && self.r#power == 2
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#west == West::Up
        {
            return 4839;
        }
        if self.r#south == South::Side
            && self.r#east == East::None
            && self.r#power == 7
            && self.r#north == North::Side
            && self.r#west == West::None
        {
            return 4886;
        }
        if self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#power == 9
            && self.r#west == West::Side
        {
            return 3895;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Side
            && self.r#power == 1
            && self.r#west == West::None
        {
            return 4976;
        }
        if self.r#west == West::Side
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#power == 4
            && self.r#south == South::Side
        {
            return 5002;
        }
        if self.r#south == South::Up
            && self.r#east == East::None
            && self.r#west == West::Up
            && self.r#power == 8
            && self.r#north == North::None
        {
            return 5034;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#power == 15
            && self.r#west == West::Side
        {
            return 5098;
        }
        if self.r#east == East::None
            && self.r#power == 3
            && self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#west == West::None
        {
            return 4703;
        }
        if self.r#power == 0
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#west == West::None
        {
            return 3815;
        }
        if self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#power == 4
            && self.r#west == West::None
        {
            return 3851;
        }
        if self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#south == South::None
            && self.r#east == East::Up
            && self.r#power == 6
        {
            return 3870;
        }
        if self.r#north == North::None
            && self.r#west == West::None
            && self.r#south == South::Side
            && self.r#power == 4
            && self.r#east == East::Side
        {
            return 4571;
        }
        if self.r#east == East::None
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#power == 2
            && self.r#south == South::Up
        {
            return 4982;
        }
        if self.r#east == East::None
            && self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#south == South::None
            && self.r#power == 2
        {
            return 4698;
        }
        if self.r#power == 13
            && self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#east == East::Side
        {
            return 4364;
        }
        if self.r#east == East::Up
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#power == 7
            && self.r#south == South::Side
        {
            return 4166;
        }
        if self.r#north == North::None
            && self.r#west == West::None
            && self.r#power == 9
            && self.r#east == East::Side
            && self.r#south == South::Side
        {
            return 4616;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#north == North::Up
            && self.r#power == 6
            && self.r#east == East::Up
        {
            return 3872;
        }
        if self.r#north == North::Side
            && self.r#power == 3
            && self.r#south == South::Side
            && self.r#west == West::None
            && self.r#east == East::Up
        {
            return 3986;
        }
        if self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#west == West::Side
            && self.r#power == 1
        {
            return 3826;
        }
        if self.r#east == East::Side
            && self.r#power == 3
            && self.r#south == South::Side
            && self.r#north == North::Side
            && self.r#west == West::Side
        {
            return 4417;
        }
        if self.r#west == West::Side
            && self.r#power == 13
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#south == South::Up
        {
            return 4936;
        }
        if self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#power == 11
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 4919;
        }
        if self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#south == South::None
            && self.r#power == 15
            && self.r#north == North::None
        {
            return 4671;
        }
        if self.r#north == North::None
            && self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#power == 13
            && self.r#south == South::Up
        {
            return 4647;
        }
        if self.r#north == North::Side
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::Side
            && self.r#power == 6
        {
            return 4448;
        }
        if self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#power == 10
            && self.r#west == West::Up
        {
            return 3903;
        }
        if self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#power == 6
        {
            return 3868;
        }
        if self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#power == 10
        {
            return 3902;
        }
        if self.r#south == South::Side
            && self.r#power == 6
            && self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#west == West::Side
        {
            return 4012;
        }
        if self.r#south == South::Up
            && self.r#power == 13
            && self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#east == East::None
        {
            return 4935;
        }
        if self.r#north == North::Up
            && self.r#south == South::Up
            && self.r#power == 14
            && self.r#west == West::Up
            && self.r#east == East::None
        {
            return 4800;
        }
        if self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#power == 10
        {
            return 4621;
        }
        if self.r#east == East::None
            && self.r#west == West::Up
            && self.r#power == 5
            && self.r#south == South::None
            && self.r#north == North::Up
        {
            return 4725;
        }
        if self.r#power == 9
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#east == East::Up
        {
            return 3891;
        }
        if self.r#south == South::Side
            && self.r#power == 8
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#west == West::Up
        {
            return 3885;
        }
        if self.r#power == 0
            && self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#west == West::Side
        {
            return 4387;
        }
        if self.r#south == South::None
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#east == East::Side
            && self.r#power == 1
        {
            return 4403;
        }
        if self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#power == 15
            && self.r#north == North::None
            && self.r#west == West::Up
        {
            return 4233;
        }
        if self.r#north == North::Side
            && self.r#south == South::None
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#power == 8
        {
            return 4465;
        }
        if self.r#north == North::Up
            && self.r#power == 2
            && self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#west == West::Side
        {
            return 4264;
        }
        if self.r#west == West::None
            && self.r#south == South::Up
            && self.r#north == North::None
            && self.r#power == 12
            && self.r#east == East::Side
        {
            return 4640;
        }
        if self.r#west == West::None
            && self.r#north == North::Up
            && self.r#power == 7
            && self.r#east == East::Side
            && self.r#south == South::Up
        {
            return 4307;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#power == 1
        {
            return 4259;
        }
        if self.r#power == 3
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#north == North::None
        {
            return 4126;
        }
        if self.r#north == North::Up
            && self.r#south == South::None
            && self.r#power == 7
            && self.r#east == East::Side
            && self.r#west == West::Side
        {
            return 4312;
        }
        if self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#power == 6
            && self.r#south == South::Side
        {
            return 4733;
        }
        if self.r#power == 14
            && self.r#north == North::Up
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 4808;
        }
        if self.r#south == South::None
            && self.r#power == 7
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#west == West::Side
        {
            return 4168;
        }
        if self.r#east == East::Up
            && self.r#power == 5
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::Up
        {
            return 3999;
        }
        if self.r#north == North::None
            && self.r#east == East::Up
            && self.r#power == 9
            && self.r#south == South::None
            && self.r#west == West::Up
        {
            return 4185;
        }
        if self.r#east == East::None
            && self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#power == 5
        {
            return 4724;
        }
        if self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 5
            && self.r#west == West::Side
        {
            return 4870;
        }
        if self.r#south == South::Up
            && self.r#power == 8
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Side
        {
            return 4892;
        }
        if self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#power == 4
            && self.r#west == West::Side
            && self.r#south == South::Up
        {
            return 4279;
        }
        if self.r#power == 3
            && self.r#north == North::Side
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 4853;
        }
        if self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#east == East::None
            && self.r#power == 6
            && self.r#west == West::None
        {
            return 4877;
        }
        if self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#power == 7
            && self.r#east == East::Side
        {
            return 4309;
        }
        if self.r#north == North::Side
            && self.r#power == 12
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::Up
        {
            return 4928;
        }
        if self.r#east == East::Up
            && self.r#north == North::None
            && self.r#power == 0
            && self.r#west == West::Up
            && self.r#south == South::Side
        {
            return 4101;
        }
        if self.r#power == 4
            && self.r#east == East::Side
            && self.r#west == West::None
            && self.r#north == North::Side
            && self.r#south == South::Side
        {
            return 4427;
        }
        if self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#south == South::None
            && self.r#north == North::Up
            && self.r#power == 0
        {
            return 4249;
        }
        if self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#west == West::None
            && self.r#power == 14
        {
            return 4373;
        }
        if self.r#east == East::None
            && self.r#power == 12
            && self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#south == South::None
        {
            return 4788;
        }
        if self.r#south == South::None
            && self.r#west == West::Up
            && self.r#power == 14
            && self.r#north == North::Up
            && self.r#east == East::None
        {
            return 4806;
        }
        if self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#north == North::Side
            && self.r#west == West::Up
            && self.r#power == 10
        {
            return 4047;
        }
        if self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#power == 0
        {
            return 3810;
        }
        if self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#south == South::Up
            && self.r#power == 6
            && self.r#east == East::Up
        {
            return 3864;
        }
        if self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#north == North::None
            && self.r#power == 5
        {
            return 4144;
        }
        if self.r#east == East::Side
            && self.r#south == South::None
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#power == 2
        {
            return 4412;
        }
        if self.r#south == South::None
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#power == 0
            && self.r#west == West::None
        {
            return 4970;
        }
        if self.r#west == West::Side
            && self.r#south == South::None
            && self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#power == 1
        {
            return 4258;
        }
        if self.r#north == North::Side
            && self.r#south == South::None
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#power == 12
        {
            return 4501;
        }
        if self.r#east == East::Up
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#power == 9
        {
            return 4187;
        }
        if self.r#east == East::Side
            && self.r#power == 15
            && self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#south == South::Up
        {
            return 4377;
        }
        if self.r#south == South::None
            && self.r#power == 8
            && self.r#north == North::Side
            && self.r#west == West::Side
            && self.r#east == East::None
        {
            return 4897;
        }
        if self.r#west == West::Side
            && self.r#power == 0
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#east == East::Up
        {
            return 4105;
        }
        if self.r#north == North::Up
            && self.r#power == 5
            && self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 4288;
        }
        if self.r#west == West::Up
            && self.r#power == 10
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#south == South::Up
        {
            return 4332;
        }
        if self.r#power == 2
            && self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#north == North::Up
        {
            return 3831;
        }
        if self.r#east == East::Side
            && self.r#west == West::Up
            && self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#power == 3
        {
            return 4413;
        }
        if self.r#west == West::None
            && self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#power == 4
        {
            return 3995;
        }
        if self.r#south == South::Side
            && self.r#power == 5
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#north == North::Side
        {
            return 4868;
        }
        if self.r#east == East::None
            && self.r#power == 12
            && self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#south == South::None
        {
            return 4933;
        }
        if self.r#power == 4
            && self.r#north == North::Side
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#west == West::Side
        {
            return 4861;
        }
        if self.r#east == East::None
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#power == 15
        {
            return 5097;
        }
        if self.r#west == West::Up
            && self.r#power == 12
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#south == South::None
        {
            return 4644;
        }
        if self.r#north == North::None
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::Up
            && self.r#power == 6
        {
            return 4160;
        }
        if self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#east == East::None
            && self.r#power == 7
            && self.r#west == West::Up
        {
            return 4737;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::Side
            && self.r#power == 4
        {
            return 4574;
        }
        if self.r#power == 10
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 3901;
        }
        if self.r#south == South::Side
            && self.r#west == West::None
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#power == 9
        {
            return 4328;
        }
        if self.r#north == North::Up
            && self.r#power == 15
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#west == West::Up
        {
            return 4815;
        }
        if self.r#west == West::Side
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#power == 14
            && self.r#north == North::Side
        {
            return 4948;
        }
        if self.r#south == South::None
            && self.r#north == North::Up
            && self.r#power == 3
            && self.r#east == East::None
            && self.r#west == West::Up
        {
            return 4707;
        }
        if self.r#north == North::Up
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#power == 4
            && self.r#south == South::None
        {
            return 4718;
        }
        if self.r#east == East::Up
            && self.r#west == West::Side
            && self.r#power == 3
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 4132;
        }
        if self.r#east == East::None
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#south == South::None
            && self.r#power == 10
        {
            return 4771;
        }
        if self.r#power == 10
            && self.r#south == South::Side
            && self.r#north == North::Side
            && self.r#west == West::Up
            && self.r#east == East::None
        {
            return 4911;
        }
        if self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#power == 2
            && self.r#north == North::None
            && self.r#east == East::None
        {
            return 4980;
        }
        if self.r#north == North::Side
            && self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#power == 2
            && self.r#south == South::Side
        {
            return 3975;
        }
        if self.r#east == East::Side
            && self.r#power == 1
            && self.r#west == West::None
            && self.r#north == North::Side
            && self.r#south == South::Side
        {
            return 4400;
        }
        if self.r#east == East::None
            && self.r#power == 1
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 4979;
        }
        if self.r#power == 12
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::Up
        {
            return 4214;
        }
        if self.r#south == South::Up
            && self.r#east == East::Up
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#power == 4
        {
            return 3847;
        }
        if self.r#power == 11
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 4925;
        }
        if self.r#north == North::None
            && self.r#east == East::Up
            && self.r#power == 2
            && self.r#west == West::Up
            && self.r#south == South::None
        {
            return 4122;
        }
        if self.r#south == South::Side
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#power == 5
        {
            return 4580;
        }
        if self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#north == North::Up
            && self.r#power == 13
        {
            return 4799;
        }
        if self.r#power == 5
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#east == East::Up
            && self.r#north == North::None
        {
            return 4145;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#north == North::Side
            && self.r#power == 4
            && self.r#east == East::Up
        {
            return 3998;
        }
        if self.r#south == South::None
            && self.r#power == 2
            && self.r#north == North::Side
            && self.r#west == West::Up
            && self.r#east == East::Up
        {
            return 3978;
        }
        if self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#south == South::None
            && self.r#power == 14
        {
            return 4375;
        }
        if self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#power == 6
        {
            return 4304;
        }
        if self.r#north == North::Side
            && self.r#power == 11
            && self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#south == South::None
        {
            return 4060;
        }
        if self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#power == 15
            && self.r#north == North::Side
            && self.r#south == South::Side
        {
            return 4525;
        }
        if self.r#power == 14
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#south == South::Side
            && self.r#north == North::None
        {
            return 4660;
        }
        if self.r#east == East::Up
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#power == 14
        {
            return 4226;
        }
        if self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#power == 3
        {
            return 4269;
        }
        if self.r#power == 0
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#south == South::Side
        {
            return 4965;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#power == 12
        {
            return 5078;
        }
        if self.r#power == 6
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#east == East::Side
            && self.r#north == North::None
        {
            return 4586;
        }
        if self.r#power == 6
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#north == North::Side
        {
            return 4441;
        }
        if self.r#east == East::None
            && self.r#north == North::Side
            && self.r#west == West::Up
            && self.r#south == South::None
            && self.r#power == 8
        {
            return 4896;
        }
        if self.r#south == South::None
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#power == 7
        {
            return 4599;
        }
        if self.r#power == 3
            && self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#east == East::Up
        {
            return 3981;
        }
        if self.r#power == 8
            && self.r#south == South::None
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#north == North::Up
        {
            return 4321;
        }
        if self.r#south == South::Up
            && self.r#power == 1
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#west == West::Side
        {
            return 4396;
        }
        if self.r#east == East::Up
            && self.r#power == 13
            && self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#west == West::None
        {
            return 4073;
        }
        if self.r#north == North::None
            && self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#power == 10
            && self.r#south == South::None
        {
            return 4626;
        }
        if self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#power == 10
            && self.r#west == West::Up
        {
            return 3900;
        }
        if self.r#power == 6
            && self.r#west == West::None
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#east == East::Up
        {
            return 4010;
        }
        if self.r#east == East::None
            && self.r#power == 15
            && self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#south == South::None
        {
            return 4960;
        }
        if self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#power == 10
        {
            return 4620;
        }
        if self.r#north == North::None
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#power == 0
        {
            return 4533;
        }
        if self.r#north == North::None
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#power == 14
            && self.r#south == South::None
        {
            return 4663;
        }
        if self.r#east == East::Up
            && self.r#power == 15
            && self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#west == West::Up
        {
            return 4089;
        }
        if self.r#power == 14
            && self.r#east == East::Up
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 4225;
        }
        if self.r#power == 14
            && self.r#west == West::Side
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 5095;
        }
        if self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#power == 11
            && self.r#east == East::Side
        {
            return 4489;
        }
        if self.r#east == East::Side
            && self.r#power == 12
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#west == West::Side
        {
            return 4354;
        }
        if self.r#power == 9
            && self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#south == South::None
        {
            return 4329;
        }
        if self.r#north == North::Up
            && self.r#west == West::Side
            && self.r#east == East::None
            && self.r#south == South::Up
            && self.r#power == 11
        {
            return 4774;
        }
        if self.r#east == East::None
            && self.r#power == 13
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#south == South::None
        {
            return 4798;
        }
        if self.r#south == South::Side
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 10
            && self.r#west == West::Side
        {
            return 4912;
        }
        if self.r#west == West::Up
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#power == 2
            && self.r#south == South::None
        {
            return 4554;
        }
        if self.r#south == South::None
            && self.r#power == 1
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#west == West::None
        {
            return 4835;
        }
        if self.r#east == East::None
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#south == South::Side
            && self.r#power == 9
        {
            return 5046;
        }
        if self.r#south == South::None
            && self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#power == 15
        {
            return 4528;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#power == 13
        {
            return 5087;
        }
        if self.r#power == 12
            && self.r#north == North::Up
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 4790;
        }
        if self.r#east == East::Up
            && self.r#west == West::Side
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#power == 9
        {
            return 4186;
        }
        if self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#power == 3
            && self.r#west == West::None
        {
            return 4562;
        }
        if self.r#power == 5
            && self.r#north == North::Up
            && self.r#south == South::None
            && self.r#west == West::Side
            && self.r#east == East::Side
        {
            return 4294;
        }
        if self.r#power == 12
            && self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#south == South::Up
        {
            return 4495;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#power == 8
            && self.r#east == East::Side
        {
            return 4610;
        }
        if self.r#west == West::Side
            && self.r#east == East::None
            && self.r#power == 12
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 5077;
        }
        if self.r#east == East::Up
            && self.r#power == 1
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#west == West::None
        {
            return 3968;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#power == 8
        {
            return 5036;
        }
        if self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#east == East::Up
            && self.r#power == 3
        {
            return 3842;
        }
        if self.r#east == East::None
            && self.r#south == South::Up
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#power == 14
        {
            return 5090;
        }
        if self.r#north == North::None
            && self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#power == 12
            && self.r#west == West::Up
        {
            return 4638;
        }
        if self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#power == 13
        {
            return 4072;
        }
        if self.r#west == West::None
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#power == 0
            && self.r#north == North::None
        {
            return 4106;
        }
        if self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#power == 2
        {
            return 4842;
        }
        if self.r#east == East::Up
            && self.r#power == 0
            && self.r#north == North::Up
            && self.r#west == West::Side
            && self.r#south == South::Side
        {
            return 3814;
        }
        if self.r#south == South::None
            && self.r#north == North::Side
            && self.r#power == 12
            && self.r#west == West::Up
            && self.r#east == East::Side
        {
            return 4500;
        }
        if self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#power == 13
        {
            return 4791;
        }
        if self.r#north == North::None
            && self.r#power == 9
            && self.r#east == East::None
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 5044;
        }
        if self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#east == East::None
            && self.r#power == 4
            && self.r#south == South::None
        {
            return 4860;
        }
        if self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#power == 2
            && self.r#north == North::Side
        {
            return 3972;
        }
        if self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#power == 9
        {
            return 4614;
        }
        if self.r#north == North::None
            && self.r#west == West::Up
            && self.r#power == 9
            && self.r#east == East::Up
            && self.r#south == South::Side
        {
            return 4182;
        }
        if self.r#west == West::Up
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#power == 4
        {
            return 4716;
        }
        if self.r#east == East::Up
            && self.r#power == 11
            && self.r#north == North::Side
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 4061;
        }
        if self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#west == West::None
            && self.r#power == 3
        {
            return 4418;
        }
        if self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#power == 8
            && self.r#north == North::Side
            && self.r#south == South::Side
        {
            return 4461;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#power == 10
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 5054;
        }
        if self.r#south == South::None
            && self.r#north == North::Side
            && self.r#west == West::Up
            && self.r#power == 10
            && self.r#east == East::None
        {
            return 4914;
        }
        if self.r#east == East::Side
            && self.r#power == 14
            && self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#west == West::None
        {
            return 4370;
        }
        if self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#power == 8
            && self.r#west == West::None
        {
            return 4034;
        }
        if self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#power == 6
        {
            return 4591;
        }
        if self.r#west == West::Up
            && self.r#north == North::None
            && self.r#south == South::Side
            && self.r#power == 12
            && self.r#east == East::Up
        {
            return 4209;
        }
        if self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#power == 14
            && self.r#north == North::Up
            && self.r#east == East::Up
        {
            return 3939;
        }
        if self.r#south == South::Side
            && self.r#west == West::None
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#power == 15
        {
            return 3950;
        }
        if self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#power == 14
            && self.r#north == North::Side
            && self.r#west == West::None
        {
            return 4085;
        }
        if self.r#south == South::Side
            && self.r#west == West::None
            && self.r#east == East::Up
            && self.r#power == 9
            && self.r#north == North::Side
        {
            return 4040;
        }
        if self.r#power == 5
            && self.r#north == North::Up
            && self.r#west == West::Side
            && self.r#east == East::None
            && self.r#south == South::None
        {
            return 4726;
        }
        if self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#power == 8
        {
            return 4462;
        }
        if self.r#south == South::Up
            && self.r#west == West::None
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 8
        {
            return 4460;
        }
        if self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#power == 4
            && self.r#south == South::None
            && self.r#north == North::Side
        {
            return 3997;
        }
        if self.r#west == West::Side
            && self.r#power == 7
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#east == East::Up
        {
            return 4021;
        }
        if self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#east == East::None
            && self.r#power == 11
        {
            return 4777;
        }
        if self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#power == 4
            && self.r#south == South::None
            && self.r#east == East::Side
        {
            return 4429;
        }
        if self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#power == 10
            && self.r#south == South::None
            && self.r#west == West::Up
        {
            return 4338;
        }
        if self.r#north == North::Side
            && self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#power == 7
            && self.r#south == South::None
        {
            return 4455;
        }
        if self.r#east == East::Up
            && self.r#north == North::None
            && self.r#west == West::Side
            && self.r#south == South::Side
            && self.r#power == 12
        {
            return 4210;
        }
        if self.r#power == 7
            && self.r#east == East::None
            && self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#south == South::None
        {
            return 4743;
        }
        if self.r#west == West::None
            && self.r#east == East::Up
            && self.r#power == 15
            && self.r#south == South::Up
            && self.r#north == North::Up
        {
            return 3947;
        }
        if self.r#east == East::None
            && self.r#north == North::Up
            && self.r#power == 11
            && self.r#south == South::Side
            && self.r#west == West::None
        {
            return 4778;
        }
        if self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#power == 10
            && self.r#south == South::None
        {
            return 4339;
        }
        if self.r#east == East::Side
            && self.r#power == 0
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#north == North::Side
        {
            return 4389;
        }
        if self.r#west == West::Up
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 3
            && self.r#south == South::Up
        {
            return 4845;
        }
        if self.r#power == 11
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 4632;
        }
        if self.r#north == North::None
            && self.r#east == East::Side
            && self.r#power == 12
            && self.r#south == South::Side
            && self.r#west == West::Side
        {
            return 4642;
        }
        if self.r#power == 9
            && self.r#east == East::Up
            && self.r#west == West::Side
            && self.r#south == South::None
            && self.r#north == North::Side
        {
            return 4042;
        }
        if self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#power == 9
        {
            return 4467;
        }
        if self.r#east == East::None
            && self.r#power == 7
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::Up
        {
            return 4745;
        }
        if self.r#power == 14
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#south == South::Side
            && self.r#north == North::Side
        {
            return 4516;
        }
        if self.r#west == West::None
            && self.r#east == East::Side
            && self.r#power == 9
            && self.r#north == North::Up
            && self.r#south == South::Up
        {
            return 4325;
        }
        if self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#power == 8
            && self.r#south == South::None
            && self.r#west == West::Up
        {
            return 4032;
        }
        if self.r#power == 3
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 4997;
        }
        if self.r#east == East::Up
            && self.r#power == 10
            && self.r#north == North::None
            && self.r#south == South::Side
            && self.r#west == West::Side
        {
            return 4192;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#power == 12
            && self.r#west == West::Side
        {
            return 5071;
        }
        if self.r#power == 1
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#east == East::Up
        {
            return 3964;
        }
        if self.r#power == 5
            && self.r#south == South::Side
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Up
        {
            return 4148;
        }
        if self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#north == North::None
            && self.r#power == 3
            && self.r#west == West::Up
        {
            return 4128;
        }
        if self.r#north == North::Side
            && self.r#west == West::Up
            && self.r#south == South::Up
            && self.r#power == 12
            && self.r#east == East::Up
        {
            return 4062;
        }
        if self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#power == 9
            && self.r#north == North::Side
            && self.r#south == South::Side
        {
            return 4038;
        }
        if self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#west == West::None
            && self.r#north == North::Side
            && self.r#power == 13
        {
            return 4076;
        }
        if self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#power == 14
            && self.r#north == North::Up
        {
            return 4368;
        }
        if self.r#west == West::Side
            && self.r#power == 4
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#south == South::Side
        {
            return 4426;
        }
        if self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#power == 11
            && self.r#west == West::None
        {
            return 3911;
        }
        if self.r#north == North::None
            && self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#power == 5
        {
            return 4576;
        }
        if self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#west == West::Side
            && self.r#power == 13
        {
            return 4078;
        }
        if self.r#west == West::Side
            && self.r#power == 3
            && self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#east == East::Side
        {
            return 4414;
        }
        if self.r#north == North::None
            && self.r#west == West::Side
            && self.r#south == South::None
            && self.r#power == 8
            && self.r#east == East::Side
        {
            return 4609;
        }
        if self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#north == North::None
            && self.r#power == 0
            && self.r#south == South::None
        {
            return 4104;
        }
        if self.r#north == North::Up
            && self.r#south == South::Up
            && self.r#east == East::None
            && self.r#west == West::Up
            && self.r#power == 4
        {
            return 4710;
        }
        if self.r#south == South::Up
            && self.r#power == 7
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#west == West::Up
        {
            return 4881;
        }
        if self.r#south == South::None
            && self.r#power == 12
            && self.r#west == West::None
            && self.r#north == North::Side
            && self.r#east == East::Up
        {
            return 4070;
        }
        if self.r#power == 1
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#west == West::None
            && self.r#south == South::Up
        {
            return 4541;
        }
        if self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#power == 12
            && self.r#west == West::None
        {
            return 3923;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#power == 3
        {
            return 4995;
        }
        if self.r#power == 14
            && self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#west == West::Side
        {
            return 4084;
        }
        if self.r#west == West::Side
            && self.r#power == 11
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Up
        {
            return 5062;
        }
        if self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#east == East::None
            && self.r#power == 2
            && self.r#south == South::Up
        {
            return 4836;
        }
        if self.r#south == South::Up
            && self.r#power == 11
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#west == West::Up
        {
            return 4197;
        }
        if self.r#power == 7
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 5029;
        }
        if self.r#east == East::None
            && self.r#power == 11
            && self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#south == South::Up
        {
            return 4773;
        }
        if self.r#power == 11
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::Up
        {
            return 4917;
        }
        if self.r#south == South::Up
            && self.r#power == 13
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#north == North::None
        {
            return 4648;
        }
        if self.r#power == 2
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#south == South::Up
        {
            return 4116;
        }
        if self.r#power == 2
            && self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#west == West::None
        {
            return 4265;
        }
        if self.r#east == East::None
            && self.r#south == South::Side
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#power == 3
        {
            return 4850;
        }
        if self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#power == 5
            && self.r#north == North::Side
        {
            return 4435;
        }
        if self.r#south == South::Side
            && self.r#power == 1
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Up
        {
            return 4688;
        }
        if self.r#east == East::None
            && self.r#power == 7
            && self.r#west == West::Up
            && self.r#south == South::Up
            && self.r#north == North::None
        {
            return 5025;
        }
        if self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#power == 4
            && self.r#north == North::Side
            && self.r#east == East::Side
        {
            return 4422;
        }
        if self.r#power == 11
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#west == West::Side
            && self.r#north == North::Side
        {
            return 4924;
        }
        if self.r#power == 15
            && self.r#east == East::Up
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#south == South::Side
        {
            return 4236;
        }
        if self.r#west == West::Up
            && self.r#east == East::None
            && self.r#power == 6
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 5022;
        }
        if self.r#south == South::Up
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Side
            && self.r#power == 2
        {
            return 4981;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Side
            && self.r#south == South::None
            && self.r#power == 8
        {
            return 5041;
        }
        if self.r#north == North::Up
            && self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#west == West::Up
            && self.r#power == 6
        {
            return 4296;
        }
        if self.r#power == 0
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#east == East::Up
            && self.r#west == West::Side
        {
            return 3955;
        }
        if self.r#east == East::None
            && self.r#power == 13
            && self.r#west == West::Side
            && self.r#south == South::Side
            && self.r#north == North::None
        {
            return 5083;
        }
        if self.r#power == 13
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::Side
            && self.r#east == East::None
        {
            return 5086;
        }
        if self.r#power == 0
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#south == South::Side
        {
            return 4534;
        }
        if self.r#north == North::None
            && self.r#power == 11
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 5069;
        }
        if self.r#power == 8
            && self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#north == North::None
        {
            return 4171;
        }
        if self.r#north == North::None
            && self.r#power == 0
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#east == East::Up
        {
            return 4099;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#west == West::Side
            && self.r#power == 1
            && self.r#north == North::Side
        {
            return 4834;
        }
        if self.r#power == 11
            && self.r#north == North::Up
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#south == South::None
        {
            return 4348;
        }
        if self.r#west == West::Up
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#power == 14
            && self.r#south == South::Side
        {
            return 5091;
        }
        if self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#power == 3
            && self.r#east == East::Side
            && self.r#south == South::Side
        {
            return 4272;
        }
        if self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#power == 14
        {
            return 4227;
        }
        if self.r#south == South::Side
            && self.r#power == 13
            && self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#west == West::Up
        {
            return 3930;
        }
        if self.r#south == South::Side
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#power == 12
            && self.r#west == West::Side
        {
            return 5074;
        }
        if self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 11
            && self.r#west == West::None
            && self.r#south == South::Up
        {
            return 4487;
        }
        if self.r#north == North::Up
            && self.r#west == West::None
            && self.r#south == South::Up
            && self.r#east == East::None
            && self.r#power == 10
        {
            return 4766;
        }
        if self.r#north == North::Up
            && self.r#power == 6
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 4731;
        }
        if self.r#north == North::Up
            && self.r#power == 7
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#west == West::Side
        {
            return 4744;
        }
        if self.r#east == East::Up
            && self.r#power == 3
            && self.r#south == South::None
            && self.r#north == North::Side
            && self.r#west == West::Up
        {
            return 3987;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#west == West::Side
            && self.r#south == South::Up
            && self.r#power == 14
        {
            return 5089;
        }
        if self.r#east == East::Up
            && self.r#west == West::None
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#power == 8
        {
            return 4031;
        }
        if self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#power == 4
            && self.r#south == South::None
            && self.r#north == North::Up
        {
            return 3853;
        }
        if self.r#west == West::Up
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#power == 14
        {
            return 5088;
        }
        if self.r#north == North::Side
            && self.r#east == East::None
            && self.r#power == 7
            && self.r#west == West::Up
            && self.r#south == South::Side
        {
            return 4884;
        }
        if self.r#north == North::None
            && self.r#power == 4
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 5001;
        }
        if self.r#east == East::None
            && self.r#power == 6
            && self.r#south == South::None
            && self.r#north == North::Up
            && self.r#west == West::None
        {
            return 4736;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Side
            && self.r#power == 10
            && self.r#west == West::Up
        {
            return 5055;
        }
        if self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#power == 15
        {
            return 4961;
        }
        if self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#power == 14
            && self.r#west == West::Side
        {
            return 4081;
        }
        if self.r#power == 15
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#east == East::Side
        {
            return 4381;
        }
        if self.r#east == East::Up
            && self.r#west == West::None
            && self.r#power == 10
            && self.r#south == South::Up
            && self.r#north == North::None
        {
            return 4190;
        }
        if self.r#south == South::None
            && self.r#west == West::Side
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#power == 6
        {
            return 5023;
        }
        if self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 13
            && self.r#west == West::None
            && self.r#south == South::Up
        {
            return 4505;
        }
        if self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 14
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 4515;
        }
        if self.r#north == North::Up
            && self.r#power == 12
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#west == West::Side
        {
            return 4786;
        }
        if self.r#east == East::Side
            && self.r#power == 13
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#north == North::Up
        {
            return 4360;
        }
        if self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 7
            && self.r#south == South::Side
            && self.r#west == West::Side
        {
            return 4885;
        }
        if self.r#north == North::Up
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#power == 9
        {
            return 4760;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#power == 7
            && self.r#north == North::None
            && self.r#east == East::None
        {
            return 5033;
        }
        if self.r#east == East::Side
            && self.r#power == 0
            && self.r#west == West::Up
            && self.r#south == South::None
            && self.r#north == North::Up
        {
            return 4248;
        }
        if self.r#east == East::Up
            && self.r#west == West::Side
            && self.r#power == 3
            && self.r#south == South::Up
            && self.r#north == North::Side
        {
            return 3982;
        }
        if self.r#east == East::None
            && self.r#power == 10
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#north == North::Side
        {
            return 4908;
        }
        if self.r#south == South::Side
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#power == 13
            && self.r#west == West::Up
        {
            return 5082;
        }
        if self.r#east == East::Side
            && self.r#power == 8
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#south == South::Up
        {
            return 4315;
        }
        if self.r#power == 3
            && self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#south == South::None
        {
            return 3988;
        }
        if self.r#north == North::None
            && self.r#east == East::Up
            && self.r#power == 8
            && self.r#south == South::None
            && self.r#west == West::Up
        {
            return 4176;
        }
        if self.r#south == South::Side
            && self.r#north == North::Side
            && self.r#west == West::Up
            && self.r#power == 11
            && self.r#east == East::None
        {
            return 4920;
        }
        if self.r#north == North::Side
            && self.r#east == East::None
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#power == 14
        {
            return 4944;
        }
        if self.r#north == North::None
            && self.r#east == East::Up
            && self.r#power == 15
            && self.r#west == West::Up
            && self.r#south == South::None
        {
            return 4239;
        }
        if self.r#east == East::None
            && self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#power == 14
            && self.r#west == West::Side
        {
            return 4804;
        }
        if self.r#south == South::Side
            && self.r#north == North::Side
            && self.r#east == East::None
            && self.r#power == 6
            && self.r#west == West::Side
        {
            return 4876;
        }
        if self.r#power == 7
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#east == East::Up
        {
            return 3876;
        }
        if self.r#west == West::Side
            && self.r#power == 15
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#south == South::Up
        {
            return 4666;
        }
        if self.r#south == South::Up
            && self.r#north == North::None
            && self.r#west == West::Side
            && self.r#power == 4
            && self.r#east == East::Up
        {
            return 4135;
        }
        if self.r#power == 11
            && self.r#west == West::Side
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#south == South::Up
        {
            return 4630;
        }
        if self.r#east == East::Up
            && self.r#power == 0
            && self.r#north == North::Side
            && self.r#west == West::Side
            && self.r#south == South::Side
        {
            return 3958;
        }
        if self.r#north == North::None
            && self.r#power == 10
            && self.r#east == East::Up
            && self.r#west == West::Side
            && self.r#south == South::None
        {
            return 4195;
        }
        if self.r#east == East::None
            && self.r#power == 12
            && self.r#north == North::Up
            && self.r#west == West::Side
            && self.r#south == South::Up
        {
            return 4783;
        }
        if self.r#power == 14
            && self.r#south == South::None
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#east == East::Up
        {
            return 4088;
        }
        if self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#power == 8
            && self.r#west == West::Up
            && self.r#south == South::Up
        {
            return 4026;
        }
        if self.r#power == 4
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#north == North::Up
        {
            return 3846;
        }
        if self.r#power == 0
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 4531;
        }
        if self.r#north == North::Side
            && self.r#power == 2
            && self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#west == West::None
        {
            return 4409;
        }
        if self.r#power == 5
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#north == North::Side
        {
            return 4002;
        }
        if self.r#south == South::Side
            && self.r#west == West::None
            && self.r#power == 5
            && self.r#north == North::Side
            && self.r#east == East::Up
        {
            return 4004;
        }
        if self.r#south == South::Up
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#power == 10
        {
            return 5052;
        }
        if self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#power == 13
            && self.r#south == South::Side
            && self.r#north == North::Side
        {
            return 4075;
        }
        if self.r#east == East::Side
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#south == South::Up
            && self.r#power == 4
        {
            return 4568;
        }
        if self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::Up
            && self.r#power == 9
            && self.r#north == North::None
        {
            return 5045;
        }
        if self.r#south == South::Side
            && self.r#power == 15
            && self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#west == West::None
        {
            return 4094;
        }
        if self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#power == 10
            && self.r#north == North::Up
            && self.r#west == West::Side
        {
            return 4336;
        }
        if self.r#west == West::None
            && self.r#power == 2
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#north == North::Up
        {
            return 3833;
        }
        if self.r#power == 9
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#west == West::None
        {
            return 3896;
        }
        if self.r#east == East::Up
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#south == South::None
            && self.r#power == 6
        {
            return 4158;
        }
        if self.r#west == West::Up
            && self.r#north == North::None
            && self.r#power == 12
            && self.r#east == East::Up
            && self.r#south == South::Up
        {
            return 4206;
        }
        if self.r#power == 5
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#west == West::Side
        {
            return 4000;
        }
        if self.r#north == North::Side
            && self.r#east == East::None
            && self.r#power == 4
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 4857;
        }
        if self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#north == North::Side
            && self.r#power == 12
            && self.r#west == West::Up
        {
            return 4065;
        }
        if self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 13
            && self.r#west == West::Side
            && self.r#south == South::None
        {
            return 4510;
        }
        if self.r#east == East::Up
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#power == 15
            && self.r#south == South::Up
        {
            return 3946;
        }
        if self.r#south == South::Up
            && self.r#power == 4
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#east == East::Up
        {
            return 3992;
        }
        if self.r#west == West::Up
            && self.r#power == 4
            && self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#south == South::Up
        {
            return 3990;
        }
        if self.r#west == West::Side
            && self.r#power == 0
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#south == South::None
        {
            return 4681;
        }
        if self.r#south == South::Up
            && self.r#east == East::Up
            && self.r#west == West::None
            && self.r#north == North::Up
            && self.r#power == 2
        {
            return 3830;
        }
        if self.r#north == North::None
            && self.r#power == 11
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#east == East::Up
        {
            return 4199;
        }
        if self.r#east == East::None
            && self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#power == 12
            && self.r#south == South::Up
        {
            return 4927;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::Side
            && self.r#power == 6
            && self.r#west == West::Up
        {
            return 4590;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#power == 1
            && self.r#north == North::Up
            && self.r#east == East::None
        {
            return 4691;
        }
        if self.r#power == 5
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Side
        {
            return 4871;
        }
        if self.r#east == East::Up
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#power == 2
            && self.r#south == South::Up
        {
            return 4118;
        }
        if self.r#power == 11
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#east == East::Up
        {
            return 3912;
        }
        if self.r#north == North::Up
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#power == 14
            && self.r#east == East::Up
        {
            return 3944;
        }
        if self.r#west == West::Up
            && self.r#south == South::Up
            && self.r#north == North::None
            && self.r#power == 7
            && self.r#east == East::Side
        {
            return 4593;
        }
        if self.r#power == 1
            && self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 3822;
        }
        if self.r#east == East::None
            && self.r#north == North::Up
            && self.r#power == 0
            && self.r#west == West::Side
            && self.r#south == South::Side
        {
            return 4678;
        }
        if self.r#power == 8
            && self.r#west == West::None
            && self.r#north == North::Up
            && self.r#south == South::Up
            && self.r#east == East::None
        {
            return 4748;
        }
        if self.r#south == South::None
            && self.r#north == North::Side
            && self.r#power == 4
            && self.r#west == West::None
            && self.r#east == East::None
        {
            return 4862;
        }
        if self.r#power == 9
            && self.r#east == East::None
            && self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#south == South::Up
        {
            return 4900;
        }
        if self.r#power == 14
            && self.r#west == West::Side
            && self.r#south == South::Up
            && self.r#east == East::Up
            && self.r#north == North::Up
        {
            return 3937;
        }
        if self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#power == 0
            && self.r#west == West::Up
        {
            return 3813;
        }
        if self.r#south == South::Up
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 3
            && self.r#west == West::Side
        {
            return 4846;
        }
        if self.r#south == South::None
            && self.r#west == West::Side
            && self.r#north == North::None
            && self.r#power == 2
            && self.r#east == East::None
        {
            return 4987;
        }
        if self.r#north == North::Side
            && self.r#east == East::None
            && self.r#power == 8
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 4898;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#power == 5
            && self.r#south == South::None
        {
            return 5013;
        }
        if self.r#south == South::Side
            && self.r#power == 7
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Up
        {
            return 5028;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#power == 8
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 5042;
        }
        if self.r#north == North::None
            && self.r#power == 13
            && self.r#east == East::None
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 5081;
        }
        if self.r#power == 14
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#west == West::Up
        {
            return 4224;
        }
        if self.r#north == North::Side
            && self.r#power == 2
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#west == West::None
        {
            return 3977;
        }
        if self.r#east == East::Up
            && self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#power == 3
            && self.r#south == South::Side
        {
            return 3840;
        }
        if self.r#west == West::None
            && self.r#power == 11
            && self.r#south == South::None
            && self.r#north == North::Side
            && self.r#east == East::Side
        {
            return 4493;
        }
        if self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#east == East::None
            && self.r#power == 11
            && self.r#south == South::Up
        {
            return 4918;
        }
        if self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#power == 15
            && self.r#west == West::Up
            && self.r#south == South::Side
        {
            return 4524;
        }
        if self.r#power == 2
            && self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#south == South::Side
        {
            return 4695;
        }
        if self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#power == 15
        {
            return 4097;
        }
        if self.r#south == South::Side
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#west == West::None
            && self.r#power == 12
        {
            return 4211;
        }
        if self.r#south == South::Up
            && self.r#power == 11
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 5063;
        }
        if self.r#power == 4
            && self.r#east == East::None
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#north == North::Side
        {
            return 4855;
        }
        if self.r#south == South::Up
            && self.r#power == 4
            && self.r#west == West::None
            && self.r#north == North::Side
            && self.r#east == East::Side
        {
            return 4424;
        }
        if self.r#east == East::None
            && self.r#south == South::Side
            && self.r#power == 7
            && self.r#west == West::Side
            && self.r#north == North::Up
        {
            return 4741;
        }
        if self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#power == 13
            && self.r#east == East::Up
        {
            return 4218;
        }
        if self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#power == 0
            && self.r#west == West::Side
        {
            return 4390;
        }
        if self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 12
        {
            return 4926;
        }
        if self.r#east == East::Side
            && self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#south == South::None
            && self.r#power == 3
        {
            return 4275;
        }
        if self.r#west == West::Side
            && self.r#north == North::None
            && self.r#power == 15
            && self.r#south == South::None
            && self.r#east == East::None
        {
            return 5104;
        }
        if self.r#power == 13
            && self.r#north == North::Up
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::Up
        {
            return 4793;
        }
        if self.r#north == North::Side
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#power == 15
        {
            return 4956;
        }
        if self.r#west == West::None
            && self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#power == 11
            && self.r#south == South::Side
        {
            return 4058;
        }
        if self.r#west == West::Up
            && self.r#east == East::None
            && self.r#power == 10
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 5058;
        }
        if self.r#north == North::Side
            && self.r#power == 14
            && self.r#west == West::Up
            && self.r#south == South::Side
            && self.r#east == East::Up
        {
            return 4083;
        }
        if self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 13
            && self.r#south == South::None
            && self.r#west == West::Up
        {
            return 4509;
        }
        if self.r#north == North::Side
            && self.r#power == 10
            && self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#south == South::Side
        {
            return 4048;
        }
        if self.r#south == South::None
            && self.r#power == 13
            && self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#north == North::Up
        {
            return 4365;
        }
        if self.r#north == North::Side
            && self.r#west == West::None
            && self.r#south == South::Side
            && self.r#east == East::None
            && self.r#power == 15
        {
            return 4958;
        }
        if self.r#west == West::None
            && self.r#power == 14
            && self.r#north == North::Side
            && self.r#east == East::None
            && self.r#south == South::Up
        {
            return 4946;
        }
        if self.r#south == South::Side
            && self.r#west == West::None
            && self.r#north == North::Side
            && self.r#power == 10
            && self.r#east == East::Up
        {
            return 4049;
        }
        if self.r#north == North::Side
            && self.r#power == 14
            && self.r#south == South::Side
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 4949;
        }
        if self.r#north == North::None
            && self.r#power == 4
            && self.r#east == East::None
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 5000;
        }
        if self.r#north == North::None
            && self.r#west == West::None
            && self.r#power == 14
            && self.r#south == South::Side
            && self.r#east == East::Up
        {
            return 4229;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#power == 14
            && self.r#north == North::Up
            && self.r#east == East::Side
        {
            return 4376;
        }
        if self.r#north == North::Side
            && self.r#power == 7
            && self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#west == West::Up
        {
            return 4449;
        }
        if self.r#east == East::Up
            && self.r#south == South::None
            && self.r#north == North::Side
            && self.r#west == West::Side
            && self.r#power == 5
        {
            return 4006;
        }
        if self.r#south == South::Side
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#power == 10
        {
            return 4193;
        }
        if self.r#power == 13
            && self.r#west == West::None
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#north == North::None
        {
            return 4217;
        }
        if self.r#east == East::Side
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#power == 2
        {
            return 4550;
        }
        if self.r#north == North::None
            && self.r#east == East::Side
            && self.r#power == 8
            && self.r#south == South::None
            && self.r#west == West::Up
        {
            return 4608;
        }
        if self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#east == East::Up
            && self.r#power == 13
        {
            return 4071;
        }
        if self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#power == 3
            && self.r#west == West::Side
            && self.r#north == North::Side
        {
            return 3985;
        }
        if self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#power == 12
        {
            return 4787;
        }
        if self.r#north == North::None
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#power == 4
            && self.r#east == East::Up
        {
            return 4137;
        }
        if self.r#north == North::Side
            && self.r#east == East::None
            && self.r#power == 10
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 4916;
        }
        if self.r#north == North::Side
            && self.r#power == 13
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#west == West::None
        {
            return 4940;
        }
        if self.r#south == South::Side
            && self.r#east == East::None
            && self.r#power == 13
            && self.r#west == West::Up
            && self.r#north == North::Up
        {
            return 4794;
        }
        if self.r#east == East::Up
            && self.r#south == South::None
            && self.r#power == 0
            && self.r#west == West::Side
            && self.r#north == North::Side
        {
            return 3961;
        }
        if self.r#south == South::Side
            && self.r#power == 2
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::None
        {
            return 4985;
        }
        if self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#power == 11
            && self.r#north == North::None
            && self.r#south == South::Side
        {
            return 4201;
        }
        if self.r#power == 9
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 4468;
        }
        if self.r#east == East::None
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#south == South::Up
            && self.r#power == 1
        {
            return 4684;
        }
        if self.r#west == West::Up
            && self.r#power == 0
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#east == East::Up
        {
            return 4098;
        }
        if self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#power == 3
        {
            return 4270;
        }
        if self.r#power == 14
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#east == East::Side
        {
            return 4513;
        }
        if self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#power == 14
            && self.r#south == South::Up
            && self.r#north == North::Side
        {
            return 4512;
        }
        if self.r#north == North::Up
            && self.r#west == West::Side
            && self.r#power == 9
            && self.r#south == South::None
            && self.r#east == East::Up
        {
            return 3898;
        }
        if self.r#south == South::Side
            && self.r#power == 9
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#north == North::Side
        {
            return 4902;
        }
        if self.r#east == East::None
            && self.r#power == 1
            && self.r#west == West::Side
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 4978;
        }
        if self.r#south == South::Up
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#power == 15
            && self.r#west == West::None
        {
            return 4235;
        }
        if self.r#power == 10
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::Up
            && self.r#north == North::Side
        {
            return 4052;
        }
        if self.r#south == South::None
            && self.r#east == East::Side
            && self.r#power == 12
            && self.r#north == North::Side
            && self.r#west == West::None
        {
            return 4502;
        }
        if self.r#north == North::Up
            && self.r#power == 11
            && self.r#south == South::Up
            && self.r#east == East::Up
            && self.r#west == West::Up
        {
            return 3909;
        }
        if self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#power == 12
            && self.r#west == West::Up
        {
            return 4497;
        }
        if self.r#power == 8
            && self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#west == West::Up
        {
            return 4602;
        }
        if self.r#west == West::None
            && self.r#north == North::Up
            && self.r#power == 2
            && self.r#east == East::None
            && self.r#south == South::Side
        {
            return 4697;
        }
        if self.r#power == 1
            && self.r#south == South::None
            && self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#west == West::Up
        {
            return 4257;
        }
        if self.r#north == North::Side
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#power == 9
            && self.r#west == West::None
        {
            return 4907;
        }
        if self.r#east == East::None
            && self.r#power == 12
            && self.r#west == West::None
            && self.r#south == South::Side
            && self.r#north == North::None
        {
            return 5075;
        }
        if self.r#power == 4
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#east == East::Side
        {
            return 4282;
        }
        if self.r#west == West::Up
            && self.r#south == South::Up
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#power == 14
        {
            return 3936;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#power == 0
            && self.r#south == South::Up
            && self.r#west == West::Up
        {
            return 4962;
        }
        if self.r#east == East::None
            && self.r#power == 1
            && self.r#west == West::Side
            && self.r#south == South::Side
            && self.r#north == North::Up
        {
            return 4687;
        }
        if self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#power == 6
        {
            return 4300;
        }
        if self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#power == 14
        {
            return 4371;
        }
        if self.r#power == 5
            && self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#north == North::None
            && self.r#west == West::Side
        {
            return 4579;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#power == 5
        {
            return 5015;
        }
        if self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#power == 10
            && self.r#north == North::Side
            && self.r#west == West::None
        {
            return 4478;
        }
        if self.r#west == West::Side
            && self.r#power == 4
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::Up
        {
            return 4141;
        }
        if self.r#south == South::None
            && self.r#west == West::Side
            && self.r#power == 11
            && self.r#east == East::Side
            && self.r#north == North::None
        {
            return 4636;
        }
        if self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#power == 15
        {
            return 4383;
        }
        if self.r#east == East::Side
            && self.r#power == 10
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 4479;
        }
        if self.r#west == West::Up
            && self.r#power == 0
            && self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#north == North::None
        {
            return 4530;
        }
        if self.r#power == 14
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::Up
            && self.r#north == North::Up
        {
            return 4802;
        }
        if self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#power == 6
            && self.r#south == South::None
        {
            return 4302;
        }
        if self.r#south == South::Up
            && self.r#power == 3
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 4991;
        }
        if self.r#east == East::Up
            && self.r#power == 14
            && self.r#south == South::None
            && self.r#west == West::Side
            && self.r#north == North::Side
        {
            return 4087;
        }
        if self.r#west == West::Side
            && self.r#power == 0
            && self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#north == North::None
        {
            return 4102;
        }
        if self.r#north == North::Side
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#power == 7
        {
            return 4887;
        }
        if self.r#east == East::None
            && self.r#power == 13
            && self.r#south == South::Side
            && self.r#west == West::None
            && self.r#north == North::Up
        {
            return 4796;
        }
        if self.r#west == West::Up
            && self.r#south == South::Up
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#power == 4
        {
            return 4566;
        }
        if self.r#south == South::Up
            && self.r#power == 13
            && self.r#east == East::Up
            && self.r#north == North::None
            && self.r#west == West::Up
        {
            return 4215;
        }
        if self.r#power == 3
            && self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#west == West::None
        {
            return 4415;
        }
        if self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#power == 0
            && self.r#east == East::None
            && self.r#south == South::Side
        {
            return 4822;
        }
        if self.r#power == 7
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#north == North::Side
            && self.r#west == West::None
        {
            return 4025;
        }
        if self.r#east == East::Up
            && self.r#power == 6
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#north == North::Up
        {
            return 3867;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::Up
            && self.r#north == North::None
            && self.r#power == 10
        {
            return 4196;
        }
        if self.r#south == South::Up
            && self.r#west == West::None
            && self.r#power == 2
            && self.r#north == North::Side
            && self.r#east == East::Up
        {
            return 3974;
        }
        if self.r#north == North::None
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#power == 5
        {
            return 4149;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#power == 4
        {
            return 4573;
        }
        if self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#power == 14
            && self.r#east == East::Up
        {
            return 3940;
        }
        if self.r#east == East::Up
            && self.r#power == 15
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::None
        {
            return 4241;
        }
        if self.r#south == South::None
            && self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#east == East::None
            && self.r#power == 9
        {
            return 4761;
        }
        if self.r#power == 0
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#east == East::None
        {
            return 4818;
        }
        if self.r#power == 6
            && self.r#east == East::None
            && self.r#south == South::Up
            && self.r#north == North::None
            && self.r#west == West::Up
        {
            return 5016;
        }
        if self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#east == East::None
            && self.r#power == 14
            && self.r#west == West::Side
        {
            return 4945;
        }
        if self.r#north == North::Up
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#power == 0
            && self.r#east == East::None
        {
            return 4676;
        }
        if self.r#west == West::None
            && self.r#north == North::Side
            && self.r#power == 5
            && self.r#south == South::Up
            && self.r#east == East::None
        {
            return 4865;
        }
        if self.r#power == 12
            && self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#west == West::None
            && self.r#south == South::Up
        {
            return 4064;
        }
        if self.r#east == East::Up
            && self.r#north == North::None
            && self.r#power == 1
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 4109;
        }
        if self.r#west == West::None
            && self.r#east == East::Side
            && self.r#power == 12
            && self.r#south == South::Side
            && self.r#north == North::Side
        {
            return 4499;
        }
        if self.r#east == East::Side
            && self.r#power == 15
            && self.r#south == South::Side
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 4670;
        }
        if self.r#south == South::None
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#power == 8
            && self.r#west == West::Up
        {
            return 4320;
        }
        if self.r#north == North::None
            && self.r#east == East::Side
            && self.r#west == West::None
            && self.r#south == South::Side
            && self.r#power == 0
        {
            return 4535;
        }
        if self.r#east == East::None
            && self.r#north == North::Up
            && self.r#power == 8
            && self.r#west == West::Side
            && self.r#south == South::Side
        {
            return 4750;
        }
        if self.r#north == North::Up
            && self.r#power == 1
            && self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#south == South::Up
        {
            return 4251;
        }
        if self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#north == North::Side
            && self.r#power == 6
        {
            return 4442;
        }
        if self.r#west == West::Up
            && self.r#power == 2
            && self.r#south == South::Up
            && self.r#east == East::None
            && self.r#north == North::Up
        {
            return 4692;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#power == 3
            && self.r#south == South::Side
        {
            return 4992;
        }
        if self.r#power == 5
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::Up
            && self.r#north == North::None
        {
            return 5009;
        }
        if self.r#south == South::None
            && self.r#north == North::Up
            && self.r#power == 5
            && self.r#east == East::Side
            && self.r#west == West::Up
        {
            return 4293;
        }
        if self.r#power == 9
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#west == West::Side
            && self.r#east == East::Side
        {
            return 4618;
        }
        if self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#power == 15
            && self.r#south == South::Side
            && self.r#north == North::None
        {
            return 4669;
        }
        if self.r#east == East::Up
            && self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#power == 7
            && self.r#south == South::None
        {
            return 4024;
        }
        if self.r#north == North::Up
            && self.r#power == 0
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::Up
        {
            return 3818;
        }
        if self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#power == 12
            && self.r#south == South::None
            && self.r#west == West::Side
        {
            return 4357;
        }
        if self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#power == 15
            && self.r#east == East::Side
            && self.r#south == South::Side
        {
            return 4380;
        }
        if self.r#east == East::Side
            && self.r#power == 6
            && self.r#south == South::Side
            && self.r#north == North::None
            && self.r#west == West::Side
        {
            return 4588;
        }
        if self.r#north == North::None
            && self.r#south == South::Up
            && self.r#east == East::Up
            && self.r#power == 10
            && self.r#west == West::Side
        {
            return 4189;
        }
        if self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#power == 5
            && self.r#east == East::Side
        {
            return 4291;
        }
        if self.r#power == 7
            && self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#east == East::Side
        {
            return 4451;
        }
        if self.r#east == East::None
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#power == 2
        {
            return 4840;
        }
        if self.r#west == West::None
            && self.r#north == North::Up
            && self.r#south == South::None
            && self.r#power == 4
            && self.r#east == East::Side
        {
            return 4286;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#power == 15
            && self.r#west == West::None
        {
            return 5105;
        }
        if self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#power == 9
            && self.r#west == West::Side
        {
            return 4039;
        }
        if self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#power == 7
        {
            return 4017;
        }
        if self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#power == 12
        {
            return 4494;
        }
        if self.r#east == East::Up
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#south == South::Side
            && self.r#power == 7
        {
            return 4164;
        }
        if self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#power == 7
            && self.r#east == East::Side
            && self.r#south == South::Up
        {
            return 4305;
        }
        if self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#power == 13
            && self.r#north == North::Up
            && self.r#south == South::None
        {
            return 4366;
        }
        if self.r#south == South::Up
            && self.r#north == North::None
            && self.r#power == 11
            && self.r#west == West::Side
            && self.r#east == East::Up
        {
            return 4198;
        }
        if self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#power == 0
        {
            return 4537;
        }
        if self.r#north == North::None
            && self.r#south == South::Up
            && self.r#power == 1
            && self.r#west == West::Up
            && self.r#east == East::Side
        {
            return 4539;
        }
        if self.r#power == 8
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#west == West::Side
            && self.r#south == South::Side
        {
            return 4606;
        }
        if self.r#east == East::Side
            && self.r#south == South::None
            && self.r#power == 13
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 4655;
        }
        if self.r#north == North::None
            && self.r#east == East::Side
            && self.r#power == 12
            && self.r#south == South::None
            && self.r#west == West::Side
        {
            return 4645;
        }
        if self.r#power == 14
            && self.r#west == West::None
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#north == North::Up
        {
            return 3941;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#power == 12
            && self.r#north == North::Up
            && self.r#east == East::Up
        {
            return 3926;
        }
        if self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#west == West::Side
            && self.r#south == South::Side
            && self.r#power == 7
        {
            return 4453;
        }
        if self.r#north == North::Side
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#power == 3
            && self.r#east == East::None
        {
            return 4851;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#power == 10
            && self.r#west == West::Side
            && self.r#east == East::None
        {
            return 5059;
        }
        if self.r#south == South::Side
            && self.r#power == 2
            && self.r#north == North::Up
            && self.r#west == West::Side
            && self.r#east == East::Up
        {
            return 3832;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::Up
            && self.r#power == 1
            && self.r#north == North::Side
        {
            return 3971;
        }
        if self.r#west == West::None
            && self.r#power == 8
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#north == North::Up
        {
            return 3884;
        }
        if self.r#power == 6
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#east == East::Up
        {
            return 4011;
        }
        if self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#power == 7
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 4306;
        }
        if self.r#west == West::None
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#power == 7
            && self.r#east == East::Side
        {
            return 4454;
        }
        if self.r#north == North::None
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#power == 11
            && self.r#south == South::Side
        {
            return 5066;
        }
        if self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#power == 12
            && self.r#north == North::Side
            && self.r#south == South::Side
        {
            return 4066;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::Up
            && self.r#west == West::None
            && self.r#power == 13
        {
            return 4223;
        }
        if self.r#south == South::None
            && self.r#power == 2
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#north == North::Side
        {
            return 4411;
        }
        if self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#power == 10
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 4627;
        }
        if self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 6
            && self.r#south == South::None
            && self.r#west == West::Up
        {
            return 4878;
        }
        if self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#power == 13
        {
            return 3927;
        }
        if self.r#east == East::Side
            && self.r#power == 4
            && self.r#west == West::Side
            && self.r#south == South::None
            && self.r#north == North::Up
        {
            return 4285;
        }
        if self.r#power == 12
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#west == West::None
            && self.r#south == South::Side
        {
            return 4355;
        }
        if self.r#west == West::Up
            && self.r#north == North::None
            && self.r#power == 10
            && self.r#south == South::Up
            && self.r#east == East::Up
        {
            return 4188;
        }
        if self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#power == 1
            && self.r#south == South::None
        {
            return 3969;
        }
        if self.r#power == 5
            && self.r#east == East::Side
            && self.r#south == South::None
            && self.r#west == West::Side
            && self.r#north == North::Side
        {
            return 4438;
        }
        if self.r#west == West::Up
            && self.r#power == 0
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#east == East::None
        {
            return 4821;
        }
        if self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#power == 7
            && self.r#north == North::Up
            && self.r#south == South::Side
        {
            return 4308;
        }
        if self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#power == 13
            && self.r#west == West::Up
            && self.r#east == East::Side
        {
            return 4362;
        }
        if self.r#west == West::Side
            && self.r#south == South::Up
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#power == 7
        {
            return 5026;
        }
        if self.r#north == North::Side
            && self.r#power == 0
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#east == East::Side
        {
            return 4392;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 7
        {
            return 4457;
        }
        if self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#power == 1
            && self.r#south == South::None
            && self.r#east == East::None
        {
            return 4689;
        }
        if self.r#north == North::Side
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#power == 8
            && self.r#south == South::Up
        {
            return 4459;
        }
        if self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#power == 3
            && self.r#north == North::None
        {
            return 4557;
        }
        if self.r#north == North::None
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#power == 12
        {
            return 4641;
        }
        if self.r#south == South::None
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#power == 6
            && self.r#east == East::None
        {
            return 4735;
        }
        if self.r#south == South::None
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#power == 5
            && self.r#west == West::None
        {
            return 4583;
        }
        if self.r#north == North::Up
            && self.r#power == 11
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#south == South::Up
        {
            return 4342;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#power == 6
        {
            return 4592;
        }
        if self.r#north == North::Side
            && self.r#east == East::None
            && self.r#west == West::Side
            && self.r#south == South::Up
            && self.r#power == 5
        {
            return 4864;
        }
        if self.r#north == North::Side
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#power == 13
            && self.r#east == East::None
        {
            return 4941;
        }
        if self.r#north == North::None
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#south == South::Up
            && self.r#power == 9
        {
            return 5043;
        }
        if self.r#north == North::Up
            && self.r#power == 11
            && self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#west == West::Up
        {
            return 4341;
        }
        if self.r#south == South::None
            && self.r#west == West::Up
            && self.r#power == 14
            && self.r#east == East::Side
            && self.r#north == North::None
        {
            return 4662;
        }
        if self.r#south == South::None
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 10
        {
            return 4483;
        }
        if self.r#south == South::None
            && self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#power == 7
            && self.r#west == West::Up
        {
            return 4023;
        }
        if self.r#power == 11
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 4631;
        }
        if self.r#east == East::Up
            && self.r#west == West::Up
            && self.r#power == 5
            && self.r#north == North::Side
            && self.r#south == South::None
        {
            return 4005;
        }
        if self.r#power == 12
            && self.r#east == East::Side
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 4646;
        }
        if self.r#west == West::Up
            && self.r#south == South::None
            && self.r#power == 12
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 5076;
        }
        if self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#power == 14
            && self.r#south == South::None
            && self.r#east == East::Up
        {
            return 3942;
        }
        if self.r#power == 9
            && self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#north == North::Up
        {
            return 4324;
        }
        if self.r#east == East::Up
            && self.r#west == West::Up
            && self.r#power == 11
            && self.r#north == North::Side
            && self.r#south == South::Up
        {
            return 4053;
        }
        if self.r#north == North::None
            && self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#power == 5
        {
            return 4578;
        }
        if self.r#east == East::None
            && self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#power == 10
            && self.r#south == South::None
        {
            return 4770;
        }
        if self.r#power == 12
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#south == South::Up
        {
            return 5070;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#power == 0
        {
            return 3962;
        }
        if self.r#west == West::Side
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::Up
            && self.r#power == 13
        {
            return 5080;
        }
        if self.r#north == North::Up
            && self.r#power == 0
            && self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 4245;
        }
        if self.r#north == North::Side
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#power == 14
            && self.r#west == West::Side
        {
            return 4951;
        }
        if self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#power == 8
            && self.r#west == West::Side
            && self.r#north == North::None
        {
            return 4603;
        }
        if self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#power == 15
            && self.r#north == North::Side
            && self.r#south == South::None
        {
            return 4527;
        }
        if self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#power == 9
            && self.r#west == West::Side
            && self.r#south == South::None
        {
            return 4474;
        }
        if self.r#power == 13
            && self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#north == North::Side
        {
            return 4074;
        }
        if self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#power == 13
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 3935;
        }
        if self.r#power == 11
            && self.r#south == South::Side
            && self.r#east == East::None
            && self.r#west == West::Up
            && self.r#north == North::Up
        {
            return 4776;
        }
        if self.r#north == North::Up
            && self.r#west == West::None
            && self.r#power == 9
            && self.r#east == East::Side
            && self.r#south == South::None
        {
            return 4331;
        }
        if self.r#north == North::Up
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#power == 0
            && self.r#south == South::Side
        {
            return 4246;
        }
        if self.r#east == East::Side
            && self.r#power == 6
            && self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#south == South::Side
        {
            return 4443;
        }
        if self.r#south == South::Up
            && self.r#power == 10
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#west == West::Side
        {
            return 4765;
        }
        if self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#power == 4
        {
            return 4425;
        }
        if self.r#east == East::None
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#power == 8
            && self.r#south == South::Side
        {
            return 4895;
        }
        if self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#power == 1
            && self.r#west == West::None
            && self.r#east == East::Up
        {
            return 3824;
        }
        if self.r#power == 13
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#east == East::Side
        {
            return 4359;
        }
        if self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#power == 11
        {
            return 5067;
        }
        if self.r#power == 0
            && self.r#west == West::Up
            && self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#east == East::None
        {
            return 4674;
        }
        if self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#power == 15
        {
            return 4522;
        }
        if self.r#north == North::Side
            && self.r#power == 10
            && self.r#east == East::None
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 4909;
        }
        if self.r#north == North::Up
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#power == 2
        {
            return 4696;
        }
        if self.r#power == 1
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#west == West::Side
        {
            return 4114;
        }
        if self.r#east == East::None
            && self.r#power == 15
            && self.r#north == North::Up
            && self.r#south == South::Up
            && self.r#west == West::Up
        {
            return 4809;
        }
        if self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#power == 11
            && self.r#west == West::Side
            && self.r#south == South::Side
        {
            return 3913;
        }
        if self.r#north == North::Up
            && self.r#south == South::None
            && self.r#power == 7
            && self.r#west == West::Up
            && self.r#east == East::Up
        {
            return 3879;
        }
        if self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#power == 7
            && self.r#east == East::Up
        {
            return 4020;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Side
            && self.r#power == 9
            && self.r#south == South::None
        {
            return 5050;
        }
        if self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#power == 5
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 3857;
        }
        if self.r#power == 6
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#east == East::Up
        {
            return 4009;
        }
        if self.r#east == East::Up
            && self.r#power == 7
            && self.r#west == West::None
            && self.r#north == North::Up
            && self.r#south == South::Up
        {
            return 3875;
        }
        if self.r#north == North::Up
            && self.r#power == 6
            && self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#west == West::None
        {
            return 4301;
        }
        if self.r#east == East::Side
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#power == 3
            && self.r#west == West::Side
        {
            return 4564;
        }
        if self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#power == 13
            && self.r#west == West::Side
            && self.r#north == North::Up
        {
            return 3931;
        }
        if self.r#north == North::None
            && self.r#south == South::Up
            && self.r#power == 2
            && self.r#east == East::Up
            && self.r#west == West::Side
        {
            return 4117;
        }
        if self.r#north == North::Up
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#power == 0
            && self.r#west == West::Up
        {
            return 4680;
        }
        if self.r#north == North::Side
            && self.r#west == West::None
            && self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#power == 13
        {
            return 4508;
        }
        if self.r#power == 14
            && self.r#west == West::Side
            && self.r#south == South::None
            && self.r#north == North::Side
            && self.r#east == East::Side
        {
            return 4519;
        }
        if self.r#east == East::Side
            && self.r#west == West::None
            && self.r#power == 8
            && self.r#south == South::Up
            && self.r#north == North::None
        {
            return 4604;
        }
        if self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#power == 5
        {
            return 4722;
        }
        if self.r#east == East::Side
            && self.r#power == 10
            && self.r#north == North::Up
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 4340;
        }
        if self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#power == 11
            && self.r#south == South::Side
            && self.r#west == West::Side
        {
            return 4345;
        }
        if self.r#power == 10
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#north == North::Side
        {
            return 4477;
        }
        if self.r#south == South::None
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#power == 0
            && self.r#east == East::None
        {
            return 4968;
        }
        if self.r#power == 4
            && self.r#west == West::Side
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#north == North::Up
        {
            return 4714;
        }
        if self.r#west == West::Side
            && self.r#south == South::None
            && self.r#east == East::Up
            && self.r#power == 13
            && self.r#north == North::None
        {
            return 4222;
        }
        if self.r#west == West::None
            && self.r#north == North::Up
            && self.r#power == 1
            && self.r#east == East::Up
            && self.r#south == South::Up
        {
            return 3821;
        }
        if self.r#west == West::Up
            && self.r#east == East::None
            && self.r#power == 5
            && self.r#south == South::Side
            && self.r#north == North::None
        {
            return 5010;
        }
        if self.r#west == West::None
            && self.r#east == East::None
            && self.r#power == 6
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 5024;
        }
        if self.r#power == 3
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#south == South::None
        {
            return 4133;
        }
        if self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#west == West::Up
            && self.r#south == South::None
            && self.r#power == 10
        {
            return 4050;
        }
        if self.r#power == 6
            && self.r#south == South::None
            && self.r#east == East::Up
            && self.r#west == West::None
            && self.r#north == North::Side
        {
            return 4016;
        }
        if self.r#south == South::Up
            && self.r#power == 5
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#west == West::Up
        {
            return 4143;
        }
        if self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#power == 15
        {
            return 4955;
        }
        if self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 11
            && self.r#west == West::None
        {
            return 4490;
        }
        if self.r#east == East::Side
            && self.r#power == 1
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#south == South::Side
        {
            return 4542;
        }
        if self.r#north == North::Side
            && self.r#power == 14
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::None
        {
            return 4952;
        }
        if self.r#north == North::None
            && self.r#west == West::Up
            && self.r#power == 7
            && self.r#east == East::Up
            && self.r#south == South::None
        {
            return 4167;
        }
        if self.r#north == North::Up
            && self.r#west == West::None
            && self.r#east == East::Up
            && self.r#power == 3
            && self.r#south == South::Up
        {
            return 3839;
        }
        if self.r#south == South::None
            && self.r#east == East::Up
            && self.r#power == 3
            && self.r#north == North::Side
            && self.r#west == West::None
        {
            return 3989;
        }
        if self.r#north == North::None
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#power == 14
            && self.r#west == West::Up
        {
            return 4230;
        }
        if self.r#south == South::None
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#east == East::Up
            && self.r#power == 1
        {
            return 3827;
        }
        if self.r#north == North::Up
            && self.r#east == East::None
            && self.r#power == 12
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 4784;
        }
        if self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#power == 2
            && self.r#north == North::Side
            && self.r#west == West::Up
        {
            return 4404;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#power == 15
            && self.r#east == East::Side
            && self.r#north == North::Side
        {
            return 4529;
        }
        if self.r#south == South::None
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#power == 15
            && self.r#west == West::None
        {
            return 4673;
        }
        if self.r#power == 9
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#north == North::Side
        {
            return 4899;
        }
        if self.r#east == East::Up
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#power == 12
            && self.r#south == South::Up
        {
            return 4208;
        }
        if self.r#east == East::None
            && self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#power == 9
        {
            return 4758;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::Up
            && self.r#power == 15
            && self.r#east == East::Side
        {
            return 4385;
        }
        if self.r#west == West::Side
            && self.r#south == South::Up
            && self.r#power == 15
            && self.r#east == East::Up
            && self.r#north == North::None
        {
            return 4234;
        }
        if self.r#south == South::Up
            && self.r#power == 3
            && self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#north == North::Up
        {
            return 3837;
        }
        if self.r#power == 13
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::Up
            && self.r#north == North::Side
        {
            return 4937;
        }
        if self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#power == 10
        {
            return 4046;
        }
        if self.r#north == North::None
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#power == 13
            && self.r#east == East::Up
        {
            return 4216;
        }
        if self.r#power == 14
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#east == East::Up
            && self.r#south == South::None
        {
            return 4232;
        }
        if self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#power == 3
            && self.r#west == West::Side
        {
            return 4273;
        }
        if self.r#north == North::None
            && self.r#south == South::Side
            && self.r#power == 10
            && self.r#west == West::None
            && self.r#east == East::None
        {
            return 5057;
        }
        if self.r#north == North::None
            && self.r#east == East::Up
            && self.r#power == 6
            && self.r#south == South::Up
            && self.r#west == West::Up
        {
            return 4152;
        }
        if self.r#north == North::None
            && self.r#east == East::Up
            && self.r#power == 4
            && self.r#south == South::None
            && self.r#west == West::Up
        {
            return 4140;
        }
        if self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#power == 5
        {
            return 4003;
        }
        if self.r#north == North::Side
            && self.r#west == West::None
            && self.r#power == 8
            && self.r#east == East::Side
            && self.r#south == South::None
        {
            return 4466;
        }
        if self.r#power == 6
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#west == West::None
            && self.r#east == East::Up
        {
            return 3869;
        }
        if self.r#west == West::None
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#power == 6
            && self.r#south == South::Side
        {
            return 4445;
        }
        if self.r#power == 3
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#east == East::Side
        {
            return 4563;
        }
        if self.r#east == East::Side
            && self.r#power == 4
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#south == South::Up
        {
            return 4280;
        }
        if self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#power == 8
            && self.r#east == East::None
        {
            return 4890;
        }
        if self.r#east == East::Up
            && self.r#north == North::None
            && self.r#power == 6
            && self.r#south == South::None
            && self.r#west == West::Side
        {
            return 4159;
        }
        if self.r#west == West::Side
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#east == East::None
            && self.r#power == 6
        {
            return 5017;
        }
        if self.r#south == South::None
            && self.r#east == East::Up
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#power == 12
        {
            return 4212;
        }
        if self.r#power == 8
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#east == East::Up
            && self.r#west == West::None
        {
            return 4172;
        }
        if self.r#power == 0
            && self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#west == West::Up
        {
            return 4386;
        }
        if self.r#north == North::None
            && self.r#west == West::Side
            && self.r#south == South::Side
            && self.r#power == 2
            && self.r#east == East::Side
        {
            return 4552;
        }
        if self.r#east == East::Side
            && self.r#power == 7
            && self.r#west == West::Side
            && self.r#north == North::None
            && self.r#south == South::Up
        {
            return 4594;
        }
        if self.r#east == East::None
            && self.r#north == North::Up
            && self.r#power == 12
            && self.r#south == South::None
            && self.r#west == West::Side
        {
            return 4789;
        }
        if self.r#east == East::None
            && self.r#power == 15
            && self.r#south == South::None
            && self.r#north == North::Up
            && self.r#west == West::None
        {
            return 4817;
        }
        if self.r#east == East::Side
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#power == 1
        {
            return 4547;
        }
        if self.r#north == North::Up
            && self.r#west == West::Side
            && self.r#power == 15
            && self.r#south == South::None
            && self.r#east == East::Up
        {
            return 3952;
        }
        if self.r#power == 3
            && self.r#west == West::None
            && self.r#east == East::Side
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 4565;
        }
        if self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#power == 2
            && self.r#north == North::Up
        {
            return 3835;
        }
        if self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#power == 9
            && self.r#east == East::Up
            && self.r#north == North::Up
        {
            return 3892;
        }
        if self.r#west == West::Side
            && self.r#power == 5
            && self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#north == North::Up
        {
            return 3859;
        }
        if self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#south == South::Side
            && self.r#east == East::None
            && self.r#power == 7
        {
            return 4740;
        }
        if self.r#east == East::None
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#north == North::None
            && self.r#power == 1
        {
            return 4975;
        }
        if self.r#power == 9
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 4612;
        }
        if self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#north == North::None
            && self.r#power == 5
            && self.r#west == West::Up
        {
            return 4146;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#power == 11
            && self.r#south == South::Up
            && self.r#west == West::Up
        {
            return 5061;
        }
        if self.r#power == 9
            && self.r#west == West::Side
            && self.r#north == North::None
            && self.r#south == South::Side
            && self.r#east == East::Up
        {
            return 4183;
        }
        if self.r#south == South::Up
            && self.r#east == East::Up
            && self.r#power == 8
            && self.r#west == West::Side
            && self.r#north == North::Up
        {
            return 3883;
        }
        if self.r#west == West::None
            && self.r#north == North::Up
            && self.r#power == 0
            && self.r#south == South::Up
            && self.r#east == East::Side
        {
            return 4244;
        }
        if self.r#power == 3
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#east == East::None
            && self.r#south == South::Side
        {
            return 4705;
        }
        if self.r#power == 6
            && self.r#east == East::None
            && self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#west == West::None
        {
            return 4874;
        }
        if self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#power == 1
            && self.r#west == West::Up
        {
            return 3825;
        }
        if self.r#south == South::None
            && self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#power == 13
            && self.r#west == West::None
        {
            return 4079;
        }
        if self.r#west == West::Side
            && self.r#power == 10
            && self.r#north == North::Up
            && self.r#south == South::None
            && self.r#east == East::Up
        {
            return 3907;
        }
        if self.r#south == South::Side
            && self.r#north == North::None
            && self.r#east == East::Up
            && self.r#power == 3
            && self.r#west == West::None
        {
            return 4130;
        }
        if self.r#south == South::Side
            && self.r#power == 3
            && self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#west == West::None
        {
            return 4274;
        }
        if self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#power == 11
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 4344;
        }
        if self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#west == West::None
            && self.r#power == 10
            && self.r#north == North::Up
        {
            return 4337;
        }
        if self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#power == 9
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 4475;
        }
        if self.r#power == 11
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#north == North::Side
        {
            return 4485;
        }
        if self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#power == 15
            && self.r#west == West::Side
            && self.r#south == South::Up
        {
            return 4090;
        }
        if self.r#south == South::None
            && self.r#north == North::None
            && self.r#power == 9
            && self.r#east == East::Side
            && self.r#west == West::Up
        {
            return 4617;
        }
        if self.r#power == 2
            && self.r#north == North::Up
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#west == West::Side
        {
            return 4699;
        }
        if self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#power == 10
            && self.r#west == West::Up
        {
            return 4335;
        }
        if self.r#west == West::None
            && self.r#north == North::Up
            && self.r#power == 3
            && self.r#east == East::None
            && self.r#south == South::None
        {
            return 4709;
        }
        if self.r#south == South::Up
            && self.r#power == 1
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#west == West::Up
        {
            return 3819;
        }
        if self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#west == West::Side
            && self.r#power == 6
        {
            return 4585;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#power == 14
            && self.r#east == East::Side
        {
            return 4664;
        }
        if self.r#east == East::None
            && self.r#power == 2
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 4838;
        }
        if self.r#west == West::Up
            && self.r#south == South::Side
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#power == 9
        {
            return 4326;
        }
        if self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#north == North::None
            && self.r#power == 9
            && self.r#west == West::Side
        {
            return 4615;
        }
        if self.r#north == North::Up
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#power == 0
        {
            return 4682;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#west == West::None
            && self.r#power == 9
        {
            return 5048;
        }
        if self.r#south == South::Side
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#power == 14
            && self.r#west == West::Side
        {
            return 5092;
        }
        if self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#power == 6
            && self.r#north == North::Up
        {
            return 3865;
        }
        if self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#south == South::None
            && self.r#east == East::Side
            && self.r#power == 4
        {
            return 4284;
        }
        if self.r#east == East::None
            && self.r#west == West::None
            && self.r#north == North::Up
            && self.r#power == 5
            && self.r#south == South::Up
        {
            return 4721;
        }
        if self.r#power == 3
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#east == East::None
        {
            return 4989;
        }
        if self.r#power == 12
            && self.r#north == North::Up
            && self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#south == South::Up
        {
            return 3919;
        }
        if self.r#power == 8
            && self.r#south == South::Up
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#east == East::Up
        {
            return 4170;
        }
        if self.r#south == South::Up
            && self.r#power == 15
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 5099;
        }
        if self.r#east == East::Up
            && self.r#power == 8
            && self.r#south == South::Up
            && self.r#west == West::None
            && self.r#north == North::Side
        {
            return 4028;
        }
        if self.r#east == East::None
            && self.r#west == West::Side
            && self.r#north == North::None
            && self.r#power == 10
            && self.r#south == South::Side
        {
            return 5056;
        }
        if self.r#power == 11
            && self.r#south == South::None
            && self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#east == East::Up
        {
            return 3915;
        }
        if self.r#west == West::Up
            && self.r#power == 6
            && self.r#south == South::Up
            && self.r#north == North::Side
            && self.r#east == East::None
        {
            return 4872;
        }
        if self.r#power == 13
            && self.r#north == North::None
            && self.r#east == East::Side
            && self.r#south == South::None
            && self.r#west == West::Up
        {
            return 4653;
        }
        if self.r#east == East::Up
            && self.r#west == West::Up
            && self.r#north == North::Up
            && self.r#south == South::None
            && self.r#power == 15
        {
            return 3951;
        }
        if self.r#west == West::Side
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 13
            && self.r#south == South::Side
        {
            return 4939;
        }
        if self.r#power == 8
            && self.r#west == West::Up
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 5040;
        }
        if self.r#west == West::None
            && self.r#north == North::Up
            && self.r#power == 7
            && self.r#south == South::Side
            && self.r#east == East::Side
        {
            return 4310;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#power == 5
        {
            return 4727;
        }
        if self.r#north == North::Side
            && self.r#east == East::None
            && self.r#south == South::Side
            && self.r#power == 9
            && self.r#west == West::Side
        {
            return 4903;
        }
        if self.r#north == North::Up
            && self.r#power == 8
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::Side
        {
            return 4322;
        }
        if self.r#west == West::None
            && self.r#power == 11
            && self.r#north == North::Up
            && self.r#east == East::None
            && self.r#south == South::Up
        {
            return 4775;
        }
        if self.r#south == South::None
            && self.r#west == West::Side
            && self.r#power == 3
            && self.r#east == East::None
            && self.r#north == North::Up
        {
            return 4708;
        }
        if self.r#north == North::None
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#power == 6
        {
            return 4156;
        }
        if self.r#west == West::Side
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#power == 11
            && self.r#south == South::None
        {
            return 4780;
        }
        if self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#power == 11
        {
            return 4343;
        }
        if self.r#power == 0
            && self.r#south == South::Side
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 4967;
        }
        if self.r#east == East::Up
            && self.r#west == West::Up
            && self.r#power == 13
            && self.r#north == North::Up
            && self.r#south == South::None
        {
            return 3933;
        }
        if self.r#power == 14
            && self.r#east == East::Side
            && self.r#west == West::None
            && self.r#south == South::Up
            && self.r#north == North::Side
        {
            return 4514;
        }
        if self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#south == South::None
            && self.r#power == 14
            && self.r#west == West::Up
        {
            return 4374;
        }
        if self.r#power == 7
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 4450;
        }
        if self.r#west == West::None
            && self.r#power == 10
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#south == South::None
        {
            return 4772;
        }
        if self.r#west == West::None
            && self.r#power == 12
            && self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#south == South::Side
        {
            return 4067;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#south == South::Up
            && self.r#power == 1
        {
            return 4973;
        }
        if self.r#east == East::Side
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#power == 11
            && self.r#south == South::None
        {
            return 4637;
        }
        if self.r#west == West::None
            && self.r#north == North::Up
            && self.r#power == 15
            && self.r#south == South::Up
            && self.r#east == East::None
        {
            return 4811;
        }
        if self.r#west == West::Up
            && self.r#power == 6
            && self.r#south == South::Up
            && self.r#east == East::Side
            && self.r#north == North::Side
        {
            return 4440;
        }
        if self.r#power == 3
            && self.r#south == South::Up
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#east == East::Up
        {
            return 3838;
        }
        if self.r#east == East::Up
            && self.r#power == 11
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#south == South::Up
        {
            return 3910;
        }
        if self.r#east == East::Up
            && self.r#power == 12
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#north == North::Up
        {
            return 3922;
        }
        if self.r#west == West::Side
            && self.r#south == South::Side
            && self.r#power == 11
            && self.r#north == North::Side
            && self.r#east == East::Up
        {
            return 4057;
        }
        if self.r#power == 9
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#north == North::None
        {
            return 4179;
        }
        if self.r#east == East::None
            && self.r#north == North::Side
            && self.r#west == West::Side
            && self.r#south == South::None
            && self.r#power == 2
        {
            return 4843;
        }
        if self.r#power == 7
            && self.r#west == West::None
            && self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#north == North::Side
        {
            return 4022;
        }
        if self.r#power == 8
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#west == West::Up
            && self.r#north == North::Up
        {
            return 4752;
        }
        if self.r#power == 4
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 5006;
        }
        if self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#power == 3
            && self.r#south == South::None
        {
            return 4276;
        }
        if self.r#power == 1
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 4546;
        }
        if self.r#west == West::Side
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#power == 13
            && self.r#south == South::Up
        {
            return 4792;
        }
        if self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 1
            && self.r#south == South::Side
        {
            return 4832;
        }
        if self.r#south == South::None
            && self.r#east == East::Side
            && self.r#power == 15
            && self.r#north == North::None
            && self.r#west == West::Side
        {
            return 4672;
        }
        if self.r#north == North::Up
            && self.r#east == East::None
            && self.r#power == 1
            && self.r#south == South::None
            && self.r#west == West::Side
        {
            return 4690;
        }
        if self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Side
            && self.r#power == 11
            && self.r#west == West::Up
        {
            return 4923;
        }
        if self.r#west == West::Side
            && self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#power == 0
        {
            return 4243;
        }
        if self.r#south == South::Up
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#power == 0
            && self.r#east == East::None
        {
            return 4964;
        }
        if self.r#west == West::None
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#power == 9
            && self.r#south == South::None
        {
            return 3899;
        }
        if self.r#north == North::None
            && self.r#power == 10
            && self.r#south == South::Up
            && self.r#east == East::None
            && self.r#west == West::Side
        {
            return 5053;
        }
        if self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#west == West::Up
            && self.r#power == 0
            && self.r#south == South::None
        {
            return 3960;
        }
        if self.r#south == South::None
            && self.r#power == 2
            && self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#north == North::Side
        {
            return 4410;
        }
        if self.r#north == North::None
            && self.r#east == East::Side
            && self.r#power == 10
            && self.r#west == West::Side
            && self.r#south == South::Side
        {
            return 4624;
        }
        if self.r#north == North::None
            && self.r#south == South::Side
            && self.r#east == East::Up
            && self.r#power == 4
            && self.r#west == West::None
        {
            return 4139;
        }
        if self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#power == 6
            && self.r#south == South::Up
            && self.r#east == East::Up
        {
            return 4008;
        }
        if self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#power == 2
            && self.r#west == West::Side
            && self.r#south == South::Up
        {
            return 4261;
        }
        if self.r#north == North::None
            && self.r#power == 5
            && self.r#south == South::Side
            && self.r#west == West::None
            && self.r#east == East::None
        {
            return 5012;
        }
        if self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#power == 7
        {
            return 3874;
        }
        if self.r#west == West::None
            && self.r#power == 1
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#east == East::Up
        {
            return 3965;
        }
        if self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#power == 1
            && self.r#east == East::Up
            && self.r#north == North::Side
        {
            return 3966;
        }
        if self.r#east == East::Side
            && self.r#west == West::Up
            && self.r#power == 15
            && self.r#north == North::None
            && self.r#south == South::Side
        {
            return 4668;
        }
        if self.r#west == West::None
            && self.r#power == 15
            && self.r#north == North::Up
            && self.r#east == East::Up
            && self.r#south == South::None
        {
            return 3953;
        }
        if self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#power == 7
        {
            return 4600;
        }
        if self.r#east == East::Up
            && self.r#north == North::None
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#power == 1
        {
            return 4110;
        }
        if self.r#power == 9
            && self.r#east == East::Side
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#west == West::Up
        {
            return 4470;
        }
        if self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#power == 13
            && self.r#west == West::Side
            && self.r#north == North::None
        {
            return 4651;
        }
        if self.r#east == East::None
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#power == 5
            && self.r#north == North::Up
        {
            return 4719;
        }
        if self.r#north == North::Up
            && self.r#west == West::Up
            && self.r#power == 9
            && self.r#south == South::Side
            && self.r#east == East::Up
        {
            return 3894;
        }
        if self.r#west == West::Up
            && self.r#east == East::None
            && self.r#north == North::Up
            && self.r#power == 12
            && self.r#south == South::Side
        {
            return 4785;
        }
        if self.r#power == 10
            && self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 4484;
        }
        if self.r#south == South::Up
            && self.r#north == North::Up
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#power == 1
        {
            return 4685;
        }
        if self.r#south == South::Side
            && self.r#power == 11
            && self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#north == North::None
        {
            return 4200;
        }
        if self.r#power == 10
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#north == North::Side
        {
            return 4044;
        }
        if self.r#west == West::Side
            && self.r#power == 4
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#east == East::Side
        {
            return 4423;
        }
        if self.r#north == North::Up
            && self.r#south == South::None
            && self.r#east == East::Up
            && self.r#west == West::Up
            && self.r#power == 4
        {
            return 3852;
        }
        if self.r#east == East::Side
            && self.r#power == 14
            && self.r#north == North::None
            && self.r#south == South::Up
            && self.r#west == West::Side
        {
            return 4657;
        }
        if self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#power == 7
            && self.r#south == South::Side
        {
            return 3877;
        }
        if self.r#north == North::Up
            && self.r#power == 7
            && self.r#south == South::Side
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 4742;
        }
        if self.r#power == 2
            && self.r#south == South::None
            && self.r#north == North::Side
            && self.r#east == East::Up
            && self.r#west == West::Side
        {
            return 3979;
        }
        if self.r#power == 0
            && self.r#west == West::Side
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Side
        {
            return 4825;
        }
        if self.r#north == North::Up
            && self.r#south == South::None
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#power == 9
        {
            return 4330;
        }
        if self.r#south == South::None
            && self.r#power == 2
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Side
        {
            return 4844;
        }
        if self.r#north == North::Side
            && self.r#power == 14
            && self.r#east == East::Up
            && self.r#west == West::None
            && self.r#south == South::Up
        {
            return 4082;
        }
        if self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#power == 15
            && self.r#south == South::Side
            && self.r#north == North::Side
        {
            return 4092;
        }
        if self.r#east == East::None
            && self.r#west == West::Side
            && self.r#power == 6
            && self.r#south == South::Side
            && self.r#north == North::None
        {
            return 5020;
        }
        if self.r#east == East::Side
            && self.r#north == North::None
            && self.r#power == 14
            && self.r#south == South::Up
            && self.r#west == West::Up
        {
            return 4656;
        }
        if self.r#north == North::Up
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#power == 10
            && self.r#south == South::Side
        {
            return 4769;
        }
        if self.r#power == 4
            && self.r#south == South::Side
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#east == East::Up
        {
            return 3850;
        }
        if self.r#west == West::None
            && self.r#power == 7
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#north == North::None
        {
            return 4163;
        }
        if self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#power == 3
            && self.r#south == South::None
            && self.r#west == West::Up
        {
            return 3843;
        }
        if self.r#north == North::Up
            && self.r#south == South::None
            && self.r#power == 15
            && self.r#west == West::Side
            && self.r#east == East::None
        {
            return 4816;
        }
        if self.r#west == West::None
            && self.r#east == East::Up
            && self.r#north == North::Side
            && self.r#south == South::None
            && self.r#power == 9
        {
            return 4043;
        }
        if self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#power == 14
            && self.r#west == West::Side
            && self.r#south == South::Side
        {
            return 4372;
        }
        if self.r#east == East::Up
            && self.r#power == 3
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#south == South::Up
        {
            return 4127;
        }
        if self.r#south == South::Side
            && self.r#power == 7
            && self.r#north == North::None
            && self.r#west == West::Up
            && self.r#east == East::Side
        {
            return 4596;
        }
        if self.r#north == North::Up
            && self.r#west == West::None
            && self.r#power == 12
            && self.r#east == East::Up
            && self.r#south == South::Up
        {
            return 3920;
        }
        if self.r#north == North::Side
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#power == 3
            && self.r#south == South::None
        {
            return 4420;
        }
        if self.r#west == West::Up
            && self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#north == North::None
            && self.r#power == 15
        {
            return 4665;
        }
        if self.r#north == North::Side
            && self.r#west == West::Side
            && self.r#power == 3
            && self.r#east == East::None
            && self.r#south == South::Side
        {
            return 4849;
        }
        if self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#south == South::None
            && self.r#power == 10
        {
            return 3906;
        }
        if self.r#south == South::None
            && self.r#west == West::Side
            && self.r#north == North::Up
            && self.r#east == East::Side
            && self.r#power == 15
        {
            return 4384;
        }
        if self.r#power == 2
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#south == South::Side
            && self.r#east == East::None
        {
            return 4983;
        }
        if self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#north == North::None
            && self.r#south == South::Side
            && self.r#power == 4
        {
            return 4138;
        }
        if self.r#east == East::None
            && self.r#north == North::Up
            && self.r#power == 15
            && self.r#south == South::Side
            && self.r#west == West::Side
        {
            return 4813;
        }
        if self.r#east == East::None
            && self.r#power == 2
            && self.r#north == North::Up
            && self.r#south == South::Up
            && self.r#west == West::None
        {
            return 4694;
        }
        if self.r#east == East::Side
            && self.r#south == South::Up
            && self.r#west == West::Up
            && self.r#power == 0
            && self.r#north == North::Up
        {
            return 4242;
        }
        if self.r#south == South::Up
            && self.r#east == East::Up
            && self.r#north == North::Up
            && self.r#power == 2
            && self.r#west == West::Side
        {
            return 3829;
        }
        if self.r#south == South::Side
            && self.r#east == East::Side
            && self.r#west == West::Side
            && self.r#power == 13
            && self.r#north == North::Up
        {
            return 4363;
        }
        if self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#power == 6
            && self.r#north == North::Up
            && self.r#south == South::None
        {
            return 4303;
        }
        if self.r#south == South::Side
            && self.r#north == North::Side
            && self.r#west == West::None
            && self.r#east == East::Side
            && self.r#power == 15
        {
            return 4526;
        }
        if self.r#east == East::Side
            && self.r#power == 12
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::Side
        {
            return 4643;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::Side
            && self.r#power == 0
            && self.r#west == West::Up
        {
            return 4824;
        }
        if self.r#west == West::Side
            && self.r#north == North::Side
            && self.r#power == 1
            && self.r#east == East::None
            && self.r#south == South::Up
        {
            return 4828;
        }
        if self.r#south == South::None
            && self.r#west == West::Side
            && self.r#power == 7
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 5032;
        }
        if self.r#east == East::None
            && self.r#north == North::Side
            && self.r#south == South::Up
            && self.r#power == 10
            && self.r#west == West::None
        {
            return 4910;
        }
        if self.r#west == West::Side
            && self.r#east == East::Up
            && self.r#south == South::Side
            && self.r#north == North::Side
            && self.r#power == 15
        {
            return 4093;
        }
        if self.r#south == South::None
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#power == 0
            && self.r#east == East::Side
        {
            return 4538;
        }
        if self.r#north == North::None
            && self.r#south == South::Side
            && self.r#power == 4
            && self.r#east == East::Side
            && self.r#west == West::Up
        {
            return 4569;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#power == 10
            && self.r#south == South::None
            && self.r#east == East::None
        {
            return 5060;
        }
        if self.r#north == North::Side
            && self.r#power == 13
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#west == West::Up
        {
            return 4077;
        }
        if self.r#power == 13
            && self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#east == East::None
            && self.r#west == West::Up
        {
            return 4938;
        }
        if self.r#power == 15
            && self.r#west == West::None
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#east == East::Side
        {
            return 4382;
        }
        if self.r#north == North::None
            && self.r#power == 3
            && self.r#west == West::Side
            && self.r#east == East::Side
            && self.r#south == South::Side
        {
            return 4561;
        }
        if self.r#north == North::Side
            && self.r#south == South::Side
            && self.r#power == 1
            && self.r#west == West::Side
            && self.r#east == East::Side
        {
            return 4399;
        }
        if self.r#south == South::None
            && self.r#east == East::Up
            && self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#power == 4
        {
            return 3996;
        }
        if self.r#power == 8
            && self.r#east == East::Up
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 4178;
        }
        if self.r#power == 8
            && self.r#south == South::Side
            && self.r#west == West::Up
            && self.r#north == North::None
            && self.r#east == East::Up
        {
            return 4173;
        }
        if self.r#power == 4
            && self.r#east == East::Up
            && self.r#south == South::Up
            && self.r#north == North::None
            && self.r#west == West::Up
        {
            return 4134;
        }
        if self.r#north == North::None
            && self.r#east == East::Side
            && self.r#south == South::Side
            && self.r#west == West::None
            && self.r#power == 7
        {
            return 4598;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#power == 6
            && self.r#west == West::None
            && self.r#south == South::Up
        {
            return 5018;
        }
        if self.r#west == West::Up
            && self.r#east == East::Up
            && self.r#north == North::None
            && self.r#power == 11
            && self.r#south == South::None
        {
            return 4203;
        }
        if self.r#east == East::Side
            && self.r#north == North::None
            && self.r#power == 11
            && self.r#west == West::None
            && self.r#south == South::Side
        {
            return 4634;
        }
        if self.r#south == South::None
            && self.r#west == West::Side
            && self.r#power == 8
            && self.r#north == North::None
            && self.r#east == East::Up
        {
            return 4177;
        }
        if self.r#power == 11
            && self.r#east == East::Side
            && self.r#north == North::Up
            && self.r#south == South::Side
            && self.r#west == West::None
        {
            return 4346;
        }
        if self.r#south == South::Side
            && self.r#west == West::None
            && self.r#power == 6
            && self.r#north == North::None
            && self.r#east == East::None
        {
            return 5021;
        }
        if self.r#east == East::None
            && self.r#power == 8
            && self.r#west == West::Up
            && self.r#north == North::Side
            && self.r#south == South::Side
        {
            return 4893;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 4572 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::None,
                r#west: West::Up,
                r#north: North::None,
                r#power: 4,
            });
        }
        if state_id == 4252 {
            return Some(RedstoneWire {
                r#power: 1,
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4247 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#south: South::Side,
                r#north: North::Up,
                r#power: 0,
            });
        }
        if state_id == 4739 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::None,
                r#south: South::Up,
                r#north: North::Up,
                r#power: 7,
            });
        }
        if state_id == 4295 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 5,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Side,
            });
        }
        if state_id == 4281 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#north: North::Up,
                r#power: 4,
                r#south: South::Side,
            });
        }
        if state_id == 4391 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::Side,
                r#power: 0,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4759 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::None,
                r#power: 9,
            });
        }
        if state_id == 4830 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::None,
                r#north: North::Side,
                r#power: 1,
                r#south: South::Side,
            });
        }
        if state_id == 3817 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::None,
                r#power: 0,
                r#west: West::Side,
                r#north: North::Up,
            });
        }
        if state_id == 5068 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#power: 11,
                r#west: West::Side,
            });
        }
        if state_id == 4131 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#west: West::Up,
                r#south: South::None,
                r#power: 3,
            });
        }
        if state_id == 4932 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#power: 12,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4231 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#power: 14,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 4129 {
            return Some(RedstoneWire {
                r#power: 3,
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::None,
            });
        }
        if state_id == 4781 {
            return Some(RedstoneWire {
                r#power: 11,
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Up,
            });
        }
        if state_id == 4966 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#power: 0,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4623 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::None,
                r#power: 10,
            });
        }
        if state_id == 4858 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::Side,
                r#east: East::None,
                r#power: 4,
                r#north: North::Side,
            });
        }
        if state_id == 5004 {
            return Some(RedstoneWire {
                r#power: 4,
                r#south: South::None,
                r#west: West::Up,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 3925 {
            return Some(RedstoneWire {
                r#power: 12,
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 5094 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 14,
                r#west: West::Up,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 5100 {
            return Some(RedstoneWire {
                r#power: 15,
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 4402 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 1,
                r#north: North::Side,
                r#west: West::Side,
                r#south: South::None,
            });
        }
        if state_id == 4150 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::None,
                r#east: East::Up,
                r#power: 5,
                r#west: West::Side,
            });
        }
        if state_id == 4408 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#power: 2,
                r#east: East::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4622 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 10,
                r#west: West::None,
                r#east: East::Side,
                r#north: North::None,
            });
        }
        if state_id == 4237 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::None,
                r#east: East::Up,
                r#power: 15,
                r#south: South::Side,
            });
        }
        if state_id == 4820 {
            return Some(RedstoneWire {
                r#power: 0,
                r#south: South::Up,
                r#north: North::Side,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 4439 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#west: West::None,
                r#power: 5,
                r#east: East::Side,
            });
        }
        if state_id == 4866 {
            return Some(RedstoneWire {
                r#power: 5,
                r#west: West::Up,
                r#east: East::None,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4511 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#west: West::None,
                r#power: 13,
                r#south: South::None,
            });
        }
        if state_id == 4723 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 5,
            });
        }
        if state_id == 3980 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Side,
                r#west: West::None,
                r#east: East::Up,
                r#power: 2,
            });
        }
        if state_id == 4123 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Side,
                r#power: 2,
                r#north: North::None,
            });
        }
        if state_id == 4471 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#power: 9,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4582 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 5,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 4913 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Side,
                r#north: North::Side,
                r#west: West::None,
                r#power: 10,
            });
        }
        if state_id == 5038 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 8,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4931 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#north: North::Side,
                r#power: 12,
                r#west: West::None,
            });
        }
        if state_id == 3916 {
            return Some(RedstoneWire {
                r#power: 11,
                r#south: South::None,
                r#north: North::Up,
                r#east: East::Up,
                r#west: West::Side,
            });
        }
        if state_id == 3967 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 1,
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 3816 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Up,
                r#power: 0,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 3820 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 1,
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4712 {
            return Some(RedstoneWire {
                r#power: 4,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::None,
                r#north: North::Up,
            });
        }
        if state_id == 4191 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::None,
                r#east: East::Up,
                r#power: 10,
            });
        }
        if state_id == 3949 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::Up,
                r#power: 15,
            });
        }
        if state_id == 5103 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#east: East::None,
                r#north: North::None,
                r#power: 15,
            });
        }
        if state_id == 3904 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Side,
                r#north: North::Up,
                r#south: South::Side,
                r#power: 10,
            });
        }
        if state_id == 4136 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#north: North::None,
                r#power: 4,
                r#south: South::Up,
            });
        }
        if state_id == 4915 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 10,
                r#south: South::None,
                r#east: East::None,
                r#west: West::Side,
            });
        }
        if state_id == 4827 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::None,
                r#power: 1,
                r#south: South::Up,
            });
        }
        if state_id == 4319 {
            return Some(RedstoneWire {
                r#power: 8,
                r#east: East::Side,
                r#west: West::None,
                r#south: South::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4693 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Side,
                r#power: 2,
                r#north: North::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4113 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Up,
                r#north: North::None,
                r#power: 1,
            });
        }
        if state_id == 4683 {
            return Some(RedstoneWire {
                r#power: 1,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::None,
                r#north: North::Up,
            });
        }
        if state_id == 3856 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::Up,
                r#power: 5,
                r#south: South::Up,
            });
        }
        if state_id == 3845 {
            return Some(RedstoneWire {
                r#power: 3,
                r#north: North::Up,
                r#west: West::None,
                r#east: East::Up,
                r#south: South::None,
            });
        }
        if state_id == 4313 {
            return Some(RedstoneWire {
                r#power: 7,
                r#west: West::None,
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 3849 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 4,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4536 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::None,
                r#north: North::None,
                r#power: 0,
                r#east: East::Side,
            });
        }
        if state_id == 4548 {
            return Some(RedstoneWire {
                r#power: 2,
                r#north: North::None,
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 3897 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Up,
                r#power: 9,
                r#north: North::Up,
            });
        }
        if state_id == 3855 {
            return Some(RedstoneWire {
                r#power: 5,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4947 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Up,
                r#power: 14,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4108 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Up,
                r#power: 1,
                r#east: East::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4419 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#power: 3,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 3918 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Up,
                r#south: South::Up,
                r#power: 12,
                r#north: North::Up,
            });
        }
        if state_id == 4041 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 9,
                r#west: West::Up,
                r#east: East::Up,
                r#south: South::None,
            });
        }
        if state_id == 4154 {
            return Some(RedstoneWire {
                r#power: 6,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 4119 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#power: 2,
                r#east: East::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4658 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#south: South::Up,
                r#power: 14,
                r#west: West::None,
            });
        }
        if state_id == 3871 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Up,
                r#east: East::Up,
                r#power: 6,
                r#west: West::Side,
            });
        }
        if state_id == 4469 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 9,
                r#south: South::Up,
            });
        }
        if state_id == 4639 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Side,
                r#power: 12,
                r#north: North::None,
                r#south: South::Up,
            });
        }
        if state_id == 4397 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#power: 1,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4847 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Side,
                r#power: 3,
                r#south: South::Up,
            });
        }
        if state_id == 4267 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 2,
            });
        }
        if state_id == 4096 {
            return Some(RedstoneWire {
                r#power: 15,
                r#west: West::Side,
                r#east: East::Up,
                r#south: South::None,
                r#north: North::Side,
            });
        }
        if state_id == 4486 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 11,
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4250 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 0,
                r#south: South::None,
                r#east: East::Side,
                r#west: West::None,
            });
        }
        if state_id == 4432 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#power: 5,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4037 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::None,
                r#east: East::Up,
                r#power: 9,
                r#south: South::Up,
            });
        }
        if state_id == 4543 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 1,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Side,
            });
        }
        if state_id == 4421 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 3,
                r#east: East::Side,
                r#north: North::Side,
                r#west: West::None,
            });
        }
        if state_id == 4702 {
            return Some(RedstoneWire {
                r#power: 3,
                r#south: South::Up,
                r#east: East::None,
                r#north: North::Up,
                r#west: West::Side,
            });
        }
        if state_id == 3861 {
            return Some(RedstoneWire {
                r#power: 5,
                r#east: East::Up,
                r#west: West::Up,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 4169 {
            return Some(RedstoneWire {
                r#power: 7,
                r#north: North::None,
                r#east: East::Up,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 4841 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::None,
                r#power: 2,
                r#north: North::Side,
                r#east: East::None,
            });
        }
        if state_id == 4883 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Side,
                r#power: 7,
            });
        }
        if state_id == 5011 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#power: 5,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 5027 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Up,
                r#power: 7,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 4268 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#west: West::None,
                r#power: 2,
                r#south: South::None,
            });
        }
        if state_id == 5093 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::None,
                r#power: 14,
                r#south: South::Side,
                r#east: East::None,
            });
        }
        if state_id == 4768 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Side,
                r#north: North::Up,
                r#power: 10,
                r#west: West::Side,
            });
        }
        if state_id == 5072 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 12,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 5096 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#power: 14,
                r#east: East::None,
            });
        }
        if state_id == 3921 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 12,
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4358 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::None,
                r#north: North::Up,
                r#west: West::None,
                r#power: 12,
            });
        }
        if state_id == 4628 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 10,
                r#north: North::None,
                r#east: East::Side,
                r#south: South::None,
            });
        }
        if state_id == 4452 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::Side,
                r#south: South::Side,
                r#power: 7,
            });
        }
        if state_id == 5035 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::None,
                r#east: East::None,
                r#power: 8,
                r#west: West::Side,
            });
        }
        if state_id == 4030 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Up,
                r#power: 8,
                r#north: North::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4162 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::Up,
                r#north: North::None,
                r#east: East::Up,
                r#power: 7,
            });
        }
        if state_id == 4437 {
            return Some(RedstoneWire {
                r#power: 5,
                r#south: South::None,
                r#west: West::Up,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 5049 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 9,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 4856 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#south: South::Up,
                r#power: 4,
                r#west: West::None,
            });
        }
        if state_id == 4706 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::None,
                r#north: North::Up,
                r#east: East::None,
                r#power: 3,
            });
        }
        if state_id == 4103 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::None,
                r#power: 0,
                r#north: North::None,
            });
        }
        if state_id == 4587 {
            return Some(RedstoneWire {
                r#power: 6,
                r#north: North::None,
                r#east: East::Side,
                r#west: West::Up,
                r#south: South::Side,
            });
        }
        if state_id == 3823 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 1,
                r#south: South::Side,
            });
        }
        if state_id == 3993 {
            return Some(RedstoneWire {
                r#power: 4,
                r#west: West::Up,
                r#south: South::Side,
                r#north: North::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4184 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Up,
                r#west: West::None,
                r#north: North::None,
                r#power: 9,
            });
        }
        if state_id == 4238 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Up,
                r#west: West::None,
                r#north: North::None,
                r#power: 15,
            });
        }
        if state_id == 4147 {
            return Some(RedstoneWire {
                r#power: 5,
                r#north: North::None,
                r#south: South::Side,
                r#east: East::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4520 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::None,
                r#north: North::Side,
                r#power: 14,
                r#south: South::None,
            });
        }
        if state_id == 4729 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 6,
            });
        }
        if state_id == 4891 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::Side,
                r#power: 8,
                r#east: East::None,
            });
        }
        if state_id == 4746 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 8,
                r#west: West::Up,
                r#south: South::Up,
                r#east: East::None,
            });
        }
        if state_id == 4929 {
            return Some(RedstoneWire {
                r#power: 12,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Up,
                r#east: East::None,
            });
        }
        if state_id == 4934 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#power: 12,
                r#north: North::Side,
                r#east: East::None,
            });
        }
        if state_id == 4954 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 15,
                r#north: North::Side,
                r#east: East::None,
                r#west: West::Side,
            });
        }
        if state_id == 4754 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::None,
                r#power: 8,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 4175 {
            return Some(RedstoneWire {
                r#power: 8,
                r#south: South::Side,
                r#east: East::Up,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 4007 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 5,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 3860 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#north: North::Up,
                r#power: 5,
                r#south: South::Side,
            });
        }
        if state_id == 4347 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Up,
                r#south: South::None,
                r#power: 11,
                r#east: East::Side,
            });
        }
        if state_id == 4492 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 11,
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4953 {
            return Some(RedstoneWire {
                r#power: 15,
                r#west: West::Up,
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::None,
            });
        }
        if state_id == 4433 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#power: 5,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4990 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Up,
                r#power: 3,
            });
        }
        if state_id == 4625 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::None,
                r#east: East::Side,
                r#south: South::Side,
                r#power: 10,
            });
        }
        if state_id == 4027 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 8,
                r#south: South::Up,
                r#east: East::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4589 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 6,
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4289 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 5,
                r#south: South::Up,
                r#west: West::None,
                r#east: East::Side,
            });
        }
        if state_id == 3929 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Up,
                r#power: 13,
                r#east: East::Up,
                r#west: West::None,
            });
        }
        if state_id == 4661 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 14,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Side,
            });
        }
        if state_id == 4713 {
            return Some(RedstoneWire {
                r#power: 4,
                r#north: North::Up,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4654 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Side,
                r#power: 13,
            });
        }
        if state_id == 4950 {
            return Some(RedstoneWire {
                r#power: 14,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Up,
                r#north: North::Side,
            });
        }
        if state_id == 3959 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 0,
                r#south: South::Side,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 5037 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#west: West::Up,
                r#south: South::Side,
                r#power: 8,
            });
        }
        if state_id == 5039 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#east: East::None,
                r#power: 8,
                r#west: West::None,
            });
        }
        if state_id == 4555 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 2,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4943 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Side,
                r#west: West::None,
                r#east: East::None,
                r#power: 13,
            });
        }
        if state_id == 4045 {
            return Some(RedstoneWire {
                r#power: 10,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4993 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::None,
                r#north: North::None,
                r#power: 3,
            });
        }
        if state_id == 3984 {
            return Some(RedstoneWire {
                r#power: 3,
                r#west: West::Up,
                r#east: East::Up,
                r#south: South::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4393 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#south: South::None,
                r#power: 0,
                r#north: North::Side,
            });
        }
        if state_id == 4888 {
            return Some(RedstoneWire {
                r#power: 7,
                r#west: West::Side,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Side,
            });
        }
        if state_id == 4059 {
            return Some(RedstoneWire {
                r#power: 11,
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::Up,
                r#south: South::None,
            });
        }
        if state_id == 3994 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#power: 4,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4063 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::Up,
                r#power: 12,
                r#north: North::Side,
                r#east: East::Up,
            });
        }
        if state_id == 3963 {
            return Some(RedstoneWire {
                r#power: 1,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4819 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Side,
                r#power: 0,
                r#north: North::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4810 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::None,
                r#north: North::Up,
                r#power: 15,
            });
        }
        if state_id == 4803 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Up,
                r#power: 14,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 3889 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 8,
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::Up,
            });
        }
        if state_id == 3957 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#power: 0,
                r#east: East::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4091 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::None,
                r#power: 15,
                r#east: East::Up,
            });
        }
        if state_id == 4115 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::None,
                r#power: 1,
                r#south: South::None,
                r#east: East::Up,
            });
        }
        if state_id == 4595 {
            return Some(RedstoneWire {
                r#power: 7,
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 3938 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Up,
                r#power: 14,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4904 {
            return Some(RedstoneWire {
                r#power: 9,
                r#north: North::Side,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Side,
            });
        }
        if state_id == 4601 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 7,
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 4957 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Side,
                r#power: 15,
                r#south: South::Side,
                r#east: East::None,
            });
        }
        if state_id == 4013 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::None,
                r#power: 6,
                r#east: East::Up,
                r#north: North::Side,
            });
        }
        if state_id == 3836 {
            return Some(RedstoneWire {
                r#power: 2,
                r#south: South::None,
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 3862 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Up,
                r#power: 5,
                r#west: West::Side,
                r#south: South::None,
            });
        }
        if state_id == 5073 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 12,
                r#south: South::Side,
                r#north: North::None,
                r#west: West::Up,
            });
        }
        if state_id == 4444 {
            return Some(RedstoneWire {
                r#power: 6,
                r#south: South::Side,
                r#north: North::Side,
                r#east: East::Side,
                r#west: West::Side,
            });
        }
        if state_id == 3881 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 7,
                r#north: North::Up,
                r#south: South::None,
                r#east: East::Up,
            });
        }
        if state_id == 4556 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 2,
                r#east: East::Side,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 3991 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 4,
                r#west: West::Side,
                r#south: South::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4797 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 13,
                r#south: South::None,
                r#west: West::Up,
                r#east: East::None,
            });
        }
        if state_id == 4447 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 6,
            });
        }
        if state_id == 4266 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Side,
                r#west: West::Up,
                r#power: 2,
                r#north: North::Up,
            });
        }
        if state_id == 4112 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 1,
                r#south: South::Side,
                r#east: East::Up,
                r#west: West::None,
            });
        }
        if state_id == 4814 {
            return Some(RedstoneWire {
                r#power: 15,
                r#east: East::None,
                r#north: North::Up,
                r#west: West::None,
                r#south: South::Side,
            });
        }
        if state_id == 4314 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::Up,
                r#power: 8,
            });
        }
        if state_id == 4458 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#power: 8,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4854 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 4,
                r#west: West::Up,
                r#south: South::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4323 {
            return Some(RedstoneWire {
                r#power: 9,
                r#south: South::Up,
                r#north: North::Up,
                r#east: East::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4311 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Up,
                r#east: East::Side,
                r#power: 7,
                r#west: West::Up,
            });
        }
        if state_id == 3841 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Side,
                r#east: East::Up,
                r#west: West::Side,
                r#power: 3,
            });
        }
        if state_id == 4553 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 2,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Side,
            });
        }
        if state_id == 4207 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Up,
                r#north: North::None,
                r#power: 12,
                r#west: West::Side,
            });
        }
        if state_id == 4507 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 13,
                r#north: North::Side,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4434 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::Side,
                r#power: 5,
            });
        }
        if state_id == 4873 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Side,
                r#power: 6,
                r#east: East::None,
            });
        }
        if state_id == 4560 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#south: South::Side,
                r#north: North::None,
                r#power: 3,
            });
        }
        if state_id == 4649 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::None,
                r#north: North::None,
                r#power: 13,
            });
        }
        if state_id == 5047 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#power: 9,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4220 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::None,
                r#power: 13,
                r#east: East::Up,
                r#west: West::None,
            });
        }
        if state_id == 4068 {
            return Some(RedstoneWire {
                r#power: 12,
                r#south: South::None,
                r#north: North::Side,
                r#east: East::Up,
                r#west: West::Up,
            });
        }
        if state_id == 3973 {
            return Some(RedstoneWire {
                r#power: 2,
                r#north: North::Side,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 3828 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#south: South::Up,
                r#power: 2,
                r#north: North::Up,
            });
        }
        if state_id == 4014 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 6,
                r#south: South::None,
                r#east: East::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4677 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#north: North::Up,
                r#west: West::Up,
                r#power: 0,
            });
        }
        if state_id == 4720 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Up,
                r#power: 5,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 3908 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Up,
                r#power: 10,
            });
        }
        if state_id == 4142 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 4,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 4540 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 1,
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4795 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Side,
                r#south: South::Side,
                r#east: East::None,
                r#power: 13,
            });
        }
        if state_id == 4782 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Up,
                r#power: 12,
                r#north: North::Up,
                r#south: South::Up,
            });
        }
        if state_id == 5101 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 15,
                r#east: East::None,
            });
        }
        if state_id == 4807 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::None,
                r#power: 14,
                r#south: South::None,
            });
        }
        if state_id == 5102 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::None,
                r#west: West::None,
                r#power: 15,
                r#east: East::None,
            });
        }
        if state_id == 3917 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#north: North::Up,
                r#power: 11,
                r#south: South::None,
            });
        }
        if state_id == 4717 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::None,
                r#power: 4,
            });
        }
        if state_id == 4349 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 11,
                r#west: West::None,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 4650 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#north: North::None,
                r#west: West::Up,
                r#power: 13,
            });
        }
        if state_id == 4446 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::Side,
                r#power: 6,
            });
        }
        if state_id == 4988 {
            return Some(RedstoneWire {
                r#power: 2,
                r#south: South::None,
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 5030 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::None,
                r#west: West::None,
                r#power: 7,
                r#east: East::None,
            });
        }
        if state_id == 3873 {
            return Some(RedstoneWire {
                r#power: 7,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 5005 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 4,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 3863 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#north: North::Up,
                r#power: 5,
                r#south: South::None,
            });
        }
        if state_id == 4333 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#south: South::Up,
                r#power: 10,
                r#north: North::Up,
            });
        }
        if state_id == 3954 {
            return Some(RedstoneWire {
                r#power: 0,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4619 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#east: East::Side,
                r#power: 9,
                r#north: North::None,
            });
        }
        if state_id == 4523 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::None,
                r#east: East::Side,
                r#south: South::Up,
                r#power: 15,
            });
        }
        if state_id == 4848 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::None,
                r#north: North::Side,
                r#power: 3,
                r#south: South::Side,
            });
        }
        if state_id == 4613 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::None,
                r#power: 9,
                r#north: North::None,
                r#east: East::Side,
            });
        }
        if state_id == 4882 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#power: 7,
                r#east: East::None,
                r#north: North::Side,
            });
        }
        if state_id == 3924 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#south: South::None,
                r#north: North::Up,
                r#power: 12,
            });
        }
        if state_id == 4019 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Side,
                r#power: 7,
                r#south: South::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4256 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Side,
                r#power: 1,
                r#west: West::None,
                r#east: East::Side,
            });
        }
        if state_id == 4972 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::None,
                r#north: North::None,
                r#power: 1,
            });
        }
        if state_id == 5085 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::None,
                r#power: 13,
                r#west: West::Up,
                r#east: East::None,
            });
        }
        if state_id == 4153 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#south: South::Up,
                r#power: 6,
                r#west: West::Side,
            });
        }
        if state_id == 3905 {
            return Some(RedstoneWire {
                r#power: 10,
                r#north: North::Up,
                r#west: West::None,
                r#east: East::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4700 {
            return Some(RedstoneWire {
                r#power: 2,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Up,
            });
        }
        if state_id == 3880 {
            return Some(RedstoneWire {
                r#power: 7,
                r#south: South::None,
                r#north: North::Up,
                r#east: East::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4180 {
            return Some(RedstoneWire {
                r#power: 9,
                r#south: South::Up,
                r#east: East::Up,
                r#west: West::Side,
                r#north: North::None,
            });
        }
        if state_id == 4869 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::None,
                r#north: North::Side,
                r#west: West::Up,
                r#power: 5,
            });
        }
        if state_id == 4353 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 12,
                r#west: West::Up,
                r#north: North::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4051 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 10,
                r#north: North::Side,
                r#east: East::Up,
                r#south: South::None,
            });
        }
        if state_id == 4812 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Up,
                r#power: 15,
                r#west: West::Up,
                r#east: East::None,
            });
        }
        if state_id == 4035 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Up,
                r#north: North::Side,
                r#power: 9,
                r#south: South::Up,
            });
        }
        if state_id == 4567 {
            return Some(RedstoneWire {
                r#power: 4,
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Up,
            });
        }
        if state_id == 4996 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#west: West::Side,
                r#power: 3,
                r#south: South::None,
            });
        }
        if state_id == 4298 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 6,
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4036 {
            return Some(RedstoneWire {
                r#power: 9,
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4480 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#west: West::Side,
                r#power: 10,
                r#south: South::Side,
            });
        }
        if state_id == 4704 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 3,
                r#north: North::Up,
                r#south: South::Side,
                r#east: East::None,
            });
        }
        if state_id == 4125 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::Up,
                r#power: 3,
            });
        }
        if state_id == 4974 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::None,
                r#power: 1,
                r#west: West::Up,
                r#east: East::None,
            });
        }
        if state_id == 4155 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 6,
                r#north: North::None,
                r#west: West::Up,
                r#south: South::Side,
            });
        }
        if state_id == 5007 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::None,
                r#west: West::Up,
                r#power: 5,
                r#east: East::None,
            });
        }
        if state_id == 3956 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 0,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4018 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Side,
                r#power: 7,
            });
        }
        if state_id == 4491 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 11,
                r#west: West::Up,
                r#south: South::None,
                r#north: North::Side,
            });
        }
        if state_id == 4496 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::Side,
                r#west: West::None,
                r#power: 12,
            });
        }
        if state_id == 4633 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 11,
                r#north: North::None,
                r#east: East::Side,
                r#west: West::Side,
            });
        }
        if state_id == 5064 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::Up,
                r#power: 11,
            });
        }
        if state_id == 3888 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Up,
                r#south: South::None,
                r#east: East::Up,
                r#power: 8,
            });
        }
        if state_id == 4581 {
            return Some(RedstoneWire {
                r#power: 5,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4204 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#power: 11,
                r#east: East::Up,
                r#north: North::None,
            });
        }
        if state_id == 4456 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::None,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 7,
            });
        }
        if state_id == 4124 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#power: 2,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4278 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::Up,
                r#power: 4,
                r#west: West::Up,
            });
        }
        if state_id == 5008 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 5,
                r#west: West::Side,
                r#south: South::Up,
                r#north: North::None,
            });
        }
        if state_id == 4498 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 12,
            });
        }
        if state_id == 4054 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::Side,
                r#east: East::Up,
                r#power: 11,
            });
        }
        if state_id == 4570 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 4,
                r#west: West::Side,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 4482 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 10,
                r#east: East::Side,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4228 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 14,
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::None,
            });
        }
        if state_id == 3932 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#power: 13,
                r#west: West::None,
                r#south: South::Side,
            });
        }
        if state_id == 4751 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Side,
                r#power: 8,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 4290 {
            return Some(RedstoneWire {
                r#power: 5,
                r#north: North::Up,
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4356 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#south: South::None,
                r#power: 12,
                r#north: North::Up,
            });
        }
        if state_id == 4756 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 9,
                r#east: East::None,
            });
        }
        if state_id == 4805 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 14,
                r#north: North::Up,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 4753 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 8,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Up,
            });
        }
        if state_id == 4977 {
            return Some(RedstoneWire {
                r#power: 1,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Up,
                r#south: South::None,
            });
        }
        if state_id == 3811 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Up,
                r#power: 0,
                r#east: East::Up,
                r#south: South::Up,
            });
        }
        if state_id == 3866 {
            return Some(RedstoneWire {
                r#power: 6,
                r#north: North::Up,
                r#east: East::Up,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4463 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Side,
                r#power: 8,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 4732 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Side,
                r#power: 6,
                r#east: East::None,
                r#south: South::Side,
            });
        }
        if state_id == 4378 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::Side,
                r#power: 15,
                r#south: South::Up,
            });
        }
        if state_id == 4394 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#east: East::Side,
                r#west: West::None,
                r#power: 0,
            });
        }
        if state_id == 4551 {
            return Some(RedstoneWire {
                r#power: 2,
                r#north: North::None,
                r#south: South::Side,
                r#east: East::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4327 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::Side,
                r#power: 9,
                r#south: South::Side,
            });
        }
        if state_id == 4994 {
            return Some(RedstoneWire {
                r#power: 3,
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Side,
            });
        }
        if state_id == 4701 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::Up,
                r#power: 3,
                r#east: East::None,
            });
        }
        if state_id == 4056 {
            return Some(RedstoneWire {
                r#power: 11,
                r#south: South::Side,
                r#east: East::Up,
                r#north: North::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4095 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Up,
                r#north: North::Side,
                r#west: West::Up,
                r#power: 15,
            });
        }
        if state_id == 4165 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 7,
                r#west: West::Side,
                r#north: North::None,
                r#south: South::Side,
            });
        }
        if state_id == 4174 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Side,
                r#north: North::None,
                r#west: West::Side,
                r#power: 8,
            });
        }
        if state_id == 4317 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#power: 8,
                r#south: South::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4749 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::Up,
                r#power: 8,
            });
        }
        if state_id == 4801 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::Up,
                r#east: East::None,
                r#power: 14,
                r#north: North::Up,
            });
        }
        if state_id == 4863 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::Up,
                r#power: 5,
                r#east: East::None,
                r#north: North::Side,
            });
        }
        if state_id == 4959 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Up,
                r#power: 15,
            });
        }
        if state_id == 4998 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 4,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4747 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 8,
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::None,
            });
        }
        if state_id == 4889 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::None,
                r#power: 7,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 3928 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 13,
                r#east: East::Up,
                r#south: South::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4388 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Side,
                r#power: 0,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 5051 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#west: West::None,
                r#power: 9,
            });
        }
        if state_id == 4558 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Up,
                r#power: 3,
            });
        }
        if state_id == 4100 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::None,
                r#east: East::Up,
                r#power: 0,
                r#south: South::Up,
            });
        }
        if state_id == 4464 {
            return Some(RedstoneWire {
                r#power: 8,
                r#north: North::Side,
                r#west: West::Up,
                r#south: South::None,
                r#east: East::Side,
            });
        }
        if state_id == 4679 {
            return Some(RedstoneWire {
                r#power: 0,
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 4715 {
            return Some(RedstoneWire {
                r#power: 4,
                r#east: East::None,
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 4880 {
            return Some(RedstoneWire {
                r#power: 6,
                r#south: South::None,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 3943 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::Up,
                r#power: 14,
            });
        }
        if state_id == 4069 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Side,
                r#power: 12,
                r#west: West::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4367 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Up,
                r#power: 13,
                r#east: East::Side,
                r#south: South::None,
            });
        }
        if state_id == 4532 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::None,
                r#power: 0,
                r#east: East::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4253 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 1,
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4254 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#west: West::Up,
                r#north: North::Up,
                r#power: 1,
            });
        }
        if state_id == 4350 {
            return Some(RedstoneWire {
                r#power: 12,
                r#south: South::Up,
                r#north: North::Up,
                r#east: East::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4652 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 13,
                r#north: North::None,
                r#east: East::Side,
                r#west: West::None,
            });
        }
        if state_id == 4823 {
            return Some(RedstoneWire {
                r#power: 0,
                r#north: North::Side,
                r#south: South::Side,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 4297 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 6,
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4287 {
            return Some(RedstoneWire {
                r#power: 5,
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4361 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::None,
                r#east: East::Side,
                r#north: North::Up,
                r#power: 13,
            });
        }
        if state_id == 4405 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#power: 2,
                r#west: West::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4407 {
            return Some(RedstoneWire {
                r#power: 2,
                r#west: West::Up,
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4055 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 11,
                r#south: South::Up,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 4161 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Up,
                r#north: North::None,
                r#west: West::Up,
                r#power: 7,
            });
        }
        if state_id == 4292 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::None,
                r#power: 5,
                r#east: East::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4544 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::None,
                r#power: 1,
                r#south: South::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4597 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#west: West::Side,
                r#south: South::Side,
                r#power: 7,
            });
        }
        if state_id == 4675 {
            return Some(RedstoneWire {
                r#power: 0,
                r#west: West::Side,
                r#north: North::Up,
                r#south: South::Up,
                r#east: East::None,
            });
        }
        if state_id == 4351 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 12,
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4984 {
            return Some(RedstoneWire {
                r#power: 2,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4401 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::Side,
                r#power: 1,
                r#south: South::None,
            });
        }
        if state_id == 4763 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#power: 9,
                r#north: North::Up,
                r#east: East::None,
            });
        }
        if state_id == 4757 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::None,
                r#power: 9,
                r#south: South::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4398 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#power: 1,
                r#north: North::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4738 {
            return Some(RedstoneWire {
                r#power: 7,
                r#east: East::None,
                r#north: North::Up,
                r#west: West::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4762 {
            return Some(RedstoneWire {
                r#power: 9,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Up,
            });
        }
        if state_id == 3945 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 15,
                r#south: South::Up,
                r#north: North::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4029 {
            return Some(RedstoneWire {
                r#power: 8,
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4033 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 8,
                r#north: North::Side,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4730 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#east: East::None,
                r#west: West::None,
                r#power: 6,
            });
        }
        if state_id == 4157 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 6,
                r#south: South::Side,
                r#north: North::None,
                r#east: East::Up,
            });
        }
        if state_id == 4107 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Up,
                r#north: North::None,
                r#power: 1,
                r#west: West::Up,
            });
        }
        if state_id == 4240 {
            return Some(RedstoneWire {
                r#power: 15,
                r#east: East::Up,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4607 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#west: West::None,
                r#power: 8,
                r#east: East::Side,
            });
        }
        if state_id == 4969 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Side,
                r#north: North::None,
                r#power: 0,
                r#south: South::None,
            });
        }
        if state_id == 4833 {
            return Some(RedstoneWire {
                r#power: 1,
                r#east: East::None,
                r#west: West::Up,
                r#south: South::None,
                r#north: North::Side,
            });
        }
        if state_id == 3890 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::None,
                r#power: 8,
                r#south: South::None,
            });
        }
        if state_id == 4015 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Up,
                r#south: South::None,
                r#power: 6,
                r#west: West::Side,
            });
        }
        if state_id == 4659 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 14,
                r#west: West::Up,
                r#south: South::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4906 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Side,
                r#west: West::Side,
                r#east: East::None,
                r#power: 9,
            });
        }
        if state_id == 4971 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 1,
                r#south: South::Up,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 5031 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#east: East::None,
                r#north: North::None,
                r#power: 7,
            });
        }
        if state_id == 3844 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 3,
                r#north: North::Up,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 3948 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::Side,
                r#power: 15,
            });
        }
        if state_id == 4473 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Side,
                r#east: East::Side,
                r#power: 9,
                r#west: West::Up,
            });
        }
        if state_id == 4711 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::None,
                r#south: South::Up,
                r#power: 4,
                r#north: North::Up,
            });
        }
        if state_id == 4986 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::None,
                r#west: West::Up,
                r#power: 2,
                r#east: East::None,
            });
        }
        if state_id == 4852 {
            return Some(RedstoneWire {
                r#power: 3,
                r#west: West::Side,
                r#south: South::None,
                r#north: North::Side,
                r#east: East::None,
            });
        }
        if state_id == 4922 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Side,
                r#south: South::Side,
                r#power: 11,
                r#east: East::None,
            });
        }
        if state_id == 4867 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::None,
                r#south: South::Side,
                r#north: North::Side,
                r#power: 5,
            });
        }
        if state_id == 3976 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Side,
                r#east: East::Up,
                r#west: West::Side,
                r#power: 2,
            });
        }
        if state_id == 3882 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Up,
                r#power: 8,
                r#south: South::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4080 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Side,
                r#east: East::Up,
                r#power: 14,
                r#west: West::Up,
            });
        }
        if state_id == 4416 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 3,
                r#south: South::Side,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 3934 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::Side,
                r#power: 13,
            });
        }
        if state_id == 5079 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 13,
                r#north: North::None,
                r#south: South::Up,
                r#east: East::None,
            });
        }
        if state_id == 4151 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Up,
                r#power: 5,
            });
        }
        if state_id == 4517 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 14,
                r#south: South::Side,
            });
        }
        if state_id == 4406 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::None,
                r#east: East::Side,
                r#south: South::Up,
                r#power: 2,
            });
        }
        if state_id == 4430 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Side,
                r#east: East::Side,
                r#power: 4,
                r#west: West::None,
            });
        }
        if state_id == 4001 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::Up,
                r#west: West::None,
                r#power: 5,
            });
        }
        if state_id == 4521 {
            return Some(RedstoneWire {
                r#power: 15,
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4262 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 2,
                r#south: South::Up,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 4767 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::Up,
                r#east: East::None,
                r#power: 10,
            });
        }
        if state_id == 4506 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 13,
                r#east: East::Side,
                r#south: South::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4481 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::Side,
                r#north: North::Side,
                r#east: East::Side,
                r#power: 10,
            });
        }
        if state_id == 4545 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 1,
                r#east: East::Side,
                r#west: West::Up,
                r#north: North::None,
            });
        }
        if state_id == 4277 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::None,
                r#north: North::Up,
                r#power: 3,
                r#south: South::None,
            });
        }
        if state_id == 4472 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Side,
                r#west: West::None,
                r#power: 9,
                r#east: East::Side,
            });
        }
        if state_id == 5003 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#north: North::None,
                r#power: 4,
                r#west: West::None,
            });
        }
        if state_id == 3914 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 11,
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 4837 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Side,
                r#power: 2,
                r#east: East::None,
            });
        }
        if state_id == 4859 {
            return Some(RedstoneWire {
                r#power: 4,
                r#north: North::Side,
                r#south: South::Side,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 3970 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#power: 1,
                r#west: West::Side,
                r#south: South::None,
            });
        }
        if state_id == 4436 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#east: East::Side,
                r#power: 5,
                r#west: West::None,
            });
        }
        if state_id == 4352 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 12,
                r#south: South::Up,
                r#east: East::Side,
                r#west: West::None,
            });
        }
        if state_id == 3834 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Up,
                r#east: East::Up,
                r#power: 2,
                r#west: West::Up,
            });
        }
        if state_id == 4734 {
            return Some(RedstoneWire {
                r#power: 6,
                r#south: South::None,
                r#north: North::Up,
                r#east: East::None,
                r#west: West::Up,
            });
        }
        if state_id == 4316 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 8,
                r#south: South::Up,
                r#west: West::None,
                r#east: East::Side,
            });
        }
        if state_id == 4999 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Side,
                r#south: South::Up,
                r#power: 4,
                r#east: East::None,
            });
        }
        if state_id == 4283 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Side,
                r#south: South::Side,
                r#power: 4,
                r#west: West::None,
            });
        }
        if state_id == 4219 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Side,
                r#east: East::Up,
                r#south: South::Side,
                r#power: 13,
            });
        }
        if state_id == 4205 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::None,
                r#power: 11,
                r#south: South::None,
                r#east: East::Up,
            });
        }
        if state_id == 4476 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Side,
                r#east: East::Side,
                r#power: 10,
                r#west: West::Up,
            });
        }
        if state_id == 4318 {
            return Some(RedstoneWire {
                r#power: 8,
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4503 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 13,
                r#north: North::Side,
                r#east: East::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4518 {
            return Some(RedstoneWire {
                r#power: 14,
                r#south: South::None,
                r#east: East::Side,
                r#west: West::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4764 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Up,
                r#west: West::Up,
                r#east: East::None,
                r#power: 10,
            });
        }
        if state_id == 4395 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#power: 1,
                r#west: West::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4826 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#power: 0,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 3812 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 0,
                r#south: South::Up,
                r#west: West::None,
                r#north: North::Up,
            });
        }
        if state_id == 4202 {
            return Some(RedstoneWire {
                r#power: 11,
                r#north: North::None,
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 5019 {
            return Some(RedstoneWire {
                r#power: 6,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4260 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 2,
                r#north: North::Up,
            });
        }
        if state_id == 4334 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#power: 10,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 4629 {
            return Some(RedstoneWire {
                r#power: 11,
                r#east: East::Side,
                r#west: West::Up,
                r#south: South::Up,
                r#north: North::None,
            });
        }
        if state_id == 4611 {
            return Some(RedstoneWire {
                r#power: 9,
                r#west: West::Up,
                r#north: North::None,
                r#south: South::Up,
                r#east: East::Side,
            });
        }
        if state_id == 5084 {
            return Some(RedstoneWire {
                r#power: 13,
                r#east: East::None,
                r#south: South::Side,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 4930 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 12,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4221 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#south: South::None,
                r#power: 13,
                r#west: West::Up,
            });
        }
        if state_id == 4779 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#east: East::None,
                r#power: 11,
                r#north: North::Up,
            });
        }
        if state_id == 3983 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Up,
                r#power: 3,
                r#east: East::Up,
                r#west: West::None,
            });
        }
        if state_id == 4488 {
            return Some(RedstoneWire {
                r#power: 11,
                r#north: North::Side,
                r#west: West::Up,
                r#east: East::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4255 {
            return Some(RedstoneWire {
                r#power: 1,
                r#south: South::Side,
                r#east: East::Side,
                r#west: West::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4921 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 11,
                r#north: North::Side,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 5065 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#power: 11,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4379 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#power: 15,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4111 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::None,
                r#power: 1,
            });
        }
        if state_id == 4577 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::None,
                r#power: 5,
                r#south: South::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4875 {
            return Some(RedstoneWire {
                r#power: 6,
                r#north: North::Side,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4120 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#east: East::Up,
                r#power: 2,
                r#west: West::Side,
            });
        }
        if state_id == 4369 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 14,
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4605 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#power: 8,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4428 {
            return Some(RedstoneWire {
                r#power: 4,
                r#east: East::Side,
                r#south: South::None,
                r#north: North::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4635 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 11,
                r#west: West::Up,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 5014 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Side,
                r#power: 5,
            });
        }
        if state_id == 4894 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#power: 8,
                r#north: North::Side,
                r#east: East::None,
            });
        }
        if state_id == 4879 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Side,
                r#power: 6,
            });
        }
        if state_id == 4559 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Up,
                r#power: 3,
                r#west: West::None,
            });
        }
        if state_id == 4213 {
            return Some(RedstoneWire {
                r#power: 12,
                r#west: West::Side,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Up,
            });
        }
        if state_id == 4728 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::None,
                r#north: North::Up,
                r#power: 6,
            });
        }
        if state_id == 3893 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 9,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 3887 {
            return Some(RedstoneWire {
                r#power: 8,
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 4905 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::None,
                r#power: 9,
                r#south: South::None,
                r#north: North::Side,
            });
        }
        if state_id == 4963 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 0,
                r#west: West::Side,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 4575 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Up,
                r#north: North::None,
                r#south: South::Up,
                r#power: 5,
            });
        }
        if state_id == 3886 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::Up,
                r#power: 8,
                r#north: North::Up,
            });
        }
        if state_id == 4194 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 10,
                r#east: East::Up,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 4584 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Up,
                r#south: South::Up,
                r#north: North::None,
                r#power: 6,
            });
        }
        if state_id == 4901 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::None,
                r#power: 9,
            });
        }
        if state_id == 4755 {
            return Some(RedstoneWire {
                r#power: 9,
                r#west: West::Up,
                r#east: East::None,
                r#south: South::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4831 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 1,
                r#east: East::None,
            });
        }
        if state_id == 4667 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 15,
                r#east: East::Side,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 4086 {
            return Some(RedstoneWire {
                r#power: 14,
                r#north: North::Side,
                r#south: South::None,
                r#west: West::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4942 {
            return Some(RedstoneWire {
                r#power: 13,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4431 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 5,
                r#west: West::Up,
                r#south: South::Up,
                r#north: North::Side,
            });
        }
        if state_id == 3848 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::None,
                r#power: 4,
                r#south: South::Up,
            });
        }
        if state_id == 4121 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 2,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Up,
            });
        }
        if state_id == 4263 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::Side,
                r#power: 2,
            });
        }
        if state_id == 3854 {
            return Some(RedstoneWire {
                r#power: 4,
                r#east: East::Up,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Up,
            });
        }
        if state_id == 3858 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::Side,
                r#north: North::Up,
                r#power: 5,
                r#east: East::Up,
            });
        }
        if state_id == 4299 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Up,
                r#east: East::Side,
                r#power: 6,
                r#south: South::Side,
            });
        }
        if state_id == 3878 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Side,
                r#power: 7,
                r#east: East::Up,
                r#west: West::None,
            });
        }
        if state_id == 4504 {
            return Some(RedstoneWire {
                r#power: 13,
                r#east: East::Side,
                r#west: West::Side,
                r#south: South::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4271 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 3,
                r#west: West::None,
                r#north: North::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4549 {
            return Some(RedstoneWire {
                r#power: 2,
                r#west: West::Side,
                r#south: South::Up,
                r#north: North::None,
                r#east: East::Side,
            });
        }
        if state_id == 4686 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#north: North::Up,
                r#west: West::Up,
                r#power: 1,
            });
        }
        if state_id == 4829 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Up,
                r#power: 1,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 4181 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::None,
                r#power: 9,
                r#east: East::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4839 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 2,
                r#east: East::None,
                r#north: North::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4886 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#power: 7,
                r#north: North::Side,
                r#west: West::None,
            });
        }
        if state_id == 3895 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Side,
                r#east: East::Up,
                r#power: 9,
                r#west: West::Side,
            });
        }
        if state_id == 4976 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Side,
                r#power: 1,
                r#west: West::None,
            });
        }
        if state_id == 5002 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::None,
                r#east: East::None,
                r#power: 4,
                r#south: South::Side,
            });
        }
        if state_id == 5034 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::None,
                r#west: West::Up,
                r#power: 8,
                r#north: North::None,
            });
        }
        if state_id == 5098 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Up,
                r#power: 15,
                r#west: West::Side,
            });
        }
        if state_id == 4703 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 3,
                r#south: South::Up,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 3815 {
            return Some(RedstoneWire {
                r#power: 0,
                r#north: North::Up,
                r#south: South::Side,
                r#east: East::Up,
                r#west: West::None,
            });
        }
        if state_id == 3851 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Up,
                r#north: North::Up,
                r#power: 4,
                r#west: West::None,
            });
        }
        if state_id == 3870 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Up,
                r#south: South::None,
                r#east: East::Up,
                r#power: 6,
            });
        }
        if state_id == 4571 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::None,
                r#south: South::Side,
                r#power: 4,
                r#east: East::Side,
            });
        }
        if state_id == 4982 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
                r#power: 2,
                r#south: South::Up,
            });
        }
        if state_id == 4698 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Up,
                r#north: North::Up,
                r#south: South::None,
                r#power: 2,
            });
        }
        if state_id == 4364 {
            return Some(RedstoneWire {
                r#power: 13,
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::None,
                r#east: East::Side,
            });
        }
        if state_id == 4166 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#west: West::None,
                r#power: 7,
                r#south: South::Side,
            });
        }
        if state_id == 4616 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::None,
                r#power: 9,
                r#east: East::Side,
                r#south: South::Side,
            });
        }
        if state_id == 3872 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::None,
                r#north: North::Up,
                r#power: 6,
                r#east: East::Up,
            });
        }
        if state_id == 3986 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 3,
                r#south: South::Side,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 3826 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Side,
                r#power: 1,
            });
        }
        if state_id == 4417 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 3,
                r#south: South::Side,
                r#north: North::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4936 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 13,
                r#east: East::None,
                r#north: North::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4919 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Side,
                r#power: 11,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 4671 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#south: South::None,
                r#power: 15,
                r#north: North::None,
            });
        }
        if state_id == 4647 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Up,
                r#east: East::Side,
                r#power: 13,
                r#south: South::Up,
            });
        }
        if state_id == 4448 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Side,
                r#power: 6,
            });
        }
        if state_id == 3903 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Side,
                r#east: East::Up,
                r#power: 10,
                r#west: West::Up,
            });
        }
        if state_id == 3868 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 6,
            });
        }
        if state_id == 3902 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Up,
                r#north: North::Up,
                r#west: West::None,
                r#power: 10,
            });
        }
        if state_id == 4012 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 6,
                r#east: East::Up,
                r#north: North::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4935 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 13,
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::None,
            });
        }
        if state_id == 4800 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#power: 14,
                r#west: West::Up,
                r#east: East::None,
            });
        }
        if state_id == 4621 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Side,
                r#north: North::None,
                r#south: South::Up,
                r#power: 10,
            });
        }
        if state_id == 4725 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Up,
                r#power: 5,
                r#south: South::None,
                r#north: North::Up,
            });
        }
        if state_id == 3891 {
            return Some(RedstoneWire {
                r#power: 9,
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::Up,
                r#east: East::Up,
            });
        }
        if state_id == 3885 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 8,
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4387 {
            return Some(RedstoneWire {
                r#power: 0,
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4403 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Side,
                r#west: West::None,
                r#east: East::Side,
                r#power: 1,
            });
        }
        if state_id == 4233 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Up,
                r#power: 15,
                r#north: North::None,
                r#west: West::Up,
            });
        }
        if state_id == 4465 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#east: East::Side,
                r#west: West::Side,
                r#power: 8,
            });
        }
        if state_id == 4264 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 2,
                r#south: South::Side,
                r#east: East::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4640 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::Up,
                r#north: North::None,
                r#power: 12,
                r#east: East::Side,
            });
        }
        if state_id == 4307 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Up,
                r#power: 7,
                r#east: East::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4259 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#north: North::Up,
                r#east: East::Side,
                r#power: 1,
            });
        }
        if state_id == 4126 {
            return Some(RedstoneWire {
                r#power: 3,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::None,
            });
        }
        if state_id == 4312 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::None,
                r#power: 7,
                r#east: East::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4733 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Up,
                r#power: 6,
                r#south: South::Side,
            });
        }
        if state_id == 4808 {
            return Some(RedstoneWire {
                r#power: 14,
                r#north: North::Up,
                r#east: East::None,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4168 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 7,
                r#north: North::None,
                r#east: East::Up,
                r#west: West::Side,
            });
        }
        if state_id == 3999 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 5,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4185 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#power: 9,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4724 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::None,
                r#power: 5,
            });
        }
        if state_id == 4870 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::None,
                r#north: North::Side,
                r#power: 5,
                r#west: West::Side,
            });
        }
        if state_id == 4892 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 8,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Side,
            });
        }
        if state_id == 4279 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Side,
                r#power: 4,
                r#west: West::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4853 {
            return Some(RedstoneWire {
                r#power: 3,
                r#north: North::Side,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 4877 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#east: East::None,
                r#power: 6,
                r#west: West::None,
            });
        }
        if state_id == 4309 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 7,
                r#east: East::Side,
            });
        }
        if state_id == 4928 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 12,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4101 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#power: 0,
                r#west: West::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4427 {
            return Some(RedstoneWire {
                r#power: 4,
                r#east: East::Side,
                r#west: West::None,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4249 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#south: South::None,
                r#north: North::Up,
                r#power: 0,
            });
        }
        if state_id == 4373 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::None,
                r#power: 14,
            });
        }
        if state_id == 4788 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 12,
                r#north: North::Up,
                r#west: West::Up,
                r#south: South::None,
            });
        }
        if state_id == 4806 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#power: 14,
                r#north: North::Up,
                r#east: East::None,
            });
        }
        if state_id == 4047 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Side,
                r#north: North::Side,
                r#west: West::Up,
                r#power: 10,
            });
        }
        if state_id == 3810 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 0,
            });
        }
        if state_id == 3864 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Up,
                r#south: South::Up,
                r#power: 6,
                r#east: East::Up,
            });
        }
        if state_id == 4144 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::None,
                r#power: 5,
            });
        }
        if state_id == 4412 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::None,
                r#north: North::Side,
                r#west: West::None,
                r#power: 2,
            });
        }
        if state_id == 4970 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#power: 0,
                r#west: West::None,
            });
        }
        if state_id == 4258 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::None,
                r#east: East::Side,
                r#north: North::Up,
                r#power: 1,
            });
        }
        if state_id == 4501 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#east: East::Side,
                r#west: West::Side,
                r#power: 12,
            });
        }
        if state_id == 4187 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#power: 9,
            });
        }
        if state_id == 4377 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 15,
                r#north: North::Up,
                r#west: West::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4897 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 8,
                r#north: North::Side,
                r#west: West::Side,
                r#east: East::None,
            });
        }
        if state_id == 4105 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 0,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Up,
            });
        }
        if state_id == 4288 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 5,
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4332 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 10,
                r#north: North::Up,
                r#east: East::Side,
                r#south: South::Up,
            });
        }
        if state_id == 3831 {
            return Some(RedstoneWire {
                r#power: 2,
                r#west: West::Up,
                r#east: East::Up,
                r#south: South::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4413 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Up,
                r#south: South::Up,
                r#north: North::Side,
                r#power: 3,
            });
        }
        if state_id == 3995 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::Side,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 4,
            });
        }
        if state_id == 4868 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 5,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 4933 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 12,
                r#west: West::Side,
                r#north: North::Side,
                r#south: South::None,
            });
        }
        if state_id == 4861 {
            return Some(RedstoneWire {
                r#power: 4,
                r#north: North::Side,
                r#south: South::None,
                r#east: East::None,
                r#west: West::Side,
            });
        }
        if state_id == 5097 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Up,
                r#north: North::None,
                r#south: South::Up,
                r#power: 15,
            });
        }
        if state_id == 4644 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 12,
                r#north: North::None,
                r#east: East::Side,
                r#south: South::None,
            });
        }
        if state_id == 4160 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Up,
                r#power: 6,
            });
        }
        if state_id == 4737 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Up,
                r#east: East::None,
                r#power: 7,
                r#west: West::Up,
            });
        }
        if state_id == 4574 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Side,
                r#power: 4,
            });
        }
        if state_id == 3901 {
            return Some(RedstoneWire {
                r#power: 10,
                r#east: East::Up,
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4328 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::None,
                r#north: North::Up,
                r#east: East::Side,
                r#power: 9,
            });
        }
        if state_id == 4815 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 15,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4948 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::None,
                r#south: South::Side,
                r#power: 14,
                r#north: North::Side,
            });
        }
        if state_id == 4707 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Up,
                r#power: 3,
                r#east: East::None,
                r#west: West::Up,
            });
        }
        if state_id == 4718 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::None,
                r#east: East::None,
                r#power: 4,
                r#south: South::None,
            });
        }
        if state_id == 4132 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Side,
                r#power: 3,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 4771 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Side,
                r#north: North::Up,
                r#south: South::None,
                r#power: 10,
            });
        }
        if state_id == 4911 {
            return Some(RedstoneWire {
                r#power: 10,
                r#south: South::Side,
                r#north: North::Side,
                r#west: West::Up,
                r#east: East::None,
            });
        }
        if state_id == 4980 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Up,
                r#power: 2,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 3975 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Up,
                r#east: East::Up,
                r#power: 2,
                r#south: South::Side,
            });
        }
        if state_id == 4400 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 1,
                r#west: West::None,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4979 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 1,
                r#north: North::None,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4214 {
            return Some(RedstoneWire {
                r#power: 12,
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Up,
            });
        }
        if state_id == 3847 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Up,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 4,
            });
        }
        if state_id == 4925 {
            return Some(RedstoneWire {
                r#power: 11,
                r#east: East::None,
                r#north: North::Side,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 4122 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#power: 2,
                r#west: West::Up,
                r#south: South::None,
            });
        }
        if state_id == 4580 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Side,
                r#power: 5,
            });
        }
        if state_id == 4799 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Up,
                r#power: 13,
            });
        }
        if state_id == 4145 {
            return Some(RedstoneWire {
                r#power: 5,
                r#south: South::Up,
                r#west: West::None,
                r#east: East::Up,
                r#north: North::None,
            });
        }
        if state_id == 3998 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::None,
                r#north: North::Side,
                r#power: 4,
                r#east: East::Up,
            });
        }
        if state_id == 3978 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 2,
                r#north: North::Side,
                r#west: West::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4375 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::Side,
                r#south: South::None,
                r#power: 14,
            });
        }
        if state_id == 4304 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#west: West::None,
                r#south: South::None,
                r#power: 6,
            });
        }
        if state_id == 4060 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 11,
                r#west: West::Side,
                r#east: East::Up,
                r#south: South::None,
            });
        }
        if state_id == 4525 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#power: 15,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4660 {
            return Some(RedstoneWire {
                r#power: 14,
                r#east: East::Side,
                r#west: West::Side,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 4226 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Up,
                r#power: 14,
            });
        }
        if state_id == 4269 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::Up,
                r#west: West::Up,
                r#power: 3,
            });
        }
        if state_id == 4965 {
            return Some(RedstoneWire {
                r#power: 0,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Up,
                r#south: South::Side,
            });
        }
        if state_id == 5078 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#power: 12,
            });
        }
        if state_id == 4586 {
            return Some(RedstoneWire {
                r#power: 6,
                r#south: South::Up,
                r#west: West::None,
                r#east: East::Side,
                r#north: North::None,
            });
        }
        if state_id == 4441 {
            return Some(RedstoneWire {
                r#power: 6,
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4896 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#west: West::Up,
                r#south: South::None,
                r#power: 8,
            });
        }
        if state_id == 4599 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Side,
                r#north: North::None,
                r#west: West::Up,
                r#power: 7,
            });
        }
        if state_id == 3981 {
            return Some(RedstoneWire {
                r#power: 3,
                r#west: West::Up,
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4321 {
            return Some(RedstoneWire {
                r#power: 8,
                r#south: South::None,
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4396 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 1,
                r#east: East::Side,
                r#north: North::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4073 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 13,
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::None,
            });
        }
        if state_id == 4626 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Up,
                r#east: East::Side,
                r#power: 10,
                r#south: South::None,
            });
        }
        if state_id == 3900 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Up,
                r#east: East::Up,
                r#power: 10,
                r#west: West::Up,
            });
        }
        if state_id == 4010 {
            return Some(RedstoneWire {
                r#power: 6,
                r#west: West::None,
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4960 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 15,
                r#west: West::Side,
                r#north: North::Side,
                r#south: South::None,
            });
        }
        if state_id == 4620 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::None,
                r#west: West::Up,
                r#power: 10,
            });
        }
        if state_id == 4533 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Up,
                r#east: East::Side,
                r#power: 0,
            });
        }
        if state_id == 4663 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Side,
                r#east: East::Side,
                r#power: 14,
                r#south: South::None,
            });
        }
        if state_id == 4089 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 15,
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4225 {
            return Some(RedstoneWire {
                r#power: 14,
                r#east: East::Up,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 5095 {
            return Some(RedstoneWire {
                r#power: 14,
                r#west: West::Side,
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 4489 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::Side,
                r#power: 11,
                r#east: East::Side,
            });
        }
        if state_id == 4354 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 12,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4329 {
            return Some(RedstoneWire {
                r#power: 9,
                r#west: West::Up,
                r#north: North::Up,
                r#east: East::Side,
                r#south: South::None,
            });
        }
        if state_id == 4774 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::None,
                r#south: South::Up,
                r#power: 11,
            });
        }
        if state_id == 4798 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 13,
                r#west: West::Side,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 4912 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#north: North::Side,
                r#power: 10,
                r#west: West::Side,
            });
        }
        if state_id == 4554 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::None,
                r#east: East::Side,
                r#power: 2,
                r#south: South::None,
            });
        }
        if state_id == 4835 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 1,
                r#east: East::None,
                r#north: North::Side,
                r#west: West::None,
            });
        }
        if state_id == 5046 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Up,
                r#north: North::None,
                r#south: South::Side,
                r#power: 9,
            });
        }
        if state_id == 4528 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Side,
                r#east: East::Side,
                r#power: 15,
            });
        }
        if state_id == 5087 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#west: West::None,
                r#power: 13,
            });
        }
        if state_id == 4790 {
            return Some(RedstoneWire {
                r#power: 12,
                r#north: North::Up,
                r#east: East::None,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4186 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Side,
                r#south: South::None,
                r#north: North::None,
                r#power: 9,
            });
        }
        if state_id == 4562 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#north: North::None,
                r#power: 3,
                r#west: West::None,
            });
        }
        if state_id == 4294 {
            return Some(RedstoneWire {
                r#power: 5,
                r#north: North::Up,
                r#south: South::None,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4495 {
            return Some(RedstoneWire {
                r#power: 12,
                r#west: West::Side,
                r#north: North::Side,
                r#east: East::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4610 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#power: 8,
                r#east: East::Side,
            });
        }
        if state_id == 5077 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::None,
                r#power: 12,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 3968 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 1,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 5036 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::None,
                r#power: 8,
            });
        }
        if state_id == 3842 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::None,
                r#east: East::Up,
                r#power: 3,
            });
        }
        if state_id == 5090 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Up,
                r#north: North::None,
                r#west: West::None,
                r#power: 14,
            });
        }
        if state_id == 4638 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#south: South::Up,
                r#power: 12,
                r#west: West::Up,
            });
        }
        if state_id == 4072 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::Side,
                r#power: 13,
            });
        }
        if state_id == 4106 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#south: South::None,
                r#power: 0,
                r#north: North::None,
            });
        }
        if state_id == 4842 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#south: South::None,
                r#east: East::None,
                r#power: 2,
            });
        }
        if state_id == 3814 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 0,
                r#north: North::Up,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4500 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Side,
                r#power: 12,
                r#west: West::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4791 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Up,
                r#west: West::Up,
                r#east: East::None,
                r#power: 13,
            });
        }
        if state_id == 5044 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 9,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4860 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::None,
                r#power: 4,
                r#south: South::None,
            });
        }
        if state_id == 3972 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 2,
                r#north: North::Side,
            });
        }
        if state_id == 4614 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#west: West::Up,
                r#north: North::None,
                r#power: 9,
            });
        }
        if state_id == 4182 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Up,
                r#power: 9,
                r#east: East::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4716 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Up,
                r#power: 4,
            });
        }
        if state_id == 4061 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 11,
                r#north: North::Side,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4418 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::None,
                r#power: 3,
            });
        }
        if state_id == 4461 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#power: 8,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 5054 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#power: 10,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4914 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Side,
                r#west: West::Up,
                r#power: 10,
                r#east: East::None,
            });
        }
        if state_id == 4370 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 14,
                r#south: South::Up,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 4034 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Up,
                r#south: South::None,
                r#power: 8,
                r#west: West::None,
            });
        }
        if state_id == 4591 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#south: South::None,
                r#north: North::None,
                r#power: 6,
            });
        }
        if state_id == 4209 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::None,
                r#south: South::Side,
                r#power: 12,
                r#east: East::Up,
            });
        }
        if state_id == 3939 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#power: 14,
                r#north: North::Up,
                r#east: East::Up,
            });
        }
        if state_id == 3950 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::None,
                r#east: East::Up,
                r#north: North::Up,
                r#power: 15,
            });
        }
        if state_id == 4085 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Side,
                r#power: 14,
                r#north: North::Side,
                r#west: West::None,
            });
        }
        if state_id == 4040 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::None,
                r#east: East::Up,
                r#power: 9,
                r#north: North::Side,
            });
        }
        if state_id == 4726 {
            return Some(RedstoneWire {
                r#power: 5,
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 4462 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::Side,
                r#power: 8,
            });
        }
        if state_id == 4460 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::None,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 8,
            });
        }
        if state_id == 3997 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#power: 4,
                r#south: South::None,
                r#north: North::Side,
            });
        }
        if state_id == 4021 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 7,
                r#north: North::Side,
                r#south: South::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4777 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::None,
                r#power: 11,
            });
        }
        if state_id == 4429 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Side,
                r#power: 4,
                r#south: South::None,
                r#east: East::Side,
            });
        }
        if state_id == 4338 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#power: 10,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4455 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Up,
                r#east: East::Side,
                r#power: 7,
                r#south: South::None,
            });
        }
        if state_id == 4210 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#west: West::Side,
                r#south: South::Side,
                r#power: 12,
            });
        }
        if state_id == 4743 {
            return Some(RedstoneWire {
                r#power: 7,
                r#east: East::None,
                r#west: West::Up,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 3947 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#power: 15,
                r#south: South::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4778 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Up,
                r#power: 11,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4339 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Side,
                r#west: West::Side,
                r#power: 10,
                r#south: South::None,
            });
        }
        if state_id == 4389 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 0,
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4845 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::None,
                r#north: North::Side,
                r#power: 3,
                r#south: South::Up,
            });
        }
        if state_id == 4632 {
            return Some(RedstoneWire {
                r#power: 11,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4642 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#power: 12,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4042 {
            return Some(RedstoneWire {
                r#power: 9,
                r#east: East::Up,
                r#west: West::Side,
                r#south: South::None,
                r#north: North::Side,
            });
        }
        if state_id == 4467 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Up,
                r#power: 9,
            });
        }
        if state_id == 4745 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 7,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Up,
            });
        }
        if state_id == 4516 {
            return Some(RedstoneWire {
                r#power: 14,
                r#east: East::Side,
                r#west: West::Side,
                r#south: South::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4325 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#power: 9,
                r#north: North::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4032 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#power: 8,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4997 {
            return Some(RedstoneWire {
                r#power: 3,
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 4192 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 10,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 5071 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Up,
                r#power: 12,
                r#west: West::Side,
            });
        }
        if state_id == 3964 {
            return Some(RedstoneWire {
                r#power: 1,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4148 {
            return Some(RedstoneWire {
                r#power: 5,
                r#south: South::Side,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Up,
            });
        }
        if state_id == 4128 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Side,
                r#north: North::None,
                r#power: 3,
                r#west: West::Up,
            });
        }
        if state_id == 4062 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Up,
                r#south: South::Up,
                r#power: 12,
                r#east: East::Up,
            });
        }
        if state_id == 4038 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#power: 9,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4076 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Up,
                r#west: West::None,
                r#north: North::Side,
                r#power: 13,
            });
        }
        if state_id == 4368 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::Side,
                r#power: 14,
                r#north: North::Up,
            });
        }
        if state_id == 4426 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 4,
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 3911 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::Up,
                r#power: 11,
                r#west: West::None,
            });
        }
        if state_id == 4576 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Up,
                r#east: East::Side,
                r#west: West::Side,
                r#power: 5,
            });
        }
        if state_id == 4078 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Side,
                r#power: 13,
            });
        }
        if state_id == 4414 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 3,
                r#south: South::Up,
                r#north: North::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4609 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Side,
                r#south: South::None,
                r#power: 8,
                r#east: East::Side,
            });
        }
        if state_id == 4104 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::None,
                r#power: 0,
                r#south: South::None,
            });
        }
        if state_id == 4710 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#east: East::None,
                r#west: West::Up,
                r#power: 4,
            });
        }
        if state_id == 4881 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 7,
                r#east: East::None,
                r#north: North::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4070 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 12,
                r#west: West::None,
                r#north: North::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4541 {
            return Some(RedstoneWire {
                r#power: 1,
                r#north: North::None,
                r#east: East::Side,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 3923 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Side,
                r#north: North::Up,
                r#power: 12,
                r#west: West::None,
            });
        }
        if state_id == 4995 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Up,
                r#power: 3,
            });
        }
        if state_id == 4084 {
            return Some(RedstoneWire {
                r#power: 14,
                r#north: North::Side,
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 5062 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 11,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Up,
            });
        }
        if state_id == 4836 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::None,
                r#power: 2,
                r#south: South::Up,
            });
        }
        if state_id == 4197 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 11,
                r#north: North::None,
                r#east: East::Up,
                r#west: West::Up,
            });
        }
        if state_id == 5029 {
            return Some(RedstoneWire {
                r#power: 7,
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 4773 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 11,
                r#north: North::Up,
                r#west: West::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4917 {
            return Some(RedstoneWire {
                r#power: 11,
                r#east: East::None,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4648 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 13,
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::None,
            });
        }
        if state_id == 4116 {
            return Some(RedstoneWire {
                r#power: 2,
                r#north: North::None,
                r#west: West::Up,
                r#east: East::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4265 {
            return Some(RedstoneWire {
                r#power: 2,
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4850 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Side,
                r#north: North::Side,
                r#west: West::None,
                r#power: 3,
            });
        }
        if state_id == 4435 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 5,
                r#north: North::Side,
            });
        }
        if state_id == 4688 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 1,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Up,
            });
        }
        if state_id == 5025 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 7,
                r#west: West::Up,
                r#south: South::Up,
                r#north: North::None,
            });
        }
        if state_id == 4422 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Up,
                r#power: 4,
                r#north: North::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4924 {
            return Some(RedstoneWire {
                r#power: 11,
                r#south: South::None,
                r#east: East::None,
                r#west: West::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4236 {
            return Some(RedstoneWire {
                r#power: 15,
                r#east: East::Up,
                r#north: North::None,
                r#west: West::Up,
                r#south: South::Side,
            });
        }
        if state_id == 5022 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::None,
                r#power: 6,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 4981 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Side,
                r#power: 2,
            });
        }
        if state_id == 5041 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#west: West::Side,
                r#south: South::None,
                r#power: 8,
            });
        }
        if state_id == 4296 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#east: East::Side,
                r#west: West::Up,
                r#power: 6,
            });
        }
        if state_id == 3955 {
            return Some(RedstoneWire {
                r#power: 0,
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::Up,
                r#west: West::Side,
            });
        }
        if state_id == 5083 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 13,
                r#west: West::Side,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 5086 {
            return Some(RedstoneWire {
                r#power: 13,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Side,
                r#east: East::None,
            });
        }
        if state_id == 4534 {
            return Some(RedstoneWire {
                r#power: 0,
                r#north: North::None,
                r#east: East::Side,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 5069 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 11,
                r#east: East::None,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4171 {
            return Some(RedstoneWire {
                r#power: 8,
                r#west: West::Side,
                r#east: East::Up,
                r#south: South::Up,
                r#north: North::None,
            });
        }
        if state_id == 4099 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 0,
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4834 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::None,
                r#west: West::Side,
                r#power: 1,
                r#north: North::Side,
            });
        }
        if state_id == 4348 {
            return Some(RedstoneWire {
                r#power: 11,
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::Side,
                r#south: South::None,
            });
        }
        if state_id == 5091 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::None,
                r#east: East::None,
                r#power: 14,
                r#south: South::Side,
            });
        }
        if state_id == 4272 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Up,
                r#power: 3,
                r#east: East::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4227 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::None,
                r#east: East::Up,
                r#power: 14,
            });
        }
        if state_id == 3930 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 13,
                r#north: North::Up,
                r#east: East::Up,
                r#west: West::Up,
            });
        }
        if state_id == 5074 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#north: North::None,
                r#power: 12,
                r#west: West::Side,
            });
        }
        if state_id == 4487 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#power: 11,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4766 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::None,
                r#south: South::Up,
                r#east: East::None,
                r#power: 10,
            });
        }
        if state_id == 4731 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 6,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4744 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 7,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 3987 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 3,
                r#south: South::None,
                r#north: North::Side,
                r#west: West::Up,
            });
        }
        if state_id == 5089 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#west: West::Side,
                r#south: South::Up,
                r#power: 14,
            });
        }
        if state_id == 4031 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::None,
                r#north: North::Side,
                r#south: South::Side,
                r#power: 8,
            });
        }
        if state_id == 3853 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#power: 4,
                r#south: South::None,
                r#north: North::Up,
            });
        }
        if state_id == 5088 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Up,
                r#power: 14,
            });
        }
        if state_id == 4884 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#power: 7,
                r#west: West::Up,
                r#south: South::Side,
            });
        }
        if state_id == 5001 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 4,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4736 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 6,
                r#south: South::None,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 5055 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Side,
                r#power: 10,
                r#west: West::Up,
            });
        }
        if state_id == 4961 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::None,
                r#north: North::Side,
                r#west: West::None,
                r#power: 15,
            });
        }
        if state_id == 4081 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Side,
                r#east: East::Up,
                r#power: 14,
                r#west: West::Side,
            });
        }
        if state_id == 4381 {
            return Some(RedstoneWire {
                r#power: 15,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4190 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::None,
                r#power: 10,
                r#south: South::Up,
                r#north: North::None,
            });
        }
        if state_id == 5023 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#east: East::None,
                r#north: North::None,
                r#power: 6,
            });
        }
        if state_id == 4505 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#power: 13,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4515 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#power: 14,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4786 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 12,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4360 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 13,
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4885 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#power: 7,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4760 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Side,
                r#power: 9,
            });
        }
        if state_id == 5033 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::None,
                r#power: 7,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 4248 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 0,
                r#west: West::Up,
                r#south: South::None,
                r#north: North::Up,
            });
        }
        if state_id == 3982 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Side,
                r#power: 3,
                r#south: South::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4908 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 10,
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::Side,
            });
        }
        if state_id == 5082 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::None,
                r#east: East::None,
                r#power: 13,
                r#west: West::Up,
            });
        }
        if state_id == 4315 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 8,
                r#west: West::Side,
                r#north: North::Up,
                r#south: South::Up,
            });
        }
        if state_id == 3988 {
            return Some(RedstoneWire {
                r#power: 3,
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::None,
            });
        }
        if state_id == 4176 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#power: 8,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4920 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Side,
                r#west: West::Up,
                r#power: 11,
                r#east: East::None,
            });
        }
        if state_id == 4944 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 14,
            });
        }
        if state_id == 4239 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#power: 15,
                r#west: West::Up,
                r#south: South::None,
            });
        }
        if state_id == 4804 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Side,
                r#north: North::Up,
                r#power: 14,
                r#west: West::Side,
            });
        }
        if state_id == 4876 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Side,
                r#east: East::None,
                r#power: 6,
                r#west: West::Side,
            });
        }
        if state_id == 3876 {
            return Some(RedstoneWire {
                r#power: 7,
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4666 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 15,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Up,
            });
        }
        if state_id == 4135 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::None,
                r#west: West::Side,
                r#power: 4,
                r#east: East::Up,
            });
        }
        if state_id == 4630 {
            return Some(RedstoneWire {
                r#power: 11,
                r#west: West::Side,
                r#north: North::None,
                r#east: East::Side,
                r#south: South::Up,
            });
        }
        if state_id == 3958 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 0,
                r#north: North::Side,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4195 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 10,
                r#east: East::Up,
                r#west: West::Side,
                r#south: South::None,
            });
        }
        if state_id == 4783 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 12,
                r#north: North::Up,
                r#west: West::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4088 {
            return Some(RedstoneWire {
                r#power: 14,
                r#south: South::None,
                r#north: North::Side,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 4026 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Up,
                r#power: 8,
                r#west: West::Up,
                r#south: South::Up,
            });
        }
        if state_id == 3846 {
            return Some(RedstoneWire {
                r#power: 4,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4531 {
            return Some(RedstoneWire {
                r#power: 0,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4409 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 2,
                r#south: South::Side,
                r#east: East::Side,
                r#west: West::None,
            });
        }
        if state_id == 4002 {
            return Some(RedstoneWire {
                r#power: 5,
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4004 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::None,
                r#power: 5,
                r#north: North::Side,
                r#east: East::Up,
            });
        }
        if state_id == 5052 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Up,
                r#power: 10,
            });
        }
        if state_id == 4075 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#power: 13,
                r#south: South::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4568 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Up,
                r#power: 4,
            });
        }
        if state_id == 5045 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::None,
                r#south: South::Up,
                r#power: 9,
                r#north: North::None,
            });
        }
        if state_id == 4094 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 15,
                r#east: East::Up,
                r#north: North::Side,
                r#west: West::None,
            });
        }
        if state_id == 4336 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#power: 10,
                r#north: North::Up,
                r#west: West::Side,
            });
        }
        if state_id == 3833 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 2,
                r#east: East::Up,
                r#south: South::Side,
                r#north: North::Up,
            });
        }
        if state_id == 3896 {
            return Some(RedstoneWire {
                r#power: 9,
                r#east: East::Up,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4158 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#west: West::Up,
                r#south: South::None,
                r#power: 6,
            });
        }
        if state_id == 4206 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::None,
                r#power: 12,
                r#east: East::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4000 {
            return Some(RedstoneWire {
                r#power: 5,
                r#east: East::Up,
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4857 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#power: 4,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4065 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Side,
                r#north: North::Side,
                r#power: 12,
                r#west: West::Up,
            });
        }
        if state_id == 4510 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#power: 13,
                r#west: West::Side,
                r#south: South::None,
            });
        }
        if state_id == 3946 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 15,
                r#south: South::Up,
            });
        }
        if state_id == 3992 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 4,
                r#north: North::Side,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 3990 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 4,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4681 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 0,
                r#east: East::None,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 3830 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Up,
                r#west: West::None,
                r#north: North::Up,
                r#power: 2,
            });
        }
        if state_id == 4199 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 11,
                r#south: South::Up,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 4927 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Side,
                r#north: North::Side,
                r#power: 12,
                r#south: South::Up,
            });
        }
        if state_id == 4590 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::None,
                r#east: East::Side,
                r#power: 6,
                r#west: West::Up,
            });
        }
        if state_id == 4691 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::None,
                r#power: 1,
                r#north: North::Up,
                r#east: East::None,
            });
        }
        if state_id == 4871 {
            return Some(RedstoneWire {
                r#power: 5,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Side,
            });
        }
        if state_id == 4118 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::None,
                r#north: North::None,
                r#power: 2,
                r#south: South::Up,
            });
        }
        if state_id == 3912 {
            return Some(RedstoneWire {
                r#power: 11,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Up,
                r#east: East::Up,
            });
        }
        if state_id == 3944 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::None,
                r#west: West::None,
                r#power: 14,
                r#east: East::Up,
            });
        }
        if state_id == 4593 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::Up,
                r#north: North::None,
                r#power: 7,
                r#east: East::Side,
            });
        }
        if state_id == 3822 {
            return Some(RedstoneWire {
                r#power: 1,
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4678 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Up,
                r#power: 0,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4748 {
            return Some(RedstoneWire {
                r#power: 8,
                r#west: West::None,
                r#north: North::Up,
                r#south: South::Up,
                r#east: East::None,
            });
        }
        if state_id == 4862 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Side,
                r#power: 4,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 4900 {
            return Some(RedstoneWire {
                r#power: 9,
                r#east: East::None,
                r#west: West::Side,
                r#north: North::Side,
                r#south: South::Up,
            });
        }
        if state_id == 3937 {
            return Some(RedstoneWire {
                r#power: 14,
                r#west: West::Side,
                r#south: South::Up,
                r#east: East::Up,
                r#north: North::Up,
            });
        }
        if state_id == 3813 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#south: South::Side,
                r#power: 0,
                r#west: West::Up,
            });
        }
        if state_id == 4846 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::None,
                r#north: North::Side,
                r#power: 3,
                r#west: West::Side,
            });
        }
        if state_id == 4987 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#north: North::None,
                r#power: 2,
                r#east: East::None,
            });
        }
        if state_id == 4898 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#power: 8,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 5013 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#west: West::Up,
                r#power: 5,
                r#south: South::None,
            });
        }
        if state_id == 5028 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 7,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Up,
            });
        }
        if state_id == 5042 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::None,
                r#power: 8,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 5081 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 13,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4224 {
            return Some(RedstoneWire {
                r#power: 14,
                r#north: North::None,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 3977 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 2,
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 3840 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Up,
                r#north: North::Up,
                r#power: 3,
                r#south: South::Side,
            });
        }
        if state_id == 4493 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 11,
                r#south: South::None,
                r#north: North::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4918 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Side,
                r#east: East::None,
                r#power: 11,
                r#south: South::Up,
            });
        }
        if state_id == 4524 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#power: 15,
                r#west: West::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4695 {
            return Some(RedstoneWire {
                r#power: 2,
                r#north: North::Up,
                r#west: West::Up,
                r#east: East::None,
                r#south: South::Side,
            });
        }
        if state_id == 4097 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#west: West::None,
                r#south: South::None,
                r#power: 15,
            });
        }
        if state_id == 4211 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::None,
                r#east: East::Up,
                r#west: West::None,
                r#power: 12,
            });
        }
        if state_id == 5063 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 11,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 4855 {
            return Some(RedstoneWire {
                r#power: 4,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4424 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 4,
                r#west: West::None,
                r#north: North::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4741 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Side,
                r#power: 7,
                r#west: West::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4218 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::None,
                r#power: 13,
                r#east: East::Up,
            });
        }
        if state_id == 4390 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Side,
                r#power: 0,
                r#west: West::Side,
            });
        }
        if state_id == 4926 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::None,
                r#north: North::Side,
                r#power: 12,
            });
        }
        if state_id == 4275 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Up,
                r#north: North::Up,
                r#south: South::None,
                r#power: 3,
            });
        }
        if state_id == 5104 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::None,
                r#power: 15,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 4793 {
            return Some(RedstoneWire {
                r#power: 13,
                r#north: North::Up,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4956 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::Up,
                r#power: 15,
            });
        }
        if state_id == 4058 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 11,
                r#south: South::Side,
            });
        }
        if state_id == 5058 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::None,
                r#power: 10,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 4083 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 14,
                r#west: West::Up,
                r#south: South::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4509 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#power: 13,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4048 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 10,
                r#west: West::Side,
                r#east: East::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4365 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 13,
                r#west: West::Up,
                r#east: East::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4958 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::None,
                r#south: South::Side,
                r#east: East::None,
                r#power: 15,
            });
        }
        if state_id == 4946 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 14,
                r#north: North::Side,
                r#east: East::None,
                r#south: South::Up,
            });
        }
        if state_id == 4049 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::None,
                r#north: North::Side,
                r#power: 10,
                r#east: East::Up,
            });
        }
        if state_id == 4949 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 14,
                r#south: South::Side,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 5000 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 4,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4229 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::None,
                r#power: 14,
                r#south: South::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4376 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::None,
                r#power: 14,
                r#north: North::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4449 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 7,
                r#south: South::Up,
                r#east: East::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4006 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::None,
                r#north: North::Side,
                r#west: West::Side,
                r#power: 5,
            });
        }
        if state_id == 4193 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Up,
                r#power: 10,
            });
        }
        if state_id == 4217 {
            return Some(RedstoneWire {
                r#power: 13,
                r#west: West::None,
                r#east: East::Up,
                r#south: South::Up,
                r#north: North::None,
            });
        }
        if state_id == 4550 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::None,
                r#power: 2,
            });
        }
        if state_id == 4608 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#power: 8,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4071 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::Up,
                r#power: 13,
            });
        }
        if state_id == 3985 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Up,
                r#power: 3,
                r#west: West::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4787 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::None,
                r#east: East::None,
                r#power: 12,
            });
        }
        if state_id == 4137 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Up,
                r#power: 4,
                r#east: East::Up,
            });
        }
        if state_id == 4916 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#power: 10,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 4940 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 13,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4794 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#power: 13,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 3961 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::None,
                r#power: 0,
                r#west: West::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4985 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 2,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 4201 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#power: 11,
                r#north: North::None,
                r#south: South::Side,
            });
        }
        if state_id == 4468 {
            return Some(RedstoneWire {
                r#power: 9,
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4684 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Side,
                r#north: North::Up,
                r#south: South::Up,
                r#power: 1,
            });
        }
        if state_id == 4098 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 0,
                r#north: North::None,
                r#south: South::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4270 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Up,
                r#east: East::Side,
                r#west: West::Side,
                r#power: 3,
            });
        }
        if state_id == 4513 {
            return Some(RedstoneWire {
                r#power: 14,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4512 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#power: 14,
                r#south: South::Up,
                r#north: North::Side,
            });
        }
        if state_id == 3898 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Side,
                r#power: 9,
                r#south: South::None,
                r#east: East::Up,
            });
        }
        if state_id == 4902 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 9,
                r#west: West::Up,
                r#east: East::None,
                r#north: North::Side,
            });
        }
        if state_id == 4978 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 1,
                r#west: West::Side,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 4235 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::None,
                r#east: East::Up,
                r#power: 15,
                r#west: West::None,
            });
        }
        if state_id == 4052 {
            return Some(RedstoneWire {
                r#power: 10,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4502 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Side,
                r#power: 12,
                r#north: North::Side,
                r#west: West::None,
            });
        }
        if state_id == 3909 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 11,
                r#south: South::Up,
                r#east: East::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4497 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#south: South::Side,
                r#power: 12,
                r#west: West::Up,
            });
        }
        if state_id == 4602 {
            return Some(RedstoneWire {
                r#power: 8,
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::None,
                r#west: West::Up,
            });
        }
        if state_id == 4697 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Up,
                r#power: 2,
                r#east: East::None,
                r#south: South::Side,
            });
        }
        if state_id == 4257 {
            return Some(RedstoneWire {
                r#power: 1,
                r#south: South::None,
                r#east: East::Side,
                r#north: North::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4907 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#east: East::None,
                r#power: 9,
                r#west: West::None,
            });
        }
        if state_id == 5075 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 12,
                r#west: West::None,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 4282 {
            return Some(RedstoneWire {
                r#power: 4,
                r#west: West::Side,
                r#north: North::Up,
                r#south: South::Side,
                r#east: East::Side,
            });
        }
        if state_id == 3936 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::Up,
                r#east: East::Up,
                r#north: North::Up,
                r#power: 14,
            });
        }
        if state_id == 4962 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#power: 0,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4687 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 1,
                r#west: West::Side,
                r#south: South::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4300 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::Side,
                r#power: 6,
            });
        }
        if state_id == 4371 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Up,
                r#power: 14,
            });
        }
        if state_id == 4579 {
            return Some(RedstoneWire {
                r#power: 5,
                r#east: East::Side,
                r#south: South::Side,
                r#north: North::None,
                r#west: West::Side,
            });
        }
        if state_id == 5015 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
                r#power: 5,
            });
        }
        if state_id == 4478 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Side,
                r#power: 10,
                r#north: North::Side,
                r#west: West::None,
            });
        }
        if state_id == 4141 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 4,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Up,
            });
        }
        if state_id == 4636 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#power: 11,
                r#east: East::Side,
                r#north: North::None,
            });
        }
        if state_id == 4383 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::None,
                r#west: West::Up,
                r#power: 15,
            });
        }
        if state_id == 4479 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 10,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4530 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 0,
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::None,
            });
        }
        if state_id == 4802 {
            return Some(RedstoneWire {
                r#power: 14,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4302 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#north: North::Up,
                r#power: 6,
                r#south: South::None,
            });
        }
        if state_id == 4991 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 3,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 4087 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 14,
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4102 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 0,
                r#south: South::Side,
                r#east: East::Up,
                r#north: North::None,
            });
        }
        if state_id == 4887 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Up,
                r#east: East::None,
                r#south: South::None,
                r#power: 7,
            });
        }
        if state_id == 4796 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 13,
                r#south: South::Side,
                r#west: West::None,
                r#north: North::Up,
            });
        }
        if state_id == 4566 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::Up,
                r#north: North::None,
                r#east: East::Side,
                r#power: 4,
            });
        }
        if state_id == 4215 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 13,
                r#east: East::Up,
                r#north: North::None,
                r#west: West::Up,
            });
        }
        if state_id == 4415 {
            return Some(RedstoneWire {
                r#power: 3,
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::None,
            });
        }
        if state_id == 4822 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Side,
                r#power: 0,
                r#east: East::None,
                r#south: South::Side,
            });
        }
        if state_id == 4025 {
            return Some(RedstoneWire {
                r#power: 7,
                r#east: East::Up,
                r#south: South::None,
                r#north: North::Side,
                r#west: West::None,
            });
        }
        if state_id == 3867 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 6,
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4196 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#east: East::Up,
                r#north: North::None,
                r#power: 10,
            });
        }
        if state_id == 3974 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::None,
                r#power: 2,
                r#north: North::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4149 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Up,
                r#power: 5,
            });
        }
        if state_id == 4573 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::None,
                r#west: West::Side,
                r#east: East::Side,
                r#power: 4,
            });
        }
        if state_id == 3940 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 14,
                r#east: East::Up,
            });
        }
        if state_id == 4241 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 15,
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 4761 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#north: North::Up,
                r#east: East::None,
                r#power: 9,
            });
        }
        if state_id == 4818 {
            return Some(RedstoneWire {
                r#power: 0,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::None,
            });
        }
        if state_id == 5016 {
            return Some(RedstoneWire {
                r#power: 6,
                r#east: East::None,
                r#south: South::Up,
                r#north: North::None,
                r#west: West::Up,
            });
        }
        if state_id == 4945 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::None,
                r#power: 14,
                r#west: West::Side,
            });
        }
        if state_id == 4676 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::None,
                r#power: 0,
                r#east: East::None,
            });
        }
        if state_id == 4865 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Side,
                r#power: 5,
                r#south: South::Up,
                r#east: East::None,
            });
        }
        if state_id == 4064 {
            return Some(RedstoneWire {
                r#power: 12,
                r#north: North::Side,
                r#east: East::Up,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4109 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#power: 1,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4499 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#power: 12,
                r#south: South::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4670 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 15,
                r#south: South::Side,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 4320 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Up,
                r#east: East::Side,
                r#power: 8,
                r#west: West::Up,
            });
        }
        if state_id == 4535 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#west: West::None,
                r#south: South::Side,
                r#power: 0,
            });
        }
        if state_id == 4750 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Up,
                r#power: 8,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4251 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 1,
                r#west: West::Up,
                r#east: East::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4442 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::None,
                r#north: North::Side,
                r#power: 6,
            });
        }
        if state_id == 4692 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 2,
                r#south: South::Up,
                r#east: East::None,
                r#north: North::Up,
            });
        }
        if state_id == 4992 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#west: West::Up,
                r#power: 3,
                r#south: South::Side,
            });
        }
        if state_id == 5009 {
            return Some(RedstoneWire {
                r#power: 5,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Up,
                r#north: North::None,
            });
        }
        if state_id == 4293 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Up,
                r#power: 5,
                r#east: East::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4618 {
            return Some(RedstoneWire {
                r#power: 9,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4669 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#power: 15,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 4024 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Side,
                r#north: North::Side,
                r#power: 7,
                r#south: South::None,
            });
        }
        if state_id == 3818 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 0,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 4357 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Side,
                r#power: 12,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4380 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Up,
                r#power: 15,
                r#east: East::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4588 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 6,
                r#south: South::Side,
                r#north: North::None,
                r#west: West::Side,
            });
        }
        if state_id == 4189 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Up,
                r#east: East::Up,
                r#power: 10,
                r#west: West::Side,
            });
        }
        if state_id == 4291 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 5,
                r#east: East::Side,
            });
        }
        if state_id == 4451 {
            return Some(RedstoneWire {
                r#power: 7,
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::None,
                r#east: East::Side,
            });
        }
        if state_id == 4840 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 2,
            });
        }
        if state_id == 4286 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Up,
                r#south: South::None,
                r#power: 4,
                r#east: East::Side,
            });
        }
        if state_id == 5105 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#power: 15,
                r#west: West::None,
            });
        }
        if state_id == 4039 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#east: East::Up,
                r#power: 9,
                r#west: West::Side,
            });
        }
        if state_id == 4017 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 7,
            });
        }
        if state_id == 4494 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::Side,
                r#power: 12,
            });
        }
        if state_id == 4164 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#west: West::Up,
                r#south: South::Side,
                r#power: 7,
            });
        }
        if state_id == 4305 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Up,
                r#power: 7,
                r#east: East::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4366 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Side,
                r#power: 13,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 4198 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::None,
                r#power: 11,
                r#west: West::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4537 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Side,
                r#south: South::None,
                r#north: North::None,
                r#power: 0,
            });
        }
        if state_id == 4539 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Up,
                r#power: 1,
                r#west: West::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4606 {
            return Some(RedstoneWire {
                r#power: 8,
                r#east: East::Side,
                r#north: North::None,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4655 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::None,
                r#power: 13,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 4645 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#power: 12,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 3941 {
            return Some(RedstoneWire {
                r#power: 14,
                r#west: West::None,
                r#east: East::Up,
                r#south: South::Side,
                r#north: North::Up,
            });
        }
        if state_id == 3926 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#power: 12,
                r#north: North::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4453 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#west: West::Side,
                r#south: South::Side,
                r#power: 7,
            });
        }
        if state_id == 4851 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#west: West::Up,
                r#power: 3,
                r#east: East::None,
            });
        }
        if state_id == 5059 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::None,
                r#power: 10,
                r#west: West::Side,
                r#east: East::None,
            });
        }
        if state_id == 3832 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 2,
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::Up,
            });
        }
        if state_id == 3971 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Up,
                r#power: 1,
                r#north: North::Side,
            });
        }
        if state_id == 3884 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 8,
                r#east: East::Up,
                r#south: South::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4011 {
            return Some(RedstoneWire {
                r#power: 6,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4306 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Side,
                r#power: 7,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4454 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Side,
                r#south: South::Side,
                r#power: 7,
                r#east: East::Side,
            });
        }
        if state_id == 5066 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::None,
                r#east: East::None,
                r#power: 11,
                r#south: South::Side,
            });
        }
        if state_id == 4066 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#power: 12,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4223 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::None,
                r#east: East::Up,
                r#west: West::None,
                r#power: 13,
            });
        }
        if state_id == 4411 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 2,
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4627 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Side,
                r#power: 10,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 4878 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#power: 6,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 3927 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 13,
            });
        }
        if state_id == 4285 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 4,
                r#west: West::Side,
                r#south: South::None,
                r#north: North::Up,
            });
        }
        if state_id == 4355 {
            return Some(RedstoneWire {
                r#power: 12,
                r#north: North::Up,
                r#east: East::Side,
                r#west: West::None,
                r#south: South::Side,
            });
        }
        if state_id == 4188 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::None,
                r#power: 10,
                r#south: South::Up,
                r#east: East::Up,
            });
        }
        if state_id == 3969 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 1,
                r#south: South::None,
            });
        }
        if state_id == 4438 {
            return Some(RedstoneWire {
                r#power: 5,
                r#east: East::Side,
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4821 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 0,
                r#north: North::Side,
                r#south: South::Side,
                r#east: East::None,
            });
        }
        if state_id == 4308 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#power: 7,
                r#north: North::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4362 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Side,
                r#power: 13,
                r#west: West::Up,
                r#east: East::Side,
            });
        }
        if state_id == 5026 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::Up,
                r#north: North::None,
                r#east: East::None,
                r#power: 7,
            });
        }
        if state_id == 4392 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 0,
                r#south: South::None,
                r#west: West::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4457 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 7,
            });
        }
        if state_id == 4689 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Up,
                r#power: 1,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 4459 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Side,
                r#east: East::Side,
                r#power: 8,
                r#south: South::Up,
            });
        }
        if state_id == 4557 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 3,
                r#north: North::None,
            });
        }
        if state_id == 4641 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Up,
                r#east: East::Side,
                r#power: 12,
            });
        }
        if state_id == 4735 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 6,
                r#east: East::None,
            });
        }
        if state_id == 4583 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::None,
                r#east: East::Side,
                r#power: 5,
                r#west: West::None,
            });
        }
        if state_id == 4342 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 11,
                r#west: West::Side,
                r#east: East::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4592 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Side,
                r#power: 6,
            });
        }
        if state_id == 4864 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#west: West::Side,
                r#south: South::Up,
                r#power: 5,
            });
        }
        if state_id == 4941 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#west: West::Up,
                r#power: 13,
                r#east: East::None,
            });
        }
        if state_id == 5043 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Up,
                r#east: East::None,
                r#south: South::Up,
                r#power: 9,
            });
        }
        if state_id == 4341 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 11,
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4662 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#power: 14,
                r#east: East::Side,
                r#north: North::None,
            });
        }
        if state_id == 4483 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 10,
            });
        }
        if state_id == 4023 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 7,
                r#west: West::Up,
            });
        }
        if state_id == 4631 {
            return Some(RedstoneWire {
                r#power: 11,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4005 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Up,
                r#power: 5,
                r#north: North::Side,
                r#south: South::None,
            });
        }
        if state_id == 4646 {
            return Some(RedstoneWire {
                r#power: 12,
                r#east: East::Side,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 5076 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::None,
                r#power: 12,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 3942 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Up,
                r#power: 14,
                r#south: South::None,
                r#east: East::Up,
            });
        }
        if state_id == 4324 {
            return Some(RedstoneWire {
                r#power: 9,
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4053 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Up,
                r#power: 11,
                r#north: North::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4578 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::Up,
                r#power: 5,
            });
        }
        if state_id == 4770 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Up,
                r#west: West::Up,
                r#power: 10,
                r#south: South::None,
            });
        }
        if state_id == 5070 {
            return Some(RedstoneWire {
                r#power: 12,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Up,
                r#south: South::Up,
            });
        }
        if state_id == 3962 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#north: North::Side,
                r#east: East::Up,
                r#power: 0,
            });
        }
        if state_id == 5080 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Up,
                r#power: 13,
            });
        }
        if state_id == 4245 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 0,
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4951 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#south: South::None,
                r#power: 14,
                r#west: West::Side,
            });
        }
        if state_id == 4603 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Side,
                r#power: 8,
                r#west: West::Side,
                r#north: North::None,
            });
        }
        if state_id == 4527 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#power: 15,
                r#north: North::Side,
                r#south: South::None,
            });
        }
        if state_id == 4474 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#power: 9,
                r#west: West::Side,
                r#south: South::None,
            });
        }
        if state_id == 4074 {
            return Some(RedstoneWire {
                r#power: 13,
                r#west: West::Up,
                r#east: East::Up,
                r#south: South::Side,
                r#north: North::Side,
            });
        }
        if state_id == 3935 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#power: 13,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4776 {
            return Some(RedstoneWire {
                r#power: 11,
                r#south: South::Side,
                r#east: East::None,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4331 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::None,
                r#power: 9,
                r#east: East::Side,
                r#south: South::None,
            });
        }
        if state_id == 4246 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::Side,
                r#power: 0,
                r#south: South::Side,
            });
        }
        if state_id == 4443 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 6,
                r#west: West::Up,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4765 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 10,
                r#east: East::None,
                r#north: North::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4425 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::Side,
                r#power: 4,
            });
        }
        if state_id == 4895 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#west: West::None,
                r#power: 8,
                r#south: South::Side,
            });
        }
        if state_id == 3824 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Up,
                r#power: 1,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 4359 {
            return Some(RedstoneWire {
                r#power: 13,
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::Up,
                r#east: East::Side,
            });
        }
        if state_id == 5067 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Up,
                r#power: 11,
            });
        }
        if state_id == 4674 {
            return Some(RedstoneWire {
                r#power: 0,
                r#west: West::Up,
                r#south: South::Up,
                r#north: North::Up,
                r#east: East::None,
            });
        }
        if state_id == 4522 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Side,
                r#power: 15,
            });
        }
        if state_id == 4909 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 10,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4696 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 2,
            });
        }
        if state_id == 4114 {
            return Some(RedstoneWire {
                r#power: 1,
                r#north: North::None,
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4809 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 15,
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 3913 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#power: 11,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 3879 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::None,
                r#power: 7,
                r#west: West::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4020 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Up,
                r#power: 7,
                r#east: East::Up,
            });
        }
        if state_id == 5050 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#west: West::Side,
                r#power: 9,
                r#south: South::None,
            });
        }
        if state_id == 3857 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#power: 5,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4009 {
            return Some(RedstoneWire {
                r#power: 6,
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::Side,
                r#east: East::Up,
            });
        }
        if state_id == 3875 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 7,
                r#west: West::None,
                r#north: North::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4301 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 6,
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4564 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::None,
                r#north: North::None,
                r#power: 3,
                r#west: West::Side,
            });
        }
        if state_id == 3931 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Side,
                r#power: 13,
                r#west: West::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4117 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Up,
                r#power: 2,
                r#east: East::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4680 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::None,
                r#south: South::None,
                r#power: 0,
                r#west: West::Up,
            });
        }
        if state_id == 4508 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::None,
                r#south: South::Side,
                r#east: East::Side,
                r#power: 13,
            });
        }
        if state_id == 4519 {
            return Some(RedstoneWire {
                r#power: 14,
                r#west: West::Side,
                r#south: South::None,
                r#north: North::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4604 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::None,
                r#power: 8,
                r#south: South::Up,
                r#north: North::None,
            });
        }
        if state_id == 4722 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#east: East::None,
                r#north: North::Up,
                r#power: 5,
            });
        }
        if state_id == 4340 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 10,
                r#north: North::Up,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4345 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#power: 11,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4477 {
            return Some(RedstoneWire {
                r#power: 10,
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4968 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#north: North::None,
                r#power: 0,
                r#east: East::None,
            });
        }
        if state_id == 4714 {
            return Some(RedstoneWire {
                r#power: 4,
                r#west: West::Side,
                r#east: East::None,
                r#south: South::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4222 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::None,
                r#east: East::Up,
                r#power: 13,
                r#north: North::None,
            });
        }
        if state_id == 3821 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Up,
                r#power: 1,
                r#east: East::Up,
                r#south: South::Up,
            });
        }
        if state_id == 5010 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::None,
                r#power: 5,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 5024 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::None,
                r#power: 6,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 4133 {
            return Some(RedstoneWire {
                r#power: 3,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Up,
                r#south: South::None,
            });
        }
        if state_id == 4050 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Up,
                r#west: West::Up,
                r#south: South::None,
                r#power: 10,
            });
        }
        if state_id == 4016 {
            return Some(RedstoneWire {
                r#power: 6,
                r#south: South::None,
                r#east: East::Up,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 4143 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 5,
                r#north: North::None,
                r#east: East::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4955 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::None,
                r#east: East::None,
                r#power: 15,
            });
        }
        if state_id == 4490 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 11,
                r#west: West::None,
            });
        }
        if state_id == 4542 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 1,
                r#west: West::Up,
                r#north: North::None,
                r#south: South::Side,
            });
        }
        if state_id == 4952 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 14,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 4167 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Up,
                r#power: 7,
                r#east: East::Up,
                r#south: South::None,
            });
        }
        if state_id == 3839 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::None,
                r#east: East::Up,
                r#power: 3,
                r#south: South::Up,
            });
        }
        if state_id == 3989 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Up,
                r#power: 3,
                r#north: North::Side,
                r#west: West::None,
            });
        }
        if state_id == 4230 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#south: South::None,
                r#power: 14,
                r#west: West::Up,
            });
        }
        if state_id == 3827 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Up,
                r#west: West::None,
                r#east: East::Up,
                r#power: 1,
            });
        }
        if state_id == 4784 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::None,
                r#power: 12,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4404 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Side,
                r#power: 2,
                r#north: North::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4529 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::None,
                r#power: 15,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4673 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::None,
                r#east: East::Side,
                r#power: 15,
                r#west: West::None,
            });
        }
        if state_id == 4899 {
            return Some(RedstoneWire {
                r#power: 9,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::None,
                r#north: North::Side,
            });
        }
        if state_id == 4208 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::None,
                r#north: North::None,
                r#power: 12,
                r#south: South::Up,
            });
        }
        if state_id == 4758 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Up,
                r#north: North::Up,
                r#south: South::Side,
                r#power: 9,
            });
        }
        if state_id == 4385 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#north: North::Up,
                r#power: 15,
                r#east: East::Side,
            });
        }
        if state_id == 4234 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::Up,
                r#power: 15,
                r#east: East::Up,
                r#north: North::None,
            });
        }
        if state_id == 3837 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 3,
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4937 {
            return Some(RedstoneWire {
                r#power: 13,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4046 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::None,
                r#power: 10,
            });
        }
        if state_id == 4216 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Up,
                r#west: West::Side,
                r#power: 13,
                r#east: East::Up,
            });
        }
        if state_id == 4232 {
            return Some(RedstoneWire {
                r#power: 14,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Up,
                r#south: South::None,
            });
        }
        if state_id == 4273 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#north: North::Up,
                r#power: 3,
                r#west: West::Side,
            });
        }
        if state_id == 5057 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#power: 10,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 4152 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#power: 6,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4140 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#power: 4,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4003 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::Up,
                r#power: 5,
            });
        }
        if state_id == 4466 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::None,
                r#power: 8,
                r#east: East::Side,
                r#south: South::None,
            });
        }
        if state_id == 3869 {
            return Some(RedstoneWire {
                r#power: 6,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 4445 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 6,
                r#south: South::Side,
            });
        }
        if state_id == 4563 {
            return Some(RedstoneWire {
                r#power: 3,
                r#south: South::None,
                r#west: West::Up,
                r#north: North::None,
                r#east: East::Side,
            });
        }
        if state_id == 4280 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 4,
                r#north: North::Up,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4890 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 8,
                r#east: East::None,
            });
        }
        if state_id == 4159 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#power: 6,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 5017 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::None,
                r#south: South::Up,
                r#east: East::None,
                r#power: 6,
            });
        }
        if state_id == 4212 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Up,
                r#west: West::Up,
                r#north: North::None,
                r#power: 12,
            });
        }
        if state_id == 4172 {
            return Some(RedstoneWire {
                r#power: 8,
                r#north: North::None,
                r#south: South::Up,
                r#east: East::Up,
                r#west: West::None,
            });
        }
        if state_id == 4386 {
            return Some(RedstoneWire {
                r#power: 0,
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4552 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Side,
                r#south: South::Side,
                r#power: 2,
                r#east: East::Side,
            });
        }
        if state_id == 4594 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 7,
                r#west: West::Side,
                r#north: North::None,
                r#south: South::Up,
            });
        }
        if state_id == 4789 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Up,
                r#power: 12,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4817 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 15,
                r#south: South::None,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 4547 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
                r#power: 1,
            });
        }
        if state_id == 3952 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Side,
                r#power: 15,
                r#south: South::None,
                r#east: East::Up,
            });
        }
        if state_id == 4565 {
            return Some(RedstoneWire {
                r#power: 3,
                r#west: West::None,
                r#east: East::Side,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 3835 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#south: South::None,
                r#power: 2,
                r#north: North::Up,
            });
        }
        if state_id == 3892 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#power: 9,
                r#east: East::Up,
                r#north: North::Up,
            });
        }
        if state_id == 3859 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 5,
                r#south: South::Side,
                r#east: East::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4740 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Up,
                r#south: South::Side,
                r#east: East::None,
                r#power: 7,
            });
        }
        if state_id == 4975 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::None,
                r#power: 1,
            });
        }
        if state_id == 4612 {
            return Some(RedstoneWire {
                r#power: 9,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4146 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Up,
                r#north: North::None,
                r#power: 5,
                r#west: West::Up,
            });
        }
        if state_id == 5061 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#power: 11,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4183 {
            return Some(RedstoneWire {
                r#power: 9,
                r#west: West::Side,
                r#north: North::None,
                r#south: South::Side,
                r#east: East::Up,
            });
        }
        if state_id == 3883 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Up,
                r#power: 8,
                r#west: West::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4244 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Up,
                r#power: 0,
                r#south: South::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4705 {
            return Some(RedstoneWire {
                r#power: 3,
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::None,
                r#south: South::Side,
            });
        }
        if state_id == 4874 {
            return Some(RedstoneWire {
                r#power: 6,
                r#east: East::None,
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::None,
            });
        }
        if state_id == 3825 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::None,
                r#power: 1,
                r#west: West::Up,
            });
        }
        if state_id == 4079 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 13,
                r#west: West::None,
            });
        }
        if state_id == 3907 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 10,
                r#north: North::Up,
                r#south: South::None,
                r#east: East::Up,
            });
        }
        if state_id == 4130 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::None,
                r#east: East::Up,
                r#power: 3,
                r#west: West::None,
            });
        }
        if state_id == 4274 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 3,
                r#east: East::Side,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 4344 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#power: 11,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4337 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#west: West::None,
                r#power: 10,
                r#north: North::Up,
            });
        }
        if state_id == 4475 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#power: 9,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 4485 {
            return Some(RedstoneWire {
                r#power: 11,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4090 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#power: 15,
                r#west: West::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4617 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::None,
                r#power: 9,
                r#east: East::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4699 {
            return Some(RedstoneWire {
                r#power: 2,
                r#north: North::Up,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4335 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#north: North::Up,
                r#power: 10,
                r#west: West::Up,
            });
        }
        if state_id == 4709 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Up,
                r#power: 3,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 3819 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 1,
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4585 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::None,
                r#west: West::Side,
                r#power: 6,
            });
        }
        if state_id == 4664 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#power: 14,
                r#east: East::Side,
            });
        }
        if state_id == 4838 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 2,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4326 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::Side,
                r#north: North::Up,
                r#east: East::Side,
                r#power: 9,
            });
        }
        if state_id == 4615 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Side,
                r#north: North::None,
                r#power: 9,
                r#west: West::Side,
            });
        }
        if state_id == 4682 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
                r#power: 0,
            });
        }
        if state_id == 5048 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::None,
                r#power: 9,
            });
        }
        if state_id == 5092 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#north: North::None,
                r#power: 14,
                r#west: West::Side,
            });
        }
        if state_id == 3865 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Side,
                r#power: 6,
                r#north: North::Up,
            });
        }
        if state_id == 4284 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Up,
                r#south: South::None,
                r#east: East::Side,
                r#power: 4,
            });
        }
        if state_id == 4721 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::None,
                r#north: North::Up,
                r#power: 5,
                r#south: South::Up,
            });
        }
        if state_id == 4989 {
            return Some(RedstoneWire {
                r#power: 3,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::None,
            });
        }
        if state_id == 3919 {
            return Some(RedstoneWire {
                r#power: 12,
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4170 {
            return Some(RedstoneWire {
                r#power: 8,
                r#south: South::Up,
                r#north: North::None,
                r#west: West::Up,
                r#east: East::Up,
            });
        }
        if state_id == 5099 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 15,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 4028 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 8,
                r#south: South::Up,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 5056 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Side,
                r#north: North::None,
                r#power: 10,
                r#south: South::Side,
            });
        }
        if state_id == 3915 {
            return Some(RedstoneWire {
                r#power: 11,
                r#south: South::None,
                r#north: North::Up,
                r#west: West::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4872 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 6,
                r#south: South::Up,
                r#north: North::Side,
                r#east: East::None,
            });
        }
        if state_id == 4653 {
            return Some(RedstoneWire {
                r#power: 13,
                r#north: North::None,
                r#east: East::Side,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 3951 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Up,
                r#north: North::Up,
                r#south: South::None,
                r#power: 15,
            });
        }
        if state_id == 4939 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::None,
                r#north: North::Side,
                r#power: 13,
                r#south: South::Side,
            });
        }
        if state_id == 5040 {
            return Some(RedstoneWire {
                r#power: 8,
                r#west: West::Up,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 4310 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Up,
                r#power: 7,
                r#south: South::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4727 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Up,
                r#power: 5,
            });
        }
        if state_id == 4903 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#south: South::Side,
                r#power: 9,
                r#west: West::Side,
            });
        }
        if state_id == 4322 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 8,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Side,
            });
        }
        if state_id == 4775 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 11,
                r#north: North::Up,
                r#east: East::None,
                r#south: South::Up,
            });
        }
        if state_id == 4708 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#power: 3,
                r#east: East::None,
                r#north: North::Up,
            });
        }
        if state_id == 4156 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 6,
            });
        }
        if state_id == 4780 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::None,
                r#north: North::Up,
                r#power: 11,
                r#south: South::None,
            });
        }
        if state_id == 4343 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::Up,
                r#west: West::None,
                r#power: 11,
            });
        }
        if state_id == 4967 {
            return Some(RedstoneWire {
                r#power: 0,
                r#south: South::Side,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 3933 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Up,
                r#power: 13,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 4514 {
            return Some(RedstoneWire {
                r#power: 14,
                r#east: East::Side,
                r#west: West::None,
                r#south: South::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4374 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Side,
                r#south: South::None,
                r#power: 14,
                r#west: West::Up,
            });
        }
        if state_id == 4450 {
            return Some(RedstoneWire {
                r#power: 7,
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4772 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 10,
                r#east: East::None,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 4067 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 12,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4973 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Up,
                r#power: 1,
            });
        }
        if state_id == 4637 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#west: West::None,
                r#power: 11,
                r#south: South::None,
            });
        }
        if state_id == 4811 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Up,
                r#power: 15,
                r#south: South::Up,
                r#east: East::None,
            });
        }
        if state_id == 4440 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 6,
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 3838 {
            return Some(RedstoneWire {
                r#power: 3,
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::Up,
            });
        }
        if state_id == 3910 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 11,
                r#west: West::Side,
                r#north: North::Up,
                r#south: South::Up,
            });
        }
        if state_id == 3922 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 12,
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4057 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::Side,
                r#power: 11,
                r#north: North::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4179 {
            return Some(RedstoneWire {
                r#power: 9,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::None,
            });
        }
        if state_id == 4843 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#west: West::Side,
                r#south: South::None,
                r#power: 2,
            });
        }
        if state_id == 4022 {
            return Some(RedstoneWire {
                r#power: 7,
                r#west: West::None,
                r#south: South::Side,
                r#east: East::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4752 {
            return Some(RedstoneWire {
                r#power: 8,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 5006 {
            return Some(RedstoneWire {
                r#power: 4,
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 4276 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Side,
                r#west: West::Side,
                r#power: 3,
                r#south: South::None,
            });
        }
        if state_id == 4546 {
            return Some(RedstoneWire {
                r#power: 1,
                r#east: East::Side,
                r#west: West::Side,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 4792 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::None,
                r#north: North::Up,
                r#power: 13,
                r#south: South::Up,
            });
        }
        if state_id == 4832 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Side,
                r#power: 1,
                r#south: South::Side,
            });
        }
        if state_id == 4672 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Side,
                r#power: 15,
                r#north: North::None,
                r#west: West::Side,
            });
        }
        if state_id == 4690 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::None,
                r#power: 1,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4923 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::None,
                r#north: North::Side,
                r#power: 11,
                r#west: West::Up,
            });
        }
        if state_id == 4243 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::Up,
                r#north: North::Up,
                r#east: East::Side,
                r#power: 0,
            });
        }
        if state_id == 4964 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::None,
                r#north: North::None,
                r#power: 0,
                r#east: East::None,
            });
        }
        if state_id == 3899 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#north: North::Up,
                r#power: 9,
                r#south: South::None,
            });
        }
        if state_id == 5053 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 10,
                r#south: South::Up,
                r#east: East::None,
                r#west: West::Side,
            });
        }
        if state_id == 3960 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#west: West::Up,
                r#power: 0,
                r#south: South::None,
            });
        }
        if state_id == 4410 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 2,
                r#west: West::Up,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4624 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#power: 10,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4139 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#east: East::Up,
                r#power: 4,
                r#west: West::None,
            });
        }
        if state_id == 4008 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#power: 6,
                r#south: South::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4261 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#power: 2,
                r#west: West::Side,
                r#south: South::Up,
            });
        }
        if state_id == 5012 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 5,
                r#south: South::Side,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 3874 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#south: South::Up,
                r#north: North::Up,
                r#power: 7,
            });
        }
        if state_id == 3965 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 1,
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::Up,
            });
        }
        if state_id == 3966 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#power: 1,
                r#east: East::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4668 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Up,
                r#power: 15,
                r#north: North::None,
                r#south: South::Side,
            });
        }
        if state_id == 3953 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 15,
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::None,
            });
        }
        if state_id == 4600 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::None,
                r#power: 7,
            });
        }
        if state_id == 4110 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Up,
                r#power: 1,
            });
        }
        if state_id == 4470 {
            return Some(RedstoneWire {
                r#power: 9,
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4651 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#power: 13,
                r#west: West::Side,
                r#north: North::None,
            });
        }
        if state_id == 4719 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 5,
                r#north: North::Up,
            });
        }
        if state_id == 3894 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Up,
                r#power: 9,
                r#south: South::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4785 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::None,
                r#north: North::Up,
                r#power: 12,
                r#south: South::Side,
            });
        }
        if state_id == 4484 {
            return Some(RedstoneWire {
                r#power: 10,
                r#north: North::Side,
                r#east: East::Side,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4685 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Up,
                r#west: West::None,
                r#east: East::None,
                r#power: 1,
            });
        }
        if state_id == 4200 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 11,
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::None,
            });
        }
        if state_id == 4044 {
            return Some(RedstoneWire {
                r#power: 10,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4423 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 4,
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::Side,
            });
        }
        if state_id == 3852 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::None,
                r#east: East::Up,
                r#west: West::Up,
                r#power: 4,
            });
        }
        if state_id == 4657 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 14,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 3877 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::Up,
                r#power: 7,
                r#south: South::Side,
            });
        }
        if state_id == 4742 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 7,
                r#south: South::Side,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 3979 {
            return Some(RedstoneWire {
                r#power: 2,
                r#south: South::None,
                r#north: North::Side,
                r#east: East::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4825 {
            return Some(RedstoneWire {
                r#power: 0,
                r#west: West::Side,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Side,
            });
        }
        if state_id == 4330 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::None,
                r#east: East::Side,
                r#west: West::Side,
                r#power: 9,
            });
        }
        if state_id == 4844 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 2,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Side,
            });
        }
        if state_id == 4082 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 14,
                r#east: East::Up,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4092 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#power: 15,
                r#south: South::Side,
                r#north: North::Side,
            });
        }
        if state_id == 5020 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Side,
                r#power: 6,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 4656 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#power: 14,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4769 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::None,
                r#east: East::None,
                r#power: 10,
                r#south: South::Side,
            });
        }
        if state_id == 3850 {
            return Some(RedstoneWire {
                r#power: 4,
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4163 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 7,
                r#east: East::Up,
                r#south: South::Up,
                r#north: North::None,
            });
        }
        if state_id == 3843 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#power: 3,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4816 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::None,
                r#power: 15,
                r#west: West::Side,
                r#east: East::None,
            });
        }
        if state_id == 4043 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::None,
                r#power: 9,
            });
        }
        if state_id == 4372 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Side,
                r#power: 14,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4127 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 3,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4596 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 7,
                r#north: North::None,
                r#west: West::Up,
                r#east: East::Side,
            });
        }
        if state_id == 3920 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::None,
                r#power: 12,
                r#east: East::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4420 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#west: West::Side,
                r#power: 3,
                r#south: South::None,
            });
        }
        if state_id == 4665 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::None,
                r#power: 15,
            });
        }
        if state_id == 4849 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Side,
                r#power: 3,
                r#east: East::None,
                r#south: South::Side,
            });
        }
        if state_id == 3906 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::Up,
                r#south: South::None,
                r#power: 10,
            });
        }
        if state_id == 4384 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::Side,
                r#power: 15,
            });
        }
        if state_id == 4983 {
            return Some(RedstoneWire {
                r#power: 2,
                r#west: West::Up,
                r#north: North::None,
                r#south: South::Side,
                r#east: East::None,
            });
        }
        if state_id == 4138 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::None,
                r#south: South::Side,
                r#power: 4,
            });
        }
        if state_id == 4813 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Up,
                r#power: 15,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4694 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 2,
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4242 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 0,
                r#north: North::Up,
            });
        }
        if state_id == 3829 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Up,
                r#north: North::Up,
                r#power: 2,
                r#west: West::Side,
            });
        }
        if state_id == 4363 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#west: West::Side,
                r#power: 13,
                r#north: North::Up,
            });
        }
        if state_id == 4303 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#power: 6,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 4526 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Side,
                r#west: West::None,
                r#east: East::Side,
                r#power: 15,
            });
        }
        if state_id == 4643 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 12,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Side,
            });
        }
        if state_id == 4824 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::None,
                r#north: North::Side,
                r#power: 0,
                r#west: West::Up,
            });
        }
        if state_id == 4828 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Side,
                r#power: 1,
                r#east: East::None,
                r#south: South::Up,
            });
        }
        if state_id == 5032 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#power: 7,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 4910 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#south: South::Up,
                r#power: 10,
                r#west: West::None,
            });
        }
        if state_id == 4093 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#south: South::Side,
                r#north: North::Side,
                r#power: 15,
            });
        }
        if state_id == 4538 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
                r#power: 0,
                r#east: East::Side,
            });
        }
        if state_id == 4569 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#power: 4,
                r#east: East::Side,
                r#west: West::Up,
            });
        }
        if state_id == 5060 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::None,
                r#power: 10,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 4077 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 13,
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4938 {
            return Some(RedstoneWire {
                r#power: 13,
                r#north: North::Side,
                r#south: South::Side,
                r#east: East::None,
                r#west: West::Up,
            });
        }
        if state_id == 4382 {
            return Some(RedstoneWire {
                r#power: 15,
                r#west: West::None,
                r#north: North::Up,
                r#south: South::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4561 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 3,
                r#west: West::Side,
                r#east: East::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4399 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#power: 1,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 3996 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Up,
                r#west: West::Up,
                r#north: North::Side,
                r#power: 4,
            });
        }
        if state_id == 4178 {
            return Some(RedstoneWire {
                r#power: 8,
                r#east: East::Up,
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 4173 {
            return Some(RedstoneWire {
                r#power: 8,
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::None,
                r#east: East::Up,
            });
        }
        if state_id == 4134 {
            return Some(RedstoneWire {
                r#power: 4,
                r#east: East::Up,
                r#south: South::Up,
                r#north: North::None,
                r#west: West::Up,
            });
        }
        if state_id == 4598 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::None,
                r#power: 7,
            });
        }
        if state_id == 5018 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#power: 6,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4203 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::None,
                r#power: 11,
                r#south: South::None,
            });
        }
        if state_id == 4634 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#power: 11,
                r#west: West::None,
                r#south: South::Side,
            });
        }
        if state_id == 4177 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#power: 8,
                r#north: North::None,
                r#east: East::Up,
            });
        }
        if state_id == 4346 {
            return Some(RedstoneWire {
                r#power: 11,
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 5021 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::None,
                r#power: 6,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 4893 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 8,
                r#west: West::Up,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        return None;
    }
}
