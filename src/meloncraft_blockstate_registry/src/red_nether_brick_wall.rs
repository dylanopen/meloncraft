use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedNetherBrickWall {
    pub up: bool,
    pub waterlogged: bool,
    pub r#west: West,
    pub r#east: East,
    pub r#north: North,
    pub r#south: South,
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

impl BlockState for RedNetherBrickWall {
    fn to_id(&self) -> i32 {
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#east == East::Tall
        {
            return 19530;
        }
        if self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 19402;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#south == South::Tall
        {
            return 19345;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#waterlogged == true
        {
            return 19360;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 19383;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#north == North::Low
        {
            return 19363;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 19494;
        }
        if self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 19268;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::None
        {
            return 19506;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#west == West::None
            && self.r#east == East::Tall
        {
            return 19517;
        }
        if self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 19518;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#east == East::Low
        {
            return 19355;
        }
        if self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == true
        {
            return 19264;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 19289;
        }
        if self.r#up == false
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
        {
            return 19435;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#up == true
        {
            return 19484;
        }
        if self.r#up == false
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#east == East::Tall
        {
            return 19433;
        }
        if self.r#up == false
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#west == West::Tall
        {
            return 19507;
        }
        if self.r#north == North::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 19244;
        }
        if self.r#north == North::None
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#up == false
        {
            return 19347;
        }
        if self.r#south == South::None
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 19358;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 19424;
        }
        if self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 19365;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 19485;
        }
        if self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#up == false
            && self.r#north == North::Tall
        {
            return 19409;
        }
        if self.r#up == false
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 19231;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#east == East::None
        {
            return 19221;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#waterlogged == true
        {
            return 19239;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Tall
        {
            return 19344;
        }
        if self.r#waterlogged == false
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#east == East::Low
        {
            return 19417;
        }
        if self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Tall
        {
            return 19426;
        }
        if self.r#up == true
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::None
        {
            return 19220;
        }
        if self.r#up == true
            && self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#south == South::Low
        {
            return 19222;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#south == South::Tall
        {
            return 19523;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::None
        {
            return 19351;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 19428;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 19524;
        }
        if self.r#west == West::None
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#up == true
        {
            return 19403;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 19338;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 19425;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 19237;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 19295;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 19281;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Tall
        {
            return 19450;
        }
        if self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#west == West::Low
        {
            return 19419;
        }
        if self.r#up == false
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == true
        {
            return 19418;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 19337;
        }
        if self.r#up == true
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 19357;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 19405;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#up == true
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#north == North::Low
        {
            return 19247;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::Low
        {
            return 19521;
        }
        if self.r#up == true
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 19208;
        }
        if self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::None
        {
            return 19472;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Low
        {
            return 19359;
        }
        if self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::None
        {
            return 19277;
        }
        if self.r#west == West::Low
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Low
        {
            return 19443;
        }
        if self.r#west == West::None
            && self.r#up == false
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
        {
            return 19397;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#waterlogged == true
        {
            return 19436;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 19408;
        }
        if self.r#waterlogged == true
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 19394;
        }
        if self.r#west == West::Low
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#up == false
        {
            return 19251;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#north == North::None
        {
            return 19332;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#waterlogged == false
        {
            return 19477;
        }
        if self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 19378;
        }
        if self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#north == North::Low
        {
            return 19249;
        }
        if self.r#up == true
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#waterlogged == false
        {
            return 19223;
        }
        if self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#east == East::None
        {
            return 19228;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#up == true
        {
            return 19449;
        }
        if self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 19278;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 19498;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#east == East::Tall
        {
            return 19512;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 19333;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::None
            && self.r#south == South::Tall
        {
            return 19304;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 19371;
        }
        if self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#waterlogged == true
        {
            return 19420;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#east == East::None
        {
            return 19240;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 19217;
        }
        if self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#up == true
        {
            return 19330;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#west == West::None
            && self.r#north == North::Tall
        {
            return 19412;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::Tall
        {
            return 19275;
        }
        if self.r#up == false
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#north == North::None
        {
            return 19346;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#south == South::None
        {
            return 19398;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 19266;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 19325;
        }
        if self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 19448;
        }
        if self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#east == East::Tall
        {
            return 19455;
        }
        if self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::Tall
        {
            return 19366;
        }
        if self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 19373;
        }
        if self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#up == true
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#waterlogged == false
        {
            return 19213;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 19369;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == true
        {
            return 19437;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#south == South::None
        {
            return 19317;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::None
        {
            return 19467;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::Tall
        {
            return 19492;
        }
        if self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#south == South::None
        {
            return 19502;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#east == East::None
        {
            return 19235;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 19368;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#west == West::Tall
        {
            return 19393;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 19376;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#east == East::Tall
        {
            return 19525;
        }
        if self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 19422;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#north == North::Tall
        {
            return 19527;
        }
        if self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#up == true
        {
            return 19273;
        }
        if self.r#up == false
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 19219;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#west == West::Low
        {
            return 19236;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Low
        {
            return 19225;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#up == false
        {
            return 19454;
        }
        if self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 19238;
        }
        if self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::None
        {
            return 19241;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 19282;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#east == East::None
        {
            return 19276;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 19476;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == false
        {
            return 19516;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#south == South::Tall
        {
            return 19377;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#up == false
        {
            return 19286;
        }
        if self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#north == North::None
        {
            return 19233;
        }
        if self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#waterlogged == false
        {
            return 19290;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 19262;
        }
        if self.r#west == West::Low
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == false
        {
            return 19326;
        }
        if self.r#up == true
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#west == West::None
        {
            return 19379;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#up == true
        {
            return 19381;
        }
        if self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#waterlogged == true
        {
            return 19305;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#south == South::Tall
        {
            return 19350;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#west == West::Low
        {
            return 19308;
        }
        if self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::Low
        {
            return 19316;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#west == West::Tall
        {
            return 19354;
        }
        if self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Low
        {
            return 19481;
        }
        if self.r#up == false
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#waterlogged == true
        {
            return 19250;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::Low
        {
            return 19473;
        }
        if self.r#waterlogged == false
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#east == East::Tall
        {
            return 19483;
        }
        if self.r#up == false
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Tall
        {
            return 19491;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#west == West::Tall
        {
            return 19522;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 19416;
        }
        if self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#south == South::Low
        {
            return 19519;
        }
        if self.r#west == West::Tall
            && self.r#up == false
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
        {
            return 19303;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#east == East::Low
        {
            return 19385;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 19439;
        }
        if self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 19497;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 19499;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#south == South::None
        {
            return 19500;
        }
        if self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 19306;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 19400;
        }
        if self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Tall
        {
            return 19414;
        }
        if self.r#up == true
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#west == West::None
        {
            return 19271;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 19456;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#north == North::Low
        {
            return 19261;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#up == false
        {
            return 19423;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#west == West::Low
        {
            return 19260;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#up == true
            && self.r#south == South::None
            && self.r#west == West::Tall
        {
            return 19321;
        }
        if self.r#west == West::Low
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#north == North::None
        {
            return 19458;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#up == true
            && self.r#north == North::Low
        {
            return 19463;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 19480;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#up == false
        {
            return 19442;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::None
            && self.r#waterlogged == true
        {
            return 19214;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#east == East::Tall
        {
            return 19493;
        }
        if self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 19364;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::Low
        {
            return 19508;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 19211;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::Low
        {
            return 19407;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#waterlogged == true
        {
            return 19478;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#west == West::Tall
        {
            return 19474;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::None
            && self.r#waterlogged == true
        {
            return 19226;
        }
        if self.r#south == South::Low
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::Tall
        {
            return 19298;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 19503;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 19339;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Tall
        {
            return 19413;
        }
        if self.r#up == false
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Tall
        {
            return 19528;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#south == South::Low
        {
            return 19438;
        }
        if self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#north == North::Tall
        {
            return 19392;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == false
        {
            return 19471;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 19468;
        }
        if self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#east == East::Low
        {
            return 19410;
        }
        if self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#waterlogged == false
        {
            return 19374;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#up == true
        {
            return 19293;
        }
        if self.r#west == West::Low
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#north == North::None
        {
            return 19230;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 19269;
        }
        if self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 19301;
        }
        if self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#south == South::Tall
        {
            return 19348;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 19327;
        }
        if self.r#up == false
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#waterlogged == true
        {
            return 19324;
        }
        if self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#north == North::Low
        {
            return 19387;
        }
        if self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#west == West::None
        {
            return 19292;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::None
        {
            return 19514;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#south == South::None
        {
            return 19470;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#east == East::None
            && self.r#waterlogged == false
        {
            return 19283;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#up == true
        {
            return 19353;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#up == true
            && self.r#north == North::Tall
        {
            return 19415;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == true
            && self.r#north == North::Low
        {
            return 19465;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Low
        {
            return 19404;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 19431;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#south == South::Low
        {
            return 19302;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#north == North::None
        {
            return 19441;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 19229;
        }
        if self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#east == East::None
        {
            return 19311;
        }
        if self.r#south == South::Low
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#west == West::None
        {
            return 19328;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 19309;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Low
        {
            return 19356;
        }
        if self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 19380;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == false
        {
            return 19511;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Low
        {
            return 19390;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#south == South::Tall
        {
            return 19386;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::None
        {
            return 19529;
        }
        if self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 19322;
        }
        if self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 19399;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == false
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#waterlogged == true
        {
            return 19227;
        }
        if self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 19391;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#up == false
            && self.r#east == East::Tall
        {
            return 19526;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#south == South::Tall
        {
            return 19232;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 19218;
        }
        if self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#up == true
        {
            return 19341;
        }
        if self.r#up == false
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == true
        {
            return 19444;
        }
        if self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 19242;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == false
        {
            return 19254;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Low
        {
            return 19395;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::Tall
        {
            return 19427;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#south == South::Tall
        {
            return 19234;
        }
        if self.r#north == North::Low
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 19255;
        }
        if self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#south == South::Tall
        {
            return 19307;
        }
        if self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
        {
            return 19313;
        }
        if self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#up == false
        {
            return 19335;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#south == South::Low
        {
            return 19300;
        }
        if self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#north == North::Low
        {
            return 19482;
        }
        if self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 19434;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == true
        {
            return 19460;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#south == South::Tall
        {
            return 19452;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
        {
            return 19446;
        }
        if self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#south == South::Tall
        {
            return 19270;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 19318;
        }
        if self.r#up == false
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == true
        {
            return 19384;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#east == East::Low
        {
            return 19382;
        }
        if self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#west == West::Tall
        {
            return 19258;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#up == true
            && self.r#south == South::Tall
        {
            return 19340;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#west == West::Tall
        {
            return 19372;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#north == North::None
        {
            return 19447;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 19453;
        }
        if self.r#up == false
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::None
        {
            return 19457;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 19490;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
        {
            return 19285;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 19496;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Low
        {
            return 19510;
        }
        if self.r#up == true
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#east == East::Tall
        {
            return 19464;
        }
        if self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 19388;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#north == North::Tall
        {
            return 19411;
        }
        if self.r#up == true
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#south == South::None
        {
            return 19429;
        }
        if self.r#east == East::None
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 19274;
        }
        if self.r#west == West::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#north == North::Low
        {
            return 19267;
        }
        if self.r#up == false
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#east == East::None
        {
            return 19243;
        }
        if self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 19279;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::None
        {
            return 19294;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 19291;
        }
        if self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 19367;
        }
        if self.r#up == true
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == true
        {
            return 19520;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 19487;
        }
        if self.r#up == true
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Low
        {
            return 19320;
        }
        if self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#up == false
        {
            return 19334;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == true
        {
            return 19216;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Tall
        {
            return 19287;
        }
        if self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == false
        {
            return 19469;
        }
        if self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#east == East::None
        {
            return 19288;
        }
        if self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#east == East::None
        {
            return 19297;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#up == false
        {
            return 19323;
        }
        if self.r#north == North::Low
            && self.r#up == true
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#waterlogged == true
        {
            return 19352;
        }
        if self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 19272;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#waterlogged == false
        {
            return 19495;
        }
        if self.r#waterlogged == false
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#south == South::Tall
        {
            return 19531;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 19252;
        }
        if self.r#up == true
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#south == South::Low
        {
            return 19401;
        }
        if self.r#west == West::Low
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == true
        {
            return 19299;
        }
        if self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 19375;
        }
        if self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#waterlogged == true
        {
            return 19256;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 19280;
        }
        if self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 19336;
        }
        if self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#south == South::None
        {
            return 19505;
        }
        if self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#east == East::Tall
        {
            return 19488;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == true
        {
            return 19257;
        }
        if self.r#up == true
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 19246;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#north == North::Low
        {
            return 19253;
        }
        if self.r#up == true
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#east == East::None
        {
            return 19259;
        }
        if self.r#north == North::None
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#west == West::Low
        {
            return 19329;
        }
        if self.r#west == West::Low
            && self.r#up == true
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#waterlogged == false
        {
            return 19212;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#up == false
        {
            return 19430;
        }
        if self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Low
        {
            return 19515;
        }
        if self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Tall
        {
            return 19310;
        }
        if self.r#south == South::None
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#waterlogged == true
        {
            return 19245;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#north == North::None
        {
            return 19440;
        }
        if self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#west == West::Low
        {
            return 19509;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Low
        {
            return 19406;
        }
        if self.r#up == false
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#west == West::None
        {
            return 19361;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::None
        {
            return 19432;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 19466;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#south == South::Low
        {
            return 19331;
        }
        if self.r#west == West::None
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#waterlogged == false
        {
            return 19475;
        }
        if self.r#waterlogged == true
            && self.r#up == false
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 19504;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == true
        {
            return 19296;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#north == North::None
        {
            return 19224;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#waterlogged == false
        {
            return 19349;
        }
        if self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::None
            && self.r#west == West::Low
        {
            return 19248;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == false
        {
            return 19315;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#south == South::Tall
        {
            return 19421;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#south == South::None
        {
            return 19396;
        }
        if self.r#up == false
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#south == South::Low
        {
            return 19445;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#east == East::Tall
        {
            return 19451;
        }
        if self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#up == false
        {
            return 19265;
        }
        if self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 19479;
        }
        if self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::None
        {
            return 19209;
        }
        if self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 19312;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::None
        {
            return 19215;
        }
        if self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
        {
            return 19314;
        }
        if self.r#south == South::Low
            && self.r#west == West::None
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 19370;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 19486;
        }
        if self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 19389;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#up == true
        {
            return 19342;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#west == West::Low
        {
            return 19461;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 19489;
        }
        if self.r#up == false
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 19459;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 19210;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::Tall
        {
            return 19284;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#up == true
        {
            return 19319;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::None
        {
            return 19362;
        }
        if self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#up == true
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#south == South::Tall
        {
            return 19343;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#up == true
        {
            return 19501;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 19462;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#east == East::None
            && self.r#south == South::Low
        {
            return 19263;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#south == South::Low
        {
            return 19513;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 19530 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 19402 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19345 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 19360 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19383 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 19363 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 19494 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19268 {
            return Some(RedNetherBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19506 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 19517 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19518 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 19355 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 19264 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19289 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 19435 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19484 {
            return Some(RedNetherBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 19433 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 19507 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 19244 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19347 {
            return Some(RedNetherBrickWall {
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 19358 {
            return Some(RedNetherBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 19424 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 19365 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19485 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 19409 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 19231 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 19221 {
            return Some(RedNetherBrickWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 19239 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19344 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19417 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 19426 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 19220 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 19222 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 19523 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19351 {
            return Some(RedNetherBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 19428 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 19524 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 19403 {
            return Some(RedNetherBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 19338 {
            return Some(RedNetherBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19425 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 19237 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19295 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 19281 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 19450 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 19419 {
            return Some(RedNetherBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 19418 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19337 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 19357 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19405 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 19247 {
            return Some(RedNetherBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 19521 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 19208 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 19472 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 19359 {
            return Some(RedNetherBrickWall {
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 19277 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 19443 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 19397 {
            return Some(RedNetherBrickWall {
                r#west: West::None,
                r#up: false,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19436 {
            return Some(RedNetherBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19408 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19394 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 19251 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 19332 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 19477 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19378 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19249 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 19223 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19228 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 19449 {
            return Some(RedNetherBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 19278 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19498 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19512 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 19333 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 19304 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19371 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 19420 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19240 {
            return Some(RedNetherBrickWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 19217 {
            return Some(RedNetherBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 19330 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 19412 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 19275 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19346 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 19398 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19266 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19325 {
            return Some(RedNetherBrickWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 19448 {
            return Some(RedNetherBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 19455 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 19366 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 19373 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 19213 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19369 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19437 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 19317 {
            return Some(RedNetherBrickWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 19467 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 19492 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 19502 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 19235 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 19368 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19393 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 19376 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 19525 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 19422 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 19527 {
            return Some(RedNetherBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 19273 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 19219 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19236 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 19225 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 19454 {
            return Some(RedNetherBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 19238 {
            return Some(RedNetherBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19241 {
            return Some(RedNetherBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 19282 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19276 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 19476 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 19516 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 19377 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19286 {
            return Some(RedNetherBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 19233 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 19290 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19262 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 19326 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19379 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 19381 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 19305 {
            return Some(RedNetherBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19350 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19308 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 19316 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 19354 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 19481 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 19250 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19473 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19483 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 19491 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 19522 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 19416 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19519 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 19303 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19385 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 19439 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 19497 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 19499 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19500 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 19306 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19400 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 19414 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 19271 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 19456 {
            return Some(RedNetherBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19261 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 19423 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 19260 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 19321 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 19458 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 19463 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 19480 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19442 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 19214 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19493 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 19364 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19508 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 19211 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 19407 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 19478 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19474 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 19226 {
            return Some(RedNetherBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19298 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 19503 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 19339 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 19413 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19528 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 19438 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 19392 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 19471 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 19468 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 19410 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19374 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19293 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 19230 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 19269 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19301 {
            return Some(RedNetherBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 19348 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19327 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 19324 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19387 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19292 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 19514 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 19470 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 19283 {
            return Some(RedNetherBrickWall {
                r#south: South::None,
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19353 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 19415 {
            return Some(RedNetherBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 19465 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 19404 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19431 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 19302 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 19441 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 19229 {
            return Some(RedNetherBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 19311 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 19328 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 19309 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19356 {
            return Some(RedNetherBrickWall {
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19380 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19511 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19390 {
            return Some(RedNetherBrickWall {
                r#south: South::None,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 19386 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 19529 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 19322 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 19399 {
            return Some(RedNetherBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19227 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19391 {
            return Some(RedNetherBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19526 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 19232 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19218 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19341 {
            return Some(RedNetherBrickWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 19444 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19242 {
            return Some(RedNetherBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 19254 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19395 {
            return Some(RedNetherBrickWall {
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19427 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19234 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 19255 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19307 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 19313 {
            return Some(RedNetherBrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19335 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 19300 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 19482 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 19434 {
            return Some(RedNetherBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 19460 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 19452 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19446 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 19270 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 19318 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 19384 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19382 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19258 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 19340 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 19372 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 19447 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 19453 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 19457 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 19490 {
            return Some(RedNetherBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19285 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19496 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19510 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 19464 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 19388 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19411 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 19429 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 19274 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19267 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 19243 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 19279 {
            return Some(RedNetherBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19294 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 19291 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19367 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 19520 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19487 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 19320 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19334 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 19216 {
            return Some(RedNetherBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19287 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 19469 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19288 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 19297 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 19323 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 19352 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#up: true,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19272 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19495 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19531 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 19252 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19401 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 19299 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19375 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 19256 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19280 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19336 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 19505 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19488 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 19257 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 19246 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19253 {
            return Some(RedNetherBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 19259 {
            return Some(RedNetherBrickWall {
                r#up: true,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 19329 {
            return Some(RedNetherBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 19212 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19430 {
            return Some(RedNetherBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 19515 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 19310 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 19245 {
            return Some(RedNetherBrickWall {
                r#south: South::None,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19440 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 19509 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 19406 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19361 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 19432 {
            return Some(RedNetherBrickWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19466 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 19331 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 19475 {
            return Some(RedNetherBrickWall {
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19504 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19296 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 19224 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 19349 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19248 {
            return Some(RedNetherBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 19315 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19421 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19396 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 19445 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 19451 {
            return Some(RedNetherBrickWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19265 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 19479 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 19209 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 19312 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 19215 {
            return Some(RedNetherBrickWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 19314 {
            return Some(RedNetherBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19370 {
            return Some(RedNetherBrickWall {
                r#south: South::Low,
                r#west: West::None,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 19486 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19389 {
            return Some(RedNetherBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 19342 {
            return Some(RedNetherBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 19461 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 19489 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19459 {
            return Some(RedNetherBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19210 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19284 {
            return Some(RedNetherBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 19319 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 19362 {
            return Some(RedNetherBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 19343 {
            return Some(RedNetherBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19501 {
            return Some(RedNetherBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 19462 {
            return Some(RedNetherBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19263 {
            return Some(RedNetherBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 19513 {
            return Some(RedNetherBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        return None;
    }
}
