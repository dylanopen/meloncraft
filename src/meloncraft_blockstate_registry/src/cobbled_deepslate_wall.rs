use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CobbledDeepslateWall {
    pub r#south: South,
    pub waterlogged: bool,
    pub r#east: East,
    pub up: bool,
    pub r#west: West,
    pub r#north: North,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum North {
    None,
    Low,
    Tall,
}

impl BlockState for CobbledDeepslateWall {
    fn to_id(&self) -> i32 {
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::Low
        {
            return 27851;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::None
        {
            return 27889;
        }
        if self.r#up == true
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#north == North::Tall
        {
            return 27910;
        }
        if self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#south == South::Low
        {
            return 27975;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#west == West::None
        {
            return 28015;
        }
        if self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::None
        {
            return 28093;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#east == East::None
        {
            return 27867;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#up == true
        {
            return 27837;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 28114;
        }
        if self.r#south == South::None
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Low
        {
            return 27961;
        }
        if self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 27977;
        }
        if self.r#up == false
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::Tall
        {
            return 28108;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::None
        {
            return 27847;
        }
        if self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#up == true
        {
            return 27852;
        }
        if self.r#waterlogged == false
            && self.r#up == false
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 27820;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#east == East::None
        {
            return 27873;
        }
        if self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#up == false
        {
            return 27913;
        }
        if self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#up == true
            && self.r#north == North::None
            && self.r#south == South::Tall
        {
            return 27835;
        }
        if self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == true
        {
            return 27850;
        }
        if self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
        {
            return 27816;
        }
        if self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#east == East::None
        {
            return 27887;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#up == false
        {
            return 27925;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 27964;
        }
        if self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#north == North::Tall
        {
            return 27908;
        }
        if self.r#north == North::None
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#waterlogged == false
        {
            return 28031;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 28035;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#up == true
        {
            return 28043;
        }
        if self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == true
        {
            return 28082;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == false
        {
            return 28097;
        }
        if self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#west == West::None
        {
            return 28102;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#north == North::None
        {
            return 27945;
        }
        if self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#south == South::None
            && self.r#west == West::Low
        {
            return 28103;
        }
        if self.r#up == true
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#west == West::None
        {
            return 28111;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
        {
            return 27923;
        }
        if self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#up == false
        {
            return 27937;
        }
        if self.r#up == true
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#east == East::Low
        {
            return 27947;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 27878;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::Low
        {
            return 27957;
        }
        if self.r#up == true
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::None
        {
            return 27895;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 27814;
        }
        if self.r#west == West::None
            && self.r#up == true
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#south == South::Low
        {
            return 27823;
        }
        if self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::Low
        {
            return 27982;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#west == West::Tall
        {
            return 28134;
        }
        if self.r#up == false
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#west == West::Low
        {
            return 27881;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#east == East::Low
        {
            return 27972;
        }
        if self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#up == false
        {
            return 28072;
        }
        if self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#south == South::Tall
        {
            return 27836;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#south == South::None
        {
            return 27883;
        }
        if self.r#west == West::Low
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#waterlogged == true
        {
            return 27824;
        }
        if self.r#east == East::None
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#up == true
            && self.r#north == North::Tall
        {
            return 27886;
        }
        if self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#up == true
        {
            return 27912;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 28005;
        }
        if self.r#south == South::None
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#waterlogged == true
        {
            return 27926;
        }
        if self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 28030;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#up == false
        {
            return 28118;
        }
        if self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#north == North::Tall
        {
            return 28021;
        }
        if self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#east == East::None
        {
            return 27842;
        }
        if self.r#up == true
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#east == East::None
        {
            return 27875;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == false
        {
            return 27890;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 27818;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 27918;
        }
        if self.r#south == South::Low
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#east == East::None
        {
            return 27865;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 27986;
        }
        if self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#west == West::Low
        {
            return 28001;
        }
        if self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#north == North::None
        {
            return 27935;
        }
        if self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#east == East::Low
        {
            return 28000;
        }
        if self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == true
            && self.r#north == North::None
        {
            return 28027;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#up == false
            && self.r#east == East::Tall
        {
            return 28049;
        }
        if self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::None
        {
            return 27921;
        }
        if self.r#south == South::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 28066;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#east == East::Tall
        {
            return 28077;
        }
        if self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#south == South::Tall
        {
            return 27874;
        }
        if self.r#west == West::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#north == North::Low
        {
            return 27956;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#west == West::Tall
        {
            return 28026;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#up == true
        {
            return 28112;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 28119;
        }
        if self.r#north == North::None
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 27950;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#west == West::Tall
        {
            return 28086;
        }
        if self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 27934;
        }
        if self.r#north == North::Low
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 27882;
        }
        if self.r#up == false
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == true
        {
            return 27901;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#west == West::Tall
        {
            return 27813;
        }
        if self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#north == North::None
        {
            return 27939;
        }
        if self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#up == true
        {
            return 28006;
        }
        if self.r#up == true
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#east == East::Low
        {
            return 27932;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 28046;
        }
        if self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 28064;
        }
        if self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#up == true
        {
            return 28065;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 28070;
        }
        if self.r#west == West::None
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#east == East::None
            && self.r#waterlogged == false
        {
            return 27862;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#north == North::Low
        {
            return 27983;
        }
        if self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#waterlogged == true
        {
            return 27915;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == true
        {
            return 28101;
        }
        if self.r#up == true
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#waterlogged == false
        {
            return 27960;
        }
        if self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 27998;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 27826;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#south == South::Low
        {
            return 28113;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 27821;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::Tall
        {
            return 28094;
        }
        if self.r#east == East::None
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 27864;
        }
        if self.r#west == West::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 27811;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#up == false
        {
            return 28129;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 27993;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#east == East::Low
        {
            return 27959;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Tall
        {
            return 28057;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 28044;
        }
        if self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 28053;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 28048;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#up == false
        {
            return 28121;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::None
        {
            return 27973;
        }
        if self.r#up == true
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 28042;
        }
        if self.r#west == West::None
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == true
        {
            return 28075;
        }
        if self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == false
        {
            return 28110;
        }
        if self.r#north == North::None
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#east == East::Low
        {
            return 27953;
        }
        if self.r#west == West::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 27965;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#north == North::None
        {
            return 28061;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#up == true
        {
            return 27946;
        }
        if self.r#up == false
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 28002;
        }
        if self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::None
        {
            return 27832;
        }
        if self.r#west == West::None
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 27838;
        }
        if self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == true
        {
            return 28067;
        }
        if self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#south == South::None
        {
            return 28073;
        }
        if self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#west == West::Low
        {
            return 28040;
        }
        if self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 28120;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#south == South::None
        {
            return 27885;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#waterlogged == false
        {
            return 27978;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#up == true
        {
            return 28080;
        }
        if self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 27891;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#north == North::Tall
        {
            return 27992;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Tall
        {
            return 27843;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#up == true
        {
            return 27920;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::None
        {
            return 27967;
        }
        if self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Low
        {
            return 28050;
        }
        if self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 28060;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::Tall
        {
            return 28074;
        }
        if self.r#up == true
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#north == North::Tall
        {
            return 28004;
        }
        if self.r#west == West::Tall
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#east == East::Low
        {
            return 28020;
        }
        if self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 27914;
        }
        if self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 27846;
        }
        if self.r#north == North::Low
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 27866;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 27981;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#east == East::Tall
        {
            return 28095;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::Tall
        {
            return 28019;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::Tall
        {
            return 28078;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#west == West::None
        {
            return 28096;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 28032;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 28059;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#north == North::None
        {
            return 28041;
        }
        if self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Tall
        {
            return 28062;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 27962;
        }
        if self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#up == true
        {
            return 28089;
        }
        if self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#up == true
        {
            return 28100;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#south == South::None
        {
            return 27857;
        }
        if self.r#north == North::None
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 27817;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == false
        {
            return 27930;
        }
        if self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 27840;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == false
        {
            return 28023;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#west == West::Low
        {
            return 28034;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Tall
        {
            return 27876;
        }
        if self.r#west == West::Low
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#north == North::Tall
        {
            return 28133;
        }
        if self.r#east == East::None
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#up == false
        {
            return 27903;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#up == false
            && self.r#north == North::Low
        {
            return 27988;
        }
        if self.r#north == North::Low
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::None
        {
            return 27877;
        }
        if self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#north == North::Tall
        {
            return 28022;
        }
        if self.r#up == false
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#waterlogged == true
        {
            return 28131;
        }
        if self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#up == true
        {
            return 27907;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 28132;
        }
        if self.r#up == false
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 27906;
        }
        if self.r#east == East::None
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 27902;
        }
        if self.r#north == North::None
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#south == South::None
        {
            return 27928;
        }
        if self.r#up == true
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 28104;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#west == West::Tall
        {
            return 27909;
        }
        if self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#east == East::Tall
        {
            return 28055;
        }
        if self.r#up == false
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 27844;
        }
        if self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 28058;
        }
        if self.r#west == West::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::Low
        {
            return 27976;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#east == East::Tall
        {
            return 28099;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::None
        {
            return 28028;
        }
        if self.r#up == false
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#north == North::None
        {
            return 27949;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 28029;
        }
        if self.r#up == true
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#east == East::Tall
        {
            return 28123;
        }
        if self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::None
        {
            return 27994;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#up == true
        {
            return 27922;
        }
        if self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#up == true
        {
            return 27884;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#west == West::Low
        {
            return 27854;
        }
        if self.r#up == true
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#waterlogged == false
        {
            return 27971;
        }
        if self.r#west == West::None
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == false
        {
            return 28090;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::Low
        {
            return 28084;
        }
        if self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 27974;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#west == West::None
            && self.r#north == North::None
        {
            return 27931;
        }
        if self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 28085;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#north == North::None
        {
            return 27834;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 27900;
        }
        if self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#west == West::None
        {
            return 28051;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 27845;
        }
        if self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 27868;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 27952;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#north == North::Low
        {
            return 27863;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 27948;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == false
        {
            return 27999;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#south == South::None
        {
            return 28036;
        }
        if self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Tall
        {
            return 28107;
        }
        if self.r#south == South::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::None
        {
            return 27822;
        }
        if self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == false
        {
            return 27870;
        }
        if self.r#up == true
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#east == East::None
        {
            return 27872;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 27856;
        }
        if self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == true
        {
            return 28083;
        }
        if self.r#west == West::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::Tall
        {
            return 27996;
        }
        if self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 28007;
        }
        if self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#west == West::Tall
        {
            return 28014;
        }
        if self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::Low
        {
            return 28076;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#up == true
        {
            return 27827;
        }
        if self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#north == North::None
        {
            return 27954;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::Low
        {
            return 27858;
        }
        if self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#up == false
        {
            return 27894;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#south == South::Low
        {
            return 27897;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::None
        {
            return 27966;
        }
        if self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 27940;
        }
        if self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::Tall
        {
            return 28127;
        }
        if self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 28126;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#east == East::Low
        {
            return 27929;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#up == false
        {
            return 28012;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Tall
        {
            return 27984;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#south == South::Tall
        {
            return 28087;
        }
        if self.r#west == West::None
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
        {
            return 27898;
        }
        if self.r#up == true
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 27924;
        }
        if self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::None
        {
            return 28054;
        }
        if self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == false
        {
            return 27989;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#west == West::Low
        {
            return 27860;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#south == South::Low
        {
            return 27899;
        }
        if self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 28038;
        }
        if self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
        {
            return 27911;
        }
        if self.r#up == true
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#east == East::None
        {
            return 27812;
        }
        if self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::None
        {
            return 27819;
        }
        if self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#up == false
        {
            return 28010;
        }
        if self.r#waterlogged == false
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::Tall
        {
            return 28025;
        }
        if self.r#west == West::None
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#south == South::Low
        {
            return 28003;
        }
        if self.r#south == South::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 28063;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#up == true
        {
            return 27888;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::None
        {
            return 27849;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#north == North::None
        {
            return 27927;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 27933;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#waterlogged == true
        {
            return 27938;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#west == West::Low
        {
            return 28013;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#south == South::None
        {
            return 28106;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#waterlogged == true
        {
            return 27848;
        }
        if self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#west == West::Tall
        {
            return 28116;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 27859;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 27893;
        }
        if self.r#up == true
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 27991;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 27916;
        }
        if self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#east == East::Tall
        {
            return 28052;
        }
        if self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Low
        {
            return 28115;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 27995;
        }
        if self.r#up == false
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#east == East::Low
        {
            return 27941;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 28122;
        }
        if self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 28088;
        }
        if self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#east == East::Low
        {
            return 27997;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#east == East::Tall
        {
            return 28037;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#up == false
        {
            return 28105;
        }
        if self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::None
        {
            return 28045;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == true
        {
            return 28092;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == false
        {
            return 27987;
        }
        if self.r#west == West::Tall
            && self.r#up == false
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#waterlogged == true
        {
            return 27963;
        }
        if self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#up == true
        {
            return 28068;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#up == false
        {
            return 28117;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#up == false
        {
            return 27830;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::None
            && self.r#north == North::None
        {
            return 27831;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#west == West::Low
        {
            return 28016;
        }
        if self.r#up == false
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::Tall
        {
            return 28009;
        }
        if self.r#up == true
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#west == West::Low
        {
            return 27896;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#south == South::Low
        {
            return 27936;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#waterlogged == true
        {
            return 27861;
        }
        if self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#south == South::None
        {
            return 27892;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Low
        {
            return 27944;
        }
        if self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::Low
        {
            return 27958;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 28125;
        }
        if self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 27839;
        }
        if self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 28008;
        }
        if self.r#up == false
            && self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#waterlogged == true
        {
            return 28047;
        }
        if self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 28017;
        }
        if self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#up == true
        {
            return 28124;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#west == West::None
        {
            return 27955;
        }
        if self.r#up == true
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 28091;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#up == true
        {
            return 27828;
        }
        if self.r#up == false
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#north == North::Tall
        {
            return 27917;
        }
        if self.r#waterlogged == true
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#west == West::Tall
        {
            return 28011;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#north == North::None
        {
            return 27942;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#waterlogged == false
        {
            return 27869;
        }
        if self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::None
        {
            return 27841;
        }
        if self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#north == North::Low
        {
            return 27969;
        }
        if self.r#up == false
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#east == East::None
        {
            return 27829;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#east == East::None
        {
            return 27833;
        }
        if self.r#up == true
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#waterlogged == false
        {
            return 27970;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#up == false
        {
            return 27904;
        }
        if self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 27980;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::Tall
        {
            return 28033;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 28071;
        }
        if self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 27871;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#north == North::None
        {
            return 27919;
        }
        if self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#waterlogged == true
        {
            return 28039;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#west == West::Tall
        {
            return 27855;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 28069;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#west == West::None
        {
            return 27880;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Tall
        {
            return 28128;
        }
        if self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 27825;
        }
        if self.r#up == true
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#waterlogged == false
        {
            return 27815;
        }
        if self.r#west == West::None
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#up == true
        {
            return 27943;
        }
        if self.r#up == false
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 27985;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Tall
        {
            return 27879;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Low
        {
            return 27990;
        }
        if self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == true
            && self.r#north == North::Low
        {
            return 27979;
        }
        if self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 28056;
        }
        if self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 27905;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 28098;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 28109;
        }
        if self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#waterlogged == true
        {
            return 28130;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#up == false
        {
            return 27951;
        }
        if self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#up == true
        {
            return 27968;
        }
        if self.r#up == false
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 28024;
        }
        if self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::Low
        {
            return 28079;
        }
        if self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 28081;
        }
        if self.r#up == false
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::None
        {
            return 27853;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 28018;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27851 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 27889 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 27910 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 27975 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 28015 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 28093 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 27867 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 27837 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 28114 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 27961 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 27977 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 28108 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 27847 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 27852 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 27820 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#up: false,
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 27873 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 27913 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 27835 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 27850 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 27816 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 27887 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 27925 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 27964 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 27908 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 28031 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28035 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28043 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 28082 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28097 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 28102 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 27945 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 28103 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 28111 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 27923 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 27937 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 27947 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 27878 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 27957 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 27895 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 27814 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27823 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 27982 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 28134 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 27881 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 27972 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 28072 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 27836 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 27883 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 27824 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 27886 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 27912 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 28005 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 27926 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28030 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 28118 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 28021 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 27842 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 27875 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 27890 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 27818 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 27918 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 27865 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 27986 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 28001 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 27935 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 28000 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 28027 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 28049 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 27921 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 28066 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28077 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 27874 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 27956 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 28026 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28112 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 28119 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27950 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 28086 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 27934 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 27882 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 27901 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 27813 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 27939 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 28006 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 27932 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 28046 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 28064 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28065 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 28070 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 27862 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 27983 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 27915 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28101 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 27960 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 27998 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 27826 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 28113 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 27821 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 28094 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 27864 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 27811 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 28129 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 27993 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27959 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 28057 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 28044 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 28053 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28048 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 28121 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 27973 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 28042 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 28075 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28110 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 27953 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 27965 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 28061 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 27946 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 28002 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 27832 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 27838 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 28067 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 28073 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 28040 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28120 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27885 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 27978 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28080 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 27891 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 27992 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 27843 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 27920 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 27967 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 28050 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 28060 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28074 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 28004 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 28020 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 27914 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 27846 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 27866 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 27981 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28095 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 28019 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28078 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 28096 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 28032 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 28059 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28041 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 28062 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 27962 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 28089 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 28100 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 27857 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 27817 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27930 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 27840 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 28023 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 28034 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 27876 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28133 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 27903 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 27988 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 27877 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 28022 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 28131 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 27907 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 28132 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 27906 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 27902 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 27928 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 28104 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 27909 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 28055 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 27844 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28058 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 27976 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 28099 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 28028 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 27949 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 28029 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28123 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 27994 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 27922 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 27884 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 27854 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 27971 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28090 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28084 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 27974 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27931 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 28085 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 27834 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 27900 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 28051 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 27845 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 27868 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27952 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 27863 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 27948 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 27999 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 28036 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#north: North::None,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 28107 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 27822 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 27870 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 27872 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 27856 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28083 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 27996 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 28007 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 28014 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 28076 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 27827 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 27954 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 27858 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 27894 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 27897 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 27966 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 27940 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28127 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 28126 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27929 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 28012 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 27984 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28087 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 27898 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 27924 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 28054 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 27989 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 27860 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 27899 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28038 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 27911 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 27812 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 27819 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 28010 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 28025 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28003 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28063 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 27888 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 27849 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 27927 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 27933 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 27938 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28013 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28106 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 27848 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28116 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 27859 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 27893 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 27991 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 27916 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28052 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 28115 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 27995 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 27941 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 28122 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28088 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 27997 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 28037 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 28105 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 28045 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 28092 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 27987 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 27963 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28068 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 28117 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 27830 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 27831 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 28016 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 28009 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 27896 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 27936 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 27861 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 27892 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 27944 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 27958 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 28125 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 27839 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28008 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28047 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28017 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28124 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 27955 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 28091 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 27828 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 27917 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 28011 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 27942 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 27869 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 27841 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 27969 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 27829 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 27833 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 27970 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 27904 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 27980 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 28033 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 28071 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 27871 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27919 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 28039 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 27855 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 28069 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 27880 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 28128 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 27825 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 27815 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 27943 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 27985 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 27879 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 27990 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 27979 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 28056 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 27905 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28098 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28109 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 28130 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 27951 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 27968 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 28024 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28079 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28081 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 27853 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 28018 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        return None;
    }
}
