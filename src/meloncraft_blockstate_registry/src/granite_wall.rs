use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraniteWall {
    pub r#west: West,
    pub r#east: East,
    pub waterlogged: bool,
    pub r#north: North,
    pub r#south: South,
    pub up: bool,
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

impl BlockState for GraniteWall {
    fn to_id(&self) -> i32 {
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 17605;
        }
        if self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#south == South::Tall
        {
            return 17721;
        }
        if self.r#up == true
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#south == South::Low
        {
            return 17672;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 17636;
        }
        if self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == false
        {
            return 17863;
        }
        if self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == true
            && self.r#north == North::Tall
        {
            return 17876;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::None
        {
            return 17666;
        }
        if self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#east == East::Tall
        {
            return 17910;
        }
        if self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == true
        {
            return 17616;
        }
        if self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#up == false
        {
            return 17762;
        }
        if self.r#west == West::Low
            && self.r#east == East::None
            && self.r#up == false
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 17595;
        }
        if self.r#west == West::None
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#south == South::None
        {
            return 17624;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#west == West::Low
        {
            return 17745;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#up == false
            && self.r#north == North::Tall
        {
            return 17681;
        }
        if self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Low
        {
            return 17625;
        }
        if self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::Low
        {
            return 17748;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 17650;
        }
        if self.r#up == true
            && self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#south == South::None
        {
            return 17593;
        }
        if self.r#north == North::Low
            && self.r#west == West::None
            && self.r#up == false
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#waterlogged == false
        {
            return 17633;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 17682;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#east == East::Low
        {
            return 17784;
        }
        if self.r#up == false
            && self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#south == South::None
        {
            return 17632;
        }
        if self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#up == false
        {
            return 17791;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 17600;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#up == true
        {
            return 17864;
        }
        if self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#north == North::Tall
        {
            return 17887;
        }
        if self.r#south == South::None
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 17778;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#south == South::Tall
        {
            return 17623;
        }
        if self.r#west == West::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 17651;
        }
        if self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#north == North::Tall
        {
            return 17878;
        }
        if self.r#up == false
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#north == North::Low
        {
            return 17851;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#up == false
        {
            return 17870;
        }
        if self.r#up == false
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#waterlogged == true
        {
            return 17594;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#west == West::Tall
        {
            return 17710;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 17752;
        }
        if self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#east == East::Tall
        {
            return 17894;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#up == true
            && self.r#north == North::Low
        {
            return 17735;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#waterlogged == false
        {
            return 17891;
        }
        if self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 17793;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 17848;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#south == South::None
        {
            return 17734;
        }
        if self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::None
        {
            return 17823;
        }
        if self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#up == true
        {
            return 17893;
        }
        if self.r#south == South::Tall
            && self.r#up == false
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#west == West::Low
        {
            return 17694;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#south == South::Low
        {
            return 17711;
        }
        if self.r#west == West::None
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#waterlogged == true
        {
            return 17750;
        }
        if self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#up == false
        {
            return 17642;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#up == false
        {
            return 17695;
        }
        if self.r#west == West::None
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#up == false
        {
            return 17717;
        }
        if self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#west == West::Low
        {
            return 17736;
        }
        if self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#west == West::Low
        {
            return 17652;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#south == South::Low
        {
            return 17827;
        }
        if self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#west == West::None
        {
            return 17885;
        }
        if self.r#up == false
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 17655;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#up == false
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 17618;
        }
        if self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#east == East::Low
        {
            return 17775;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#waterlogged == false
        {
            return 17849;
        }
        if self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::None
        {
            return 17882;
        }
        if self.r#up == true
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::Low
        {
            return 17723;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 17766;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#south == South::None
        {
            return 17886;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#east == East::None
        {
            return 17635;
        }
        if self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#north == North::Tall
        {
            return 17779;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 17892;
        }
        if self.r#south == South::Low
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Tall
        {
            return 17817;
        }
        if self.r#south == South::Low
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#east == East::Tall
        {
            return 17818;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#west == West::Tall
        {
            return 17647;
        }
        if self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 17781;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == true
        {
            return 17620;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::None
        {
            return 17837;
        }
        if self.r#west == West::None
            && self.r#up == false
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == true
        {
            return 17654;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Tall
        {
            return 17866;
        }
        if self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#up == false
        {
            return 17619;
        }
        if self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#up == true
        {
            return 17639;
        }
        if self.r#north == North::Low
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#south == South::Tall
        {
            return 17867;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#east == East::None
        {
            return 17592;
        }
        if self.r#up == true
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Low
        {
            return 17758;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 17908;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 17777;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#waterlogged == false
        {
            return 17611;
        }
        if self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#up == false
        {
            return 17669;
        }
        if self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#west == West::None
            && self.r#east == East::None
        {
            return 17627;
        }
        if self.r#up == true
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 17590;
        }
        if self.r#west == West::Tall
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#north == North::Low
        {
            return 17842;
        }
        if self.r#up == false
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::Low
        {
            return 17631;
        }
        if self.r#up == false
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Low
        {
            return 17896;
        }
        if self.r#north == North::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Low
        {
            return 17702;
        }
        if self.r#east == East::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::None
        {
            return 17608;
        }
        if self.r#west == West::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 17645;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#north == North::None
        {
            return 17718;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#south == South::Low
        {
            return 17755;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 17889;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::None
            && self.r#south == South::Low
        {
            return 17714;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#up == false
        {
            return 17740;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#south == South::Tall
        {
            return 17688;
        }
        if self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#up == true
        {
            return 17771;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::Tall
        {
            return 17900;
        }
        if self.r#up == false
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 17680;
        }
        if self.r#up == false
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#south == South::None
        {
            return 17846;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 17597;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#north == North::Tall
        {
            return 17785;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#west == West::None
        {
            return 17675;
        }
        if self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#south == South::Low
        {
            return 17641;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#up == true
            && self.r#west == West::None
            && self.r#waterlogged == false
        {
            return 17603;
        }
        if self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 17686;
        }
        if self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 17811;
        }
        if self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#west == West::Tall
        {
            return 17689;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#waterlogged == false
        {
            return 17706;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::None
        {
            return 17628;
        }
        if self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::Low
        {
            return 17840;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#south == South::Low
        {
            return 17708;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#south == South::Tall
        {
            return 17729;
        }
        if self.r#south == South::None
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#west == West::Low
        {
            return 17742;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#south == South::None
        {
            return 17697;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#east == East::Tall
        {
            return 17883;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#north == North::Tall
        {
            return 17901;
        }
        if self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#west == West::Tall
        {
            return 17860;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::None
        {
            return 17705;
        }
        if self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#east == East::Low
        {
            return 17713;
        }
        if self.r#south == South::None
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#up == true
        {
            return 17805;
        }
        if self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#east == East::Tall
        {
            return 17813;
        }
        if self.r#up == true
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 17819;
        }
        if self.r#west == West::None
            && self.r#up == true
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#waterlogged == true
        {
            return 17660;
        }
        if self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#waterlogged == false
        {
            return 17833;
        }
        if self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 17671;
        }
        if self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#up == true
        {
            return 17698;
        }
        if self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 17835;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#up == false
            && self.r#east == East::None
            && self.r#south == South::Tall
        {
            return 17690;
        }
        if self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#south == South::Low
        {
            return 17820;
        }
        if self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#up == true
            && self.r#east == East::Tall
        {
            return 17829;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#west == West::None
        {
            return 17774;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 17802;
        }
        if self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#up == true
            && self.r#east == East::Tall
        {
            return 17831;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Low
        {
            return 17638;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 17773;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == false
        {
            return 17767;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
        {
            return 17604;
        }
        if self.r#up == false
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#north == North::None
        {
            return 17825;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Tall
        {
            return 17845;
        }
        if self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::None
        {
            return 17606;
        }
        if self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
        {
            return 17911;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#north == North::None
        {
            return 17589;
        }
        if self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::Low
        {
            return 17712;
        }
        if self.r#up == true
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 17795;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#waterlogged == true
        {
            return 17727;
        }
        if self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 17607;
        }
        if self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#south == South::Low
        {
            return 17640;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == false
        {
            return 17839;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 17859;
        }
        if self.r#up == false
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#west == West::Low
        {
            return 17763;
        }
        if self.r#up == false
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#west == West::None
        {
            return 17801;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 17674;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Low
        {
            return 17629;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#north == North::None
        {
            return 17719;
        }
        if self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#north == North::Tall
        {
            return 17897;
        }
        if self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 17807;
        }
        if self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#west == West::None
        {
            return 17687;
        }
        if self.r#up == false
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::Low
        {
            return 17776;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 17648;
        }
        if self.r#up == false
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 17810;
        }
        if self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#south == South::Tall
        {
            return 17622;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#north == North::None
        {
            return 17703;
        }
        if self.r#up == true
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#south == South::None
        {
            return 17770;
        }
        if self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 17824;
        }
        if self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 17868;
        }
        if self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#up == false
        {
            return 17730;
        }
        if self.r#north == North::Low
            && self.r#west == West::None
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == true
        {
            return 17858;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#south == South::Low
        {
            return 17821;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#north == North::Low
        {
            return 17857;
        }
        if self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#waterlogged == true
            && self.r#up == true
        {
            return 17780;
        }
        if self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 17884;
        }
        if self.r#west == West::None
            && self.r#south == South::None
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#waterlogged == true
        {
            return 17696;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#west == West::Low
        {
            return 17751;
        }
        if self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::None
        {
            return 17664;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == false
        {
            return 17754;
        }
        if self.r#west == West::Low
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == true
        {
            return 17757;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::None
        {
            return 17609;
        }
        if self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#north == North::Low
        {
            return 17739;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#up == false
        {
            return 17728;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#west == West::None
        {
            return 17768;
        }
        if self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#up == true
        {
            return 17904;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#up == false
            && self.r#north == North::None
            && self.r#waterlogged == false
        {
            return 17621;
        }
        if self.r#north == North::Low
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 17657;
        }
        if self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#up == true
        {
            return 17663;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 17725;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#south == South::Low
        {
            return 17716;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#north == North::Tall
        {
            return 17797;
        }
        if self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Low
        {
            return 17861;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 17667;
        }
        if self.r#west == West::Tall
            && self.r#up == true
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#east == East::Tall
        {
            return 17806;
        }
        if self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 17888;
        }
        if self.r#east == East::Low
            && self.r#up == false
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#north == North::None
        {
            return 17726;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == false
            && self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#south == South::Low
        {
            return 17822;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 17906;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#east == East::None
        {
            return 17665;
        }
        if self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 17899;
        }
        if self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::None
        {
            return 17834;
        }
        if self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#up == true
        {
            return 17626;
        }
        if self.r#up == false
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#south == South::Low
        {
            return 17646;
        }
        if self.r#up == true
            && self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 17591;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#west == West::Low
        {
            return 17760;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::None
            && self.r#north == North::None
        {
            return 17615;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#north == North::Tall
        {
            return 17783;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#west == West::None
        {
            return 17855;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 17856;
        }
        if self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == false
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#waterlogged == false
        {
            return 17741;
        }
        if self.r#up == true
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 17761;
        }
        if self.r#west == West::Low
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 17874;
        }
        if self.r#south == South::Tall
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#east == East::Low
        {
            return 17803;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#up == false
        {
            return 17786;
        }
        if self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#west == West::Tall
        {
            return 17692;
        }
        if self.r#up == false
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#waterlogged == true
        {
            return 17738;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#west == West::Tall
        {
            return 17809;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#up == false
        {
            return 17898;
        }
        if self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 17789;
        }
        if self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 17659;
        }
        if self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#up == false
        {
            return 17596;
        }
        if self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::Tall
        {
            return 17693;
        }
        if self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#up == true
        {
            return 17792;
        }
        if self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 17670;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == true
            && self.r#north == North::Tall
        {
            return 17676;
        }
        if self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#east == East::Low
        {
            return 17733;
        }
        if self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == true
        {
            return 17788;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::Low
        {
            return 17634;
        }
        if self.r#north == North::None
            && self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#west == West::Low
        {
            return 17832;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#east == East::Tall
        {
            return 17872;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 17753;
        }
        if self.r#up == false
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Low
        {
            return 17790;
        }
        if self.r#north == North::Tall
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#waterlogged == true
            && self.r#east == East::Low
        {
            return 17794;
        }
        if self.r#up == false
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
        {
            return 17668;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#south == South::None
        {
            return 17877;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#north == North::Tall
        {
            return 17673;
        }
        if self.r#up == true
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#waterlogged == true
        {
            return 17853;
        }
        if self.r#north == North::Low
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 17653;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 17905;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 17808;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#east == East::None
        {
            return 17644;
        }
        if self.r#south == South::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 17630;
        }
        if self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#south == South::Low
            && self.r#west == West::Low
        {
            return 17679;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::None
        {
            return 17828;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#waterlogged == false
            && self.r#south == South::Low
        {
            return 17862;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#south == South::Tall
        {
            return 17873;
        }
        if self.r#west == West::Low
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 17826;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#west == West::None
        {
            return 17843;
        }
        if self.r#up == true
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#waterlogged == false
            && self.r#east == East::Low
        {
            return 17699;
        }
        if self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == West::None
        {
            return 17765;
        }
        if self.r#waterlogged == true
            && self.r#up == true
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#north == North::Tall
        {
            return 17661;
        }
        if self.r#up == false
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#east == East::Low
        {
            return 17787;
        }
        if self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#waterlogged == false
        {
            return 17796;
        }
        if self.r#west == West::Low
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 17844;
        }
        if self.r#up == false
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#south == South::Tall
        {
            return 17909;
        }
        if self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == North::None
        {
            return 17610;
        }
        if self.r#north == North::Low
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 17852;
        }
        if self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#east == East::None
        {
            return 17613;
        }
        if self.r#north == North::Low
            && self.r#west == West::None
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == South::Low
        {
            return 17744;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 17746;
        }
        if self.r#east == East::Low
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 17743;
        }
        if self.r#west == West::None
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#up == true
            && self.r#south == South::Low
            && self.r#waterlogged == false
        {
            return 17747;
        }
        if self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#up == false
            && self.r#south == South::Low
        {
            return 17715;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#east == East::Low
        {
            return 17799;
        }
        if self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#up == false
            && self.r#west == West::Low
        {
            return 17847;
        }
        if self.r#waterlogged == true
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#north == North::Low
        {
            return 17871;
        }
        if self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 17599;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 17601;
        }
        if self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 17737;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::Tall
        {
            return 17685;
        }
        if self.r#waterlogged == false
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#west == West::Low
            && self.r#north == North::None
        {
            return 17724;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::Low
        {
            return 17700;
        }
        if self.r#east == East::None
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 17588;
        }
        if self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#waterlogged == true
        {
            return 17722;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Tall
        {
            return 17836;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#up == true
        {
            return 17880;
        }
        if self.r#waterlogged == false
            && self.r#south == South::None
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Tall
        {
            return 17881;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Low
        {
            return 17890;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
        {
            return 17895;
        }
        if self.r#west == West::Low
            && self.r#north == North::None
            && self.r#up == false
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == false
        {
            return 17838;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#south == South::None
        {
            return 17769;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#west == West::Low
        {
            return 17709;
        }
        if self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#up == true
        {
            return 17617;
        }
        if self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 17814;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
        {
            return 17614;
        }
        if self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::Low
        {
            return 17701;
        }
        if self.r#east == East::Tall
            && self.r#up == false
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::Low
            && self.r#west == West::Low
        {
            return 17850;
        }
        if self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#south == South::Low
        {
            return 17854;
        }
        if self.r#south == South::None
            && self.r#up == true
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 17732;
        }
        if self.r#west == West::Tall
            && self.r#up == true
            && self.r#south == South::None
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::Tall
        {
            return 17662;
        }
        if self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 17691;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == true
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 17869;
        }
        if self.r#north == North::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#up == true
        {
            return 17902;
        }
        if self.r#waterlogged == true
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == true
        {
            return 17602;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 17677;
        }
        if self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#up == true
            && self.r#east == East::None
            && self.r#west == West::None
            && self.r#waterlogged == true
        {
            return 17612;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#west == West::Tall
            && self.r#waterlogged == true
        {
            return 17764;
        }
        if self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 17816;
        }
        if self.r#up == true
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#west == West::Low
        {
            return 17865;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#east == East::Low
            && self.r#south == South::Tall
        {
            return 17800;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::Low
            && self.r#east == East::Tall
        {
            return 17907;
        }
        if self.r#up == false
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#waterlogged == false
            && self.r#north == North::None
        {
            return 17815;
        }
        if self.r#west == West::None
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#south == South::Tall
        {
            return 17684;
        }
        if self.r#north == North::Low
            && self.r#up == true
            && self.r#east == East::None
            && self.r#west == West::Low
            && self.r#waterlogged == true
            && self.r#south == South::Low
        {
            return 17637;
        }
        if self.r#east == East::Low
            && self.r#up == true
            && self.r#south == South::Tall
            && self.r#waterlogged == true
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 17720;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 17749;
        }
        if self.r#north == North::Low
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#south == South::Tall
        {
            return 17759;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == true
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#up == false
            && self.r#north == North::None
        {
            return 17704;
        }
        if self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::Tall
            && self.r#waterlogged == true
            && self.r#up == true
        {
            return 17830;
        }
        if self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#up == true
            && self.r#south == South::None
        {
            return 17841;
        }
        if self.r#north == North::Low
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#up == false
        {
            return 17643;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#north == North::Low
            && self.r#waterlogged == false
        {
            return 17875;
        }
        if self.r#up == false
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#waterlogged == true
            && self.r#east == East::Tall
            && self.r#south == South::None
        {
            return 17812;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == West::None
        {
            return 17798;
        }
        if self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#east == East::Tall
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 17903;
        }
        if self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#north == North::Tall
            && self.r#up == true
            && self.r#west == West::None
        {
            return 17879;
        }
        if self.r#waterlogged == true
            && self.r#north == North::Low
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#up == true
        {
            return 17756;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#north == North::None
        {
            return 17804;
        }
        if self.r#south == South::Low
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#west == West::None
        {
            return 17678;
        }
        if self.r#waterlogged == true
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#east == East::None
            && self.r#up == true
        {
            return 17649;
        }
        if self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 17656;
        }
        if self.r#waterlogged == false
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#up == true
            && self.r#west == West::Low
        {
            return 17772;
        }
        if self.r#south == South::Low
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 17782;
        }
        if self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#up == false
            && self.r#west == West::Tall
        {
            return 17707;
        }
        if self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == South::Tall
        {
            return 17658;
        }
        if self.r#east == East::None
            && self.r#south == South::Low
            && self.r#up == false
            && self.r#north == North::Tall
            && self.r#waterlogged == false
            && self.r#west == West::Tall
        {
            return 17683;
        }
        if self.r#south == South::None
            && self.r#waterlogged == false
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#up == false
            && self.r#north == North::None
        {
            return 17598;
        }
        if self.r#up == false
            && self.r#east == East::Low
            && self.r#waterlogged == false
            && self.r#west == West::Tall
            && self.r#south == South::Tall
            && self.r#north == North::None
        {
            return 17731;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 17605 {
            return Some(GraniteWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17721 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#west: West::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 17672 {
            return Some(GraniteWall {
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 17636 {
            return Some(GraniteWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17863 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17876 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17666 {
            return Some(GraniteWall {
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 17910 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 17616 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 17762 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 17595 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17624 {
            return Some(GraniteWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 17745 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 17681 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17625 {
            return Some(GraniteWall {
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17748 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 17650 {
            return Some(GraniteWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17593 {
            return Some(GraniteWall {
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 17633 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#west: West::None,
                r#up: false,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17682 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17784 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 17632 {
            return Some(GraniteWall {
                r#up: false,
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 17791 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 17600 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 17864 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 17887 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17778 {
            return Some(GraniteWall {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17623 {
            return Some(GraniteWall {
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17651 {
            return Some(GraniteWall {
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17878 {
            return Some(GraniteWall {
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17851 {
            return Some(GraniteWall {
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17870 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 17594 {
            return Some(GraniteWall {
                r#up: false,
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17710 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 17752 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 17894 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17735 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 17891 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17793 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17848 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17734 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 17823 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 17893 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 17694 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 17711 {
            return Some(GraniteWall {
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17750 {
            return Some(GraniteWall {
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17642 {
            return Some(GraniteWall {
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 17695 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 17717 {
            return Some(GraniteWall {
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 17736 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 17652 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 17827 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 17885 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 17655 {
            return Some(GraniteWall {
                r#up: false,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 17618 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 17775 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 17849 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17882 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 17723 {
            return Some(GraniteWall {
                r#up: true,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 17766 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17886 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17635 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 17779 {
            return Some(GraniteWall {
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17892 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17817 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17818 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 17647 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 17781 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 17620 {
            return Some(GraniteWall {
                r#north: North::None,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17837 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 17654 {
            return Some(GraniteWall {
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17866 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17619 {
            return Some(GraniteWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 17639 {
            return Some(GraniteWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 17867 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 17592 {
            return Some(GraniteWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 17758 {
            return Some(GraniteWall {
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17908 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17777 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17611 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17669 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 17627 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 17590 {
            return Some(GraniteWall {
                r#up: true,
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17842 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 17631 {
            return Some(GraniteWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 17896 {
            return Some(GraniteWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17702 {
            return Some(GraniteWall {
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 17608 {
            return Some(GraniteWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17645 {
            return Some(GraniteWall {
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17718 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 17755 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17889 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 17714 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 17740 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 17688 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17771 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17900 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17680 {
            return Some(GraniteWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17846 {
            return Some(GraniteWall {
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17597 {
            return Some(GraniteWall {
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17785 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17675 {
            return Some(GraniteWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 17641 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17603 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17686 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17811 {
            return Some(GraniteWall {
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17689 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17706 {
            return Some(GraniteWall {
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17628 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 17840 {
            return Some(GraniteWall {
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 17708 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17729 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17742 {
            return Some(GraniteWall {
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 17697 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 17883 {
            return Some(GraniteWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 17901 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17860 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 17705 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 17713 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 17805 {
            return Some(GraniteWall {
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 17813 {
            return Some(GraniteWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 17819 {
            return Some(GraniteWall {
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17660 {
            return Some(GraniteWall {
                r#west: West::None,
                r#up: true,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17833 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17671 {
            return Some(GraniteWall {
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17698 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 17835 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 17690 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17820 {
            return Some(GraniteWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17829 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 17774 {
            return Some(GraniteWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 17802 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17831 {
            return Some(GraniteWall {
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 17638 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17773 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17767 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17604 {
            return Some(GraniteWall {
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 17825 {
            return Some(GraniteWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 17845 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 17606 {
            return Some(GraniteWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 17911 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 17589 {
            return Some(GraniteWall {
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 17712 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 17795 {
            return Some(GraniteWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17727 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17607 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17640 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 17839 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17859 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17763 {
            return Some(GraniteWall {
                r#up: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 17801 {
            return Some(GraniteWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 17674 {
            return Some(GraniteWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17629 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 17719 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 17897 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17807 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 17687 {
            return Some(GraniteWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 17776 {
            return Some(GraniteWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 17648 {
            return Some(GraniteWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17810 {
            return Some(GraniteWall {
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17622 {
            return Some(GraniteWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17703 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 17770 {
            return Some(GraniteWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 17824 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17868 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17730 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 17858 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17821 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 17857 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17780 {
            return Some(GraniteWall {
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 17884 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17696 {
            return Some(GraniteWall {
                r#west: West::None,
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17751 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 17664 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 17754 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 17757 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17609 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 17739 {
            return Some(GraniteWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17728 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 17768 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 17904 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 17621 {
            return Some(GraniteWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17657 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17663 {
            return Some(GraniteWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17725 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 17716 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 17797 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17861 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 17667 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17806 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 17888 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17726 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 17822 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 17906 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17665 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 17899 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17834 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 17626 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 17646 {
            return Some(GraniteWall {
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 17591 {
            return Some(GraniteWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17760 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 17615 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 17783 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17855 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 17856 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17741 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17761 {
            return Some(GraniteWall {
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17874 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17803 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 17786 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 17692 {
            return Some(GraniteWall {
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 17738 {
            return Some(GraniteWall {
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17809 {
            return Some(GraniteWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 17898 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 17789 {
            return Some(GraniteWall {
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17659 {
            return Some(GraniteWall {
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17596 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 17693 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17792 {
            return Some(GraniteWall {
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 17670 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17676 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17733 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17788 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17634 {
            return Some(GraniteWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17832 {
            return Some(GraniteWall {
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 17872 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 17753 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17790 {
            return Some(GraniteWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 17794 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 17668 {
            return Some(GraniteWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17877 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17673 {
            return Some(GraniteWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17853 {
            return Some(GraniteWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17653 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17905 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17808 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17644 {
            return Some(GraniteWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 17630 {
            return Some(GraniteWall {
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17679 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 17828 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17862 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 17873 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17826 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17843 {
            return Some(GraniteWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 17699 {
            return Some(GraniteWall {
                r#up: true,
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 17765 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17661 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17787 {
            return Some(GraniteWall {
                r#up: false,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17796 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17844 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17909 {
            return Some(GraniteWall {
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17610 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 17852 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17613 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 17744 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 17746 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17743 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17747 {
            return Some(GraniteWall {
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17715 {
            return Some(GraniteWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 17799 {
            return Some(GraniteWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 17847 {
            return Some(GraniteWall {
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 17871 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17599 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17601 {
            return Some(GraniteWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17737 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 17685 {
            return Some(GraniteWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17724 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 17700 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 17588 {
            return Some(GraniteWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 17722 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17836 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17880 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17881 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17890 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 17895 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17838 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17769 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17709 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 17617 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17814 {
            return Some(GraniteWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17614 {
            return Some(GraniteWall {
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17701 {
            return Some(GraniteWall {
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 17850 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 17854 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17732 {
            return Some(GraniteWall {
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17662 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17691 {
            return Some(GraniteWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17869 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17902 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 17602 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 17677 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17612 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17764 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17816 {
            return Some(GraniteWall {
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17865 {
            return Some(GraniteWall {
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17800 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17907 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17815 {
            return Some(GraniteWall {
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 17684 {
            return Some(GraniteWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 17637 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 17720 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 17749 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17759 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17704 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 17830 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 17841 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 17643 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17875 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17812 {
            return Some(GraniteWall {
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17798 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17903 {
            return Some(GraniteWall {
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17879 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 17756 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 17804 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 17678 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 17649 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 17656 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17772 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 17782 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17707 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17658 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17683 {
            return Some(GraniteWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17598 {
            return Some(GraniteWall {
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 17731 {
            return Some(GraniteWall {
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        return None;
    }
}
