use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleMossCarpet {
    pub r#east: East,
    pub bottom: bool,
    pub r#south: South,
    pub r#west: West,
    pub r#north: North,
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

impl BlockState for PaleMossCarpet {
    fn to_id(&self) -> i32 {
        if self.r#east == East::Low
            && self.r#west == West::None
            && self.r#bottom == true
            && self.r#south == South::Tall
            && self.r#north == North::Tall
        {
            return 29553;
        }
        if self.r#north == North::None
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#bottom == true
            && self.r#west == West::Low
        {
            return 29509;
        }
        if self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#bottom == false
            && self.r#east == East::Tall
            && self.r#north == North::None
        {
            return 29645;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#bottom == false
            && self.r#south == South::Tall
            && self.r#west == West::None
        {
            return 29625;
        }
        if self.r#north == North::Low
            && self.r#bottom == true
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#west == West::None
        {
            return 29571;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#bottom == false
        {
            return 29649;
        }
        if self.r#bottom == true
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#west == West::Low
            && self.r#east == East::Low
        {
            return 29548;
        }
        if self.r#east == East::Low
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#bottom == false
            && self.r#south == South::None
        {
            return 29621;
        }
        if self.r#bottom == true
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::Tall
        {
            return 29558;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#bottom == false
            && self.r#west == West::Tall
        {
            return 29633;
        }
        if self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#bottom == true
            && self.r#north == North::Low
            && self.r#west == West::Tall
        {
            return 29543;
        }
        if self.r#west == West::None
            && self.r#east == East::Low
            && self.r#bottom == true
            && self.r#north == North::Low
            && self.r#south == South::None
        {
            return 29538;
        }
        if self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#bottom == true
            && self.r#south == South::Tall
        {
            return 29573;
        }
        if self.r#bottom == false
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#west == West::None
        {
            return 29631;
        }
        if self.r#bottom == true
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#south == South::None
        {
            return 29539;
        }
        if self.r#east == East::Tall
            && self.r#bottom == false
            && self.r#west == West::Low
            && self.r#south == South::Tall
            && self.r#north == North::None
        {
            return 29644;
        }
        if self.r#west == West::Tall
            && self.r#bottom == true
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#east == East::Tall
        {
            return 29561;
        }
        if self.r#west == West::Low
            && self.r#bottom == true
            && self.r#south == South::Tall
            && self.r#north == North::Low
            && self.r#east == East::Low
        {
            return 29545;
        }
        if self.r#east == East::None
            && self.r#bottom == true
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::Tall
        {
            return 29526;
        }
        if self.r#bottom == true
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#west == West::Low
        {
            return 29575;
        }
        if self.r#bottom == false
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#south == South::None
        {
            return 29602;
        }
        if self.r#east == East::Tall
            && self.r#bottom == false
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#west == West::Low
        {
            return 29650;
        }
        if self.r#south == South::Low
            && self.r#west == West::None
            && self.r#bottom == true
            && self.r#east == East::Low
            && self.r#north == North::Tall
        {
            return 29550;
        }
        if self.r#bottom == false
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 29583;
        }
        if self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#east == East::None
            && self.r#bottom == false
        {
            return 29598;
        }
        if self.r#south == South::Tall
            && self.r#east == East::None
            && self.r#bottom == false
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 29600;
        }
        if self.r#bottom == false
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#west == West::Low
        {
            return 29641;
        }
        if self.r#west == West::Low
            && self.r#south == South::Low
            && self.r#bottom == false
            && self.r#north == North::None
            && self.r#east == East::Low
        {
            return 29614;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#bottom == false
            && self.r#west == West::Low
        {
            return 29656;
        }
        if self.r#east == East::None
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#north == North::None
            && self.r#bottom == true
        {
            return 29505;
        }
        if self.r#east == East::None
            && self.r#bottom == true
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::Tall
        {
            return 29504;
        }
        if self.r#north == North::Tall
            && self.r#bottom == false
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#east == East::Tall
        {
            return 29661;
        }
        if self.r#bottom == true
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#east == East::Low
        {
            return 29537;
        }
        if self.r#west == West::Low
            && self.r#bottom == false
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#east == East::Tall
        {
            return 29659;
        }
        if self.r#west == West::Tall
            && self.r#bottom == true
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Tall
        {
            return 29582;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#bottom == false
        {
            return 29609;
        }
        if self.r#north == North::Tall
            && self.r#bottom == true
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#east == East::Tall
        {
            return 29579;
        }
        if self.r#south == South::None
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#west == West::None
            && self.r#bottom == true
        {
            return 29547;
        }
        if self.r#bottom == true
            && self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#north == North::Low
        {
            return 29570;
        }
        if self.r#north == North::None
            && self.r#south == South::Low
            && self.r#bottom == false
            && self.r#east == East::None
            && self.r#west == West::None
        {
            return 29586;
        }
        if self.r#south == South::Low
            && self.r#bottom == false
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#east == East::Low
        {
            return 29613;
        }
        if self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#bottom == true
        {
            return 29567;
        }
        if self.r#west == West::None
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#bottom == false
        {
            return 29619;
        }
        if self.r#bottom == true
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#west == West::Low
            && self.r#east == East::None
        {
            return 29512;
        }
        if self.r#bottom == false
            && self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#north == North::Tall
            && self.r#east == East::Low
        {
            return 29635;
        }
        if self.r#south == South::Tall
            && self.r#bottom == true
            && self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#north == North::Tall
        {
            return 29580;
        }
        if self.r#north == North::None
            && self.r#bottom == false
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#east == East::Tall
        {
            return 29642;
        }
        if self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#bottom == false
        {
            return 29647;
        }
        if self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#bottom == false
        {
            return 29606;
        }
        if self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#east == East::Low
            && self.r#bottom == false
            && self.r#north == North::None
        {
            return 29615;
        }
        if self.r#west == West::Tall
            && self.r#bottom == true
            && self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#east == East::None
        {
            return 29525;
        }
        if self.r#west == West::Tall
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#bottom == true
        {
            return 29576;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#bottom == true
            && self.r#south == South::Tall
        {
            return 29581;
        }
        if self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#bottom == false
            && self.r#north == North::Low
        {
            return 29652;
        }
        if self.r#bottom == true
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#west == West::Tall
        {
            return 29549;
        }
        if self.r#north == North::None
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#east == East::Low
            && self.r#bottom == false
        {
            return 29616;
        }
        if self.r#bottom == true
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#south == South::Low
        {
            return 29551;
        }
        if self.r#east == East::None
            && self.r#south == South::Tall
            && self.r#bottom == false
            && self.r#west == West::Low
            && self.r#north == North::Low
        {
            return 29599;
        }
        if self.r#bottom == true
            && self.r#north == North::None
            && self.r#south == South::Low
            && self.r#east == East::Low
            && self.r#west == West::Tall
        {
            return 29534;
        }
        if self.r#west == West::Tall
            && self.r#bottom == false
            && self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#south == South::Tall
        {
            return 29663;
        }
        if self.r#bottom == true
            && self.r#south == South::None
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#east == East::Low
        {
            return 29540;
        }
        if self.r#south == South::Low
            && self.r#bottom == true
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#east == East::Low
        {
            return 29542;
        }
        if self.r#bottom == false
            && self.r#south == South::Low
            && self.r#north == North::Low
            && self.r#west == West::Tall
            && self.r#east == East::None
        {
            return 29597;
        }
        if self.r#bottom == true
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#west == West::Low
        {
            return 29518;
        }
        if self.r#east == East::None
            && self.r#bottom == false
            && self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#west == West::None
        {
            return 29604;
        }
        if self.r#south == South::Tall
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#bottom == false
            && self.r#east == East::Tall
        {
            return 29662;
        }
        if self.r#south == South::Low
            && self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#bottom == false
            && self.r#east == East::None
        {
            return 29605;
        }
        if self.r#south == South::Tall
            && self.r#bottom == false
            && self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#east == East::Low
        {
            return 29636;
        }
        if self.r#north == North::None
            && self.r#west == West::Low
            && self.r#bottom == true
            && self.r#south == South::Tall
            && self.r#east == East::Low
        {
            return 29536;
        }
        if self.r#bottom == true
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#west == West::Low
        {
            return 29533;
        }
        if self.r#bottom == false
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#west == West::Tall
        {
            return 29639;
        }
        if self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#bottom == false
            && self.r#south == South::Low
        {
            return 29640;
        }
        if self.r#bottom == false
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::Low
        {
            return 29648;
        }
        if self.r#north == North::Low
            && self.r#bottom == false
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::Tall
        {
            return 29653;
        }
        if self.r#bottom == false
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#east == East::Low
        {
            return 29618;
        }
        if self.r#north == North::Tall
            && self.r#bottom == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#south == South::None
        {
            return 29603;
        }
        if self.r#bottom == true
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#south == South::Low
            && self.r#east == East::None
        {
            return 29507;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#bottom == true
            && self.r#south == South::None
            && self.r#west == West::None
        {
            return 29529;
        }
        if self.r#south == South::Tall
            && self.r#west == West::Low
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#bottom == false
        {
            return 29617;
        }
        if self.r#north == North::Tall
            && self.r#bottom == false
            && self.r#east == East::Tall
            && self.r#west == West::None
            && self.r#south == South::None
        {
            return 29655;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Tall
            && self.r#bottom == false
            && self.r#south == South::Tall
        {
            return 29591;
        }
        if self.r#north == North::Tall
            && self.r#bottom == true
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#east == East::None
        {
            return 29528;
        }
        if self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#bottom == true
        {
            return 29572;
        }
        if self.r#north == North::None
            && self.r#bottom == true
            && self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::None
        {
            return 29557;
        }
        if self.r#east == East::Low
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#south == South::None
            && self.r#bottom == true
        {
            return 29530;
        }
        if self.r#south == South::Tall
            && self.r#bottom == false
            && self.r#east == East::Tall
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 29654;
        }
        if self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#west == West::Tall
            && self.r#bottom == true
            && self.r#south == South::None
        {
            return 29522;
        }
        if self.r#south == South::None
            && self.r#east == East::None
            && self.r#bottom == true
            && self.r#north == North::None
            && self.r#west == West::None
        {
            return 29502;
        }
        if self.r#south == South::None
            && self.r#west == West::Low
            && self.r#north == North::None
            && self.r#bottom == false
            && self.r#east == East::None
        {
            return 29584;
        }
        if self.r#bottom == false
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#west == West::None
        {
            return 29601;
        }
        if self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#north == North::Tall
            && self.r#bottom == false
            && self.r#west == West::Tall
        {
            return 29660;
        }
        if self.r#south == South::Low
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#bottom == true
            && self.r#east == East::Tall
        {
            return 29560;
        }
        if self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#west == West::None
            && self.r#bottom == false
        {
            return 29592;
        }
        if self.r#north == North::Tall
            && self.r#west == West::Tall
            && self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#bottom == false
        {
            return 29657;
        }
        if self.r#north == North::None
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#bottom == true
            && self.r#east == East::Tall
        {
            return 29562;
        }
        if self.r#bottom == false
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#north == North::None
            && self.r#west == West::Tall
        {
            return 29588;
        }
        if self.r#bottom == true
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#east == East::None
        {
            return 29520;
        }
        if self.r#south == South::Tall
            && self.r#bottom == true
            && self.r#east == East::None
            && self.r#north == North::None
            && self.r#west == West::Tall
        {
            return 29510;
        }
        if self.r#west == West::None
            && self.r#bottom == false
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::None
        {
            return 29610;
        }
        if self.r#west == West::Low
            && self.r#bottom == false
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::Tall
        {
            return 29626;
        }
        if self.r#bottom == true
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#north == North::None
            && self.r#east == East::None
        {
            return 29508;
        }
        if self.r#bottom == true
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#west == West::None
        {
            return 29511;
        }
        if self.r#bottom == true
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#west == West::None
        {
            return 29535;
        }
        if self.r#bottom == true
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#north == North::Tall
        {
            return 29555;
        }
        if self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#bottom == false
        {
            return 29630;
        }
        if self.r#bottom == false
            && self.r#west == West::None
            && self.r#south == South::None
            && self.r#north == North::Low
            && self.r#east == East::Tall
        {
            return 29646;
        }
        if self.r#bottom == false
            && self.r#north == North::Low
            && self.r#south == South::None
            && self.r#east == East::None
            && self.r#west == West::Low
        {
            return 29593;
        }
        if self.r#west == West::Low
            && self.r#bottom == false
            && self.r#north == North::Low
            && self.r#east == East::Low
            && self.r#south == South::Low
        {
            return 29623;
        }
        if self.r#east == East::Low
            && self.r#bottom == true
            && self.r#west == West::Tall
            && self.r#north == North::Tall
            && self.r#south == South::Low
        {
            return 29552;
        }
        if self.r#south == South::None
            && self.r#west == West::None
            && self.r#bottom == true
            && self.r#north == North::Low
            && self.r#east == East::Tall
        {
            return 29565;
        }
        if self.r#east == East::None
            && self.r#bottom == false
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#west == West::Low
        {
            return 29596;
        }
        if self.r#bottom == false
            && self.r#north == North::None
            && self.r#east == East::Tall
            && self.r#south == South::Tall
            && self.r#west == West::None
        {
            return 29643;
        }
        if self.r#south == South::Low
            && self.r#bottom == true
            && self.r#east == East::Low
            && self.r#west == West::None
            && self.r#north == North::Low
        {
            return 29541;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#bottom == true
            && self.r#west == West::Low
        {
            return 29554;
        }
        if self.r#bottom == true
            && self.r#west == West::Low
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::Tall
        {
            return 29527;
        }
        if self.r#south == South::None
            && self.r#east == East::Low
            && self.r#west == West::Low
            && self.r#north == North::Low
            && self.r#bottom == false
        {
            return 29620;
        }
        if self.r#east == East::Tall
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#bottom == true
            && self.r#north == North::Low
        {
            return 29568;
        }
        if self.r#bottom == true
            && self.r#south == South::None
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#west == West::Tall
        {
            return 29531;
        }
        if self.r#north == North::Tall
            && self.r#south == South::Low
            && self.r#bottom == true
            && self.r#east == East::Tall
            && self.r#west == West::None
        {
            return 29577;
        }
        if self.r#bottom == false
            && self.r#west == West::None
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#south == South::Tall
        {
            return 29607;
        }
        if self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#bottom == true
            && self.r#east == East::None
            && self.r#north == North::Tall
        {
            return 29524;
        }
        if self.r#bottom == true
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#south == South::None
        {
            return 29513;
        }
        if self.r#west == West::Low
            && self.r#bottom == false
            && self.r#north == North::Tall
            && self.r#east == East::None
            && self.r#south == South::Tall
        {
            return 29608;
        }
        if self.r#north == North::Low
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#bottom == true
        {
            return 29514;
        }
        if self.r#bottom == true
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#west == West::Low
        {
            return 29569;
        }
        if self.r#north == North::None
            && self.r#west == West::None
            && self.r#bottom == true
            && self.r#east == East::Tall
            && self.r#south == South::None
        {
            return 29556;
        }
        if self.r#bottom == true
            && self.r#west == West::None
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#east == East::None
        {
            return 29517;
        }
        if self.r#south == South::Low
            && self.r#bottom == true
            && self.r#north == North::None
            && self.r#west == West::None
            && self.r#east == East::Low
        {
            return 29532;
        }
        if self.r#north == North::Low
            && self.r#west == West::None
            && self.r#south == South::Tall
            && self.r#bottom == true
            && self.r#east == East::Low
        {
            return 29544;
        }
        if self.r#bottom == true
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#east == East::None
        {
            return 29503;
        }
        if self.r#bottom == false
            && self.r#east == East::Low
            && self.r#north == North::None
            && self.r#south == South::None
            && self.r#west == West::Low
        {
            return 29611;
        }
        if self.r#bottom == true
            && self.r#south == South::Low
            && self.r#east == East::Tall
            && self.r#west == West::Low
            && self.r#north == North::Tall
        {
            return 29578;
        }
        if self.r#west == West::None
            && self.r#bottom == false
            && self.r#east == East::Low
            && self.r#north == North::Tall
            && self.r#south == South::None
        {
            return 29628;
        }
        if self.r#south == South::None
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#bottom == false
            && self.r#west == West::None
        {
            return 29637;
        }
        if self.r#east == East::None
            && self.r#bottom == true
            && self.r#south == South::Low
            && self.r#west == West::Low
            && self.r#north == North::None
        {
            return 29506;
        }
        if self.r#south == South::Tall
            && self.r#bottom == false
            && self.r#north == North::None
            && self.r#east == East::None
            && self.r#west == West::Low
        {
            return 29590;
        }
        if self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#bottom == false
        {
            return 29622;
        }
        if self.r#bottom == false
            && self.r#west == West::Tall
            && self.r#east == East::None
            && self.r#south == South::None
            && self.r#north == North::Low
        {
            return 29594;
        }
        if self.r#bottom == false
            && self.r#east == East::Low
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#west == West::Low
        {
            return 29629;
        }
        if self.r#bottom == false
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#west == West::None
            && self.r#east == East::None
        {
            return 29589;
        }
        if self.r#east == East::None
            && self.r#north == North::None
            && self.r#bottom == false
            && self.r#west == West::Tall
            && self.r#south == South::None
        {
            return 29585;
        }
        if self.r#south == South::Low
            && self.r#east == East::None
            && self.r#bottom == false
            && self.r#north == North::Low
            && self.r#west == West::None
        {
            return 29595;
        }
        if self.r#north == North::Tall
            && self.r#west == West::Low
            && self.r#bottom == false
            && self.r#south == South::Low
            && self.r#east == East::Low
        {
            return 29632;
        }
        if self.r#north == North::Tall
            && self.r#east == East::Tall
            && self.r#bottom == false
            && self.r#south == South::Low
            && self.r#west == West::None
        {
            return 29658;
        }
        if self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#bottom == true
            && self.r#north == North::Low
        {
            return 29566;
        }
        if self.r#north == North::None
            && self.r#bottom == true
            && self.r#south == South::Low
            && self.r#west == West::None
            && self.r#east == East::Tall
        {
            return 29559;
        }
        if self.r#west == West::None
            && self.r#east == East::None
            && self.r#south == South::Low
            && self.r#bottom == true
            && self.r#north == North::Tall
        {
            return 29523;
        }
        if self.r#west == West::None
            && self.r#east == East::Low
            && self.r#south == South::Tall
            && self.r#bottom == false
            && self.r#north == North::Tall
        {
            return 29634;
        }
        if self.r#east == East::None
            && self.r#bottom == false
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#south == South::Low
        {
            return 29587;
        }
        if self.r#south == South::Tall
            && self.r#west == West::Tall
            && self.r#bottom == false
            && self.r#east == East::Low
            && self.r#north == North::Low
        {
            return 29627;
        }
        if self.r#west == West::Low
            && self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#bottom == false
            && self.r#south == South::None
        {
            return 29638;
        }
        if self.r#bottom == true
            && self.r#east == East::Low
            && self.r#north == North::Low
            && self.r#south == South::Tall
            && self.r#west == West::Tall
        {
            return 29546;
        }
        if self.r#west == West::None
            && self.r#east == East::Tall
            && self.r#south == South::None
            && self.r#north == North::Tall
            && self.r#bottom == true
        {
            return 29574;
        }
        if self.r#bottom == true
            && self.r#west == West::Tall
            && self.r#north == North::None
            && self.r#south == South::Tall
            && self.r#east == East::Tall
        {
            return 29564;
        }
        if self.r#bottom == false
            && self.r#east == East::Low
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#north == North::Low
        {
            return 29624;
        }
        if self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#bottom == true
            && self.r#east == East::None
        {
            return 29516;
        }
        if self.r#south == South::Low
            && self.r#west == West::Tall
            && self.r#north == North::Low
            && self.r#east == East::Tall
            && self.r#bottom == false
        {
            return 29651;
        }
        if self.r#bottom == false
            && self.r#south == South::None
            && self.r#north == North::None
            && self.r#east == East::Low
            && self.r#west == West::Tall
        {
            return 29612;
        }
        if self.r#south == South::Tall
            && self.r#bottom == true
            && self.r#east == East::None
            && self.r#north == North::Low
            && self.r#west == West::Tall
        {
            return 29519;
        }
        if self.r#east == East::Tall
            && self.r#north == North::None
            && self.r#west == West::Low
            && self.r#bottom == true
            && self.r#south == South::Tall
        {
            return 29563;
        }
        if self.r#bottom == true
            && self.r#east == East::None
            && self.r#north == North::Tall
            && self.r#south == South::None
            && self.r#west == West::Low
        {
            return 29521;
        }
        if self.r#bottom == true
            && self.r#north == North::Low
            && self.r#south == South::Low
            && self.r#east == East::None
            && self.r#west == West::Low
        {
            return 29515;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29553 {
            return Some(PaleMossCarpet {
                r#east: East::Low,
                r#west: West::None,
                r#bottom: true,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 29509 {
            return Some(PaleMossCarpet {
                r#north: North::None,
                r#east: East::None,
                r#south: South::Tall,
                r#bottom: true,
                r#west: West::Low,
            });
        }
        if state_id == 29645 {
            return Some(PaleMossCarpet {
                r#south: South::Tall,
                r#west: West::Tall,
                r#bottom: false,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 29625 {
            return Some(PaleMossCarpet {
                r#east: East::Low,
                r#north: North::Low,
                r#bottom: false,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 29571 {
            return Some(PaleMossCarpet {
                r#north: North::Low,
                r#bottom: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 29649 {
            return Some(PaleMossCarpet {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::None,
                r#bottom: false,
            });
        }
        if state_id == 29548 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 29621 {
            return Some(PaleMossCarpet {
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Low,
                r#bottom: false,
                r#south: South::None,
            });
        }
        if state_id == 29558 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 29633 {
            return Some(PaleMossCarpet {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#bottom: false,
                r#west: West::Tall,
            });
        }
        if state_id == 29543 {
            return Some(PaleMossCarpet {
                r#south: South::Low,
                r#east: East::Low,
                r#bottom: true,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 29538 {
            return Some(PaleMossCarpet {
                r#west: West::None,
                r#east: East::Low,
                r#bottom: true,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 29573 {
            return Some(PaleMossCarpet {
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Tall,
                r#bottom: true,
                r#south: South::Tall,
            });
        }
        if state_id == 29631 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 29539 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 29644 {
            return Some(PaleMossCarpet {
                r#east: East::Tall,
                r#bottom: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 29561 {
            return Some(PaleMossCarpet {
                r#west: West::Tall,
                r#bottom: true,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 29545 {
            return Some(PaleMossCarpet {
                r#west: West::Low,
                r#bottom: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 29526 {
            return Some(PaleMossCarpet {
                r#east: East::None,
                r#bottom: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 29575 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 29602 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 29650 {
            return Some(PaleMossCarpet {
                r#east: East::Tall,
                r#bottom: false,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 29550 {
            return Some(PaleMossCarpet {
                r#south: South::Low,
                r#west: West::None,
                r#bottom: true,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 29583 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 29598 {
            return Some(PaleMossCarpet {
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::None,
                r#bottom: false,
            });
        }
        if state_id == 29600 {
            return Some(PaleMossCarpet {
                r#south: South::Tall,
                r#east: East::None,
                r#bottom: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 29641 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 29614 {
            return Some(PaleMossCarpet {
                r#west: West::Low,
                r#south: South::Low,
                r#bottom: false,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 29656 {
            return Some(PaleMossCarpet {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#bottom: false,
                r#west: West::Low,
            });
        }
        if state_id == 29505 {
            return Some(PaleMossCarpet {
                r#east: East::None,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::None,
                r#bottom: true,
            });
        }
        if state_id == 29504 {
            return Some(PaleMossCarpet {
                r#east: East::None,
                r#bottom: true,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 29661 {
            return Some(PaleMossCarpet {
                r#north: North::Tall,
                r#bottom: false,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 29537 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 29659 {
            return Some(PaleMossCarpet {
                r#west: West::Low,
                r#bottom: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 29582 {
            return Some(PaleMossCarpet {
                r#west: West::Tall,
                r#bottom: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 29609 {
            return Some(PaleMossCarpet {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#bottom: false,
            });
        }
        if state_id == 29579 {
            return Some(PaleMossCarpet {
                r#north: North::Tall,
                r#bottom: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 29547 {
            return Some(PaleMossCarpet {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::None,
                r#bottom: true,
            });
        }
        if state_id == 29570 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29586 {
            return Some(PaleMossCarpet {
                r#north: North::None,
                r#south: South::Low,
                r#bottom: false,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 29613 {
            return Some(PaleMossCarpet {
                r#south: South::Low,
                r#bottom: false,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 29567 {
            return Some(PaleMossCarpet {
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Low,
                r#bottom: true,
            });
        }
        if state_id == 29619 {
            return Some(PaleMossCarpet {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
                r#bottom: false,
            });
        }
        if state_id == 29512 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 29635 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 29580 {
            return Some(PaleMossCarpet {
                r#south: South::Tall,
                r#bottom: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 29642 {
            return Some(PaleMossCarpet {
                r#north: North::None,
                r#bottom: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 29647 {
            return Some(PaleMossCarpet {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#bottom: false,
            });
        }
        if state_id == 29606 {
            return Some(PaleMossCarpet {
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#bottom: false,
            });
        }
        if state_id == 29615 {
            return Some(PaleMossCarpet {
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Low,
                r#bottom: false,
                r#north: North::None,
            });
        }
        if state_id == 29525 {
            return Some(PaleMossCarpet {
                r#west: West::Tall,
                r#bottom: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 29576 {
            return Some(PaleMossCarpet {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#bottom: true,
            });
        }
        if state_id == 29581 {
            return Some(PaleMossCarpet {
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#bottom: true,
                r#south: South::Tall,
            });
        }
        if state_id == 29652 {
            return Some(PaleMossCarpet {
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#bottom: false,
                r#north: North::Low,
            });
        }
        if state_id == 29549 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 29616 {
            return Some(PaleMossCarpet {
                r#north: North::None,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#bottom: false,
            });
        }
        if state_id == 29551 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 29599 {
            return Some(PaleMossCarpet {
                r#east: East::None,
                r#south: South::Tall,
                r#bottom: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29534 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 29663 {
            return Some(PaleMossCarpet {
                r#west: West::Tall,
                r#bottom: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 29540 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 29542 {
            return Some(PaleMossCarpet {
                r#south: South::Low,
                r#bottom: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 29597 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 29518 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 29604 {
            return Some(PaleMossCarpet {
                r#east: East::None,
                r#bottom: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 29662 {
            return Some(PaleMossCarpet {
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Low,
                r#bottom: false,
                r#east: East::Tall,
            });
        }
        if state_id == 29605 {
            return Some(PaleMossCarpet {
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#bottom: false,
                r#east: East::None,
            });
        }
        if state_id == 29636 {
            return Some(PaleMossCarpet {
                r#south: South::Tall,
                r#bottom: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 29536 {
            return Some(PaleMossCarpet {
                r#north: North::None,
                r#west: West::Low,
                r#bottom: true,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 29533 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 29639 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 29640 {
            return Some(PaleMossCarpet {
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::None,
                r#bottom: false,
                r#south: South::Low,
            });
        }
        if state_id == 29648 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 29653 {
            return Some(PaleMossCarpet {
                r#north: North::Low,
                r#bottom: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 29618 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 29603 {
            return Some(PaleMossCarpet {
                r#north: North::Tall,
                r#bottom: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 29507 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 29529 {
            return Some(PaleMossCarpet {
                r#east: East::Low,
                r#north: North::None,
                r#bottom: true,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 29617 {
            return Some(PaleMossCarpet {
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#bottom: false,
            });
        }
        if state_id == 29655 {
            return Some(PaleMossCarpet {
                r#north: North::Tall,
                r#bottom: false,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 29591 {
            return Some(PaleMossCarpet {
                r#east: East::None,
                r#north: North::None,
                r#west: West::Tall,
                r#bottom: false,
                r#south: South::Tall,
            });
        }
        if state_id == 29528 {
            return Some(PaleMossCarpet {
                r#north: North::Tall,
                r#bottom: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 29572 {
            return Some(PaleMossCarpet {
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#bottom: true,
            });
        }
        if state_id == 29557 {
            return Some(PaleMossCarpet {
                r#north: North::None,
                r#bottom: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 29530 {
            return Some(PaleMossCarpet {
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::None,
                r#bottom: true,
            });
        }
        if state_id == 29654 {
            return Some(PaleMossCarpet {
                r#south: South::Tall,
                r#bottom: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 29522 {
            return Some(PaleMossCarpet {
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::Tall,
                r#bottom: true,
                r#south: South::None,
            });
        }
        if state_id == 29502 {
            return Some(PaleMossCarpet {
                r#south: South::None,
                r#east: East::None,
                r#bottom: true,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 29584 {
            return Some(PaleMossCarpet {
                r#south: South::None,
                r#west: West::Low,
                r#north: North::None,
                r#bottom: false,
                r#east: East::None,
            });
        }
        if state_id == 29601 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 29660 {
            return Some(PaleMossCarpet {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#bottom: false,
                r#west: West::Tall,
            });
        }
        if state_id == 29560 {
            return Some(PaleMossCarpet {
                r#south: South::Low,
                r#north: North::None,
                r#west: West::Low,
                r#bottom: true,
                r#east: East::Tall,
            });
        }
        if state_id == 29592 {
            return Some(PaleMossCarpet {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::None,
                r#bottom: false,
            });
        }
        if state_id == 29657 {
            return Some(PaleMossCarpet {
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#bottom: false,
            });
        }
        if state_id == 29562 {
            return Some(PaleMossCarpet {
                r#north: North::None,
                r#west: West::None,
                r#south: South::Tall,
                r#bottom: true,
                r#east: East::Tall,
            });
        }
        if state_id == 29588 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 29520 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 29510 {
            return Some(PaleMossCarpet {
                r#south: South::Tall,
                r#bottom: true,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 29610 {
            return Some(PaleMossCarpet {
                r#west: West::None,
                r#bottom: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 29626 {
            return Some(PaleMossCarpet {
                r#west: West::Low,
                r#bottom: false,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 29508 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 29511 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 29535 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 29555 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 29630 {
            return Some(PaleMossCarpet {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#bottom: false,
            });
        }
        if state_id == 29646 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 29593 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 29623 {
            return Some(PaleMossCarpet {
                r#west: West::Low,
                r#bottom: false,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 29552 {
            return Some(PaleMossCarpet {
                r#east: East::Low,
                r#bottom: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 29565 {
            return Some(PaleMossCarpet {
                r#south: South::None,
                r#west: West::None,
                r#bottom: true,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 29596 {
            return Some(PaleMossCarpet {
                r#east: East::None,
                r#bottom: false,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 29643 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 29541 {
            return Some(PaleMossCarpet {
                r#south: South::Low,
                r#bottom: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 29554 {
            return Some(PaleMossCarpet {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#bottom: true,
                r#west: West::Low,
            });
        }
        if state_id == 29527 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 29620 {
            return Some(PaleMossCarpet {
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#bottom: false,
            });
        }
        if state_id == 29568 {
            return Some(PaleMossCarpet {
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::None,
                r#bottom: true,
                r#north: North::Low,
            });
        }
        if state_id == 29531 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 29577 {
            return Some(PaleMossCarpet {
                r#north: North::Tall,
                r#south: South::Low,
                r#bottom: true,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 29607 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 29524 {
            return Some(PaleMossCarpet {
                r#south: South::Low,
                r#west: West::Low,
                r#bottom: true,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 29513 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 29608 {
            return Some(PaleMossCarpet {
                r#west: West::Low,
                r#bottom: false,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 29514 {
            return Some(PaleMossCarpet {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::None,
                r#bottom: true,
            });
        }
        if state_id == 29569 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 29556 {
            return Some(PaleMossCarpet {
                r#north: North::None,
                r#west: West::None,
                r#bottom: true,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 29517 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 29532 {
            return Some(PaleMossCarpet {
                r#south: South::Low,
                r#bottom: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 29544 {
            return Some(PaleMossCarpet {
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Tall,
                r#bottom: true,
                r#east: East::Low,
            });
        }
        if state_id == 29503 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 29611 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 29578 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 29628 {
            return Some(PaleMossCarpet {
                r#west: West::None,
                r#bottom: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 29637 {
            return Some(PaleMossCarpet {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
                r#bottom: false,
                r#west: West::None,
            });
        }
        if state_id == 29506 {
            return Some(PaleMossCarpet {
                r#east: East::None,
                r#bottom: true,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 29590 {
            return Some(PaleMossCarpet {
                r#south: South::Tall,
                r#bottom: false,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 29622 {
            return Some(PaleMossCarpet {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::None,
                r#bottom: false,
            });
        }
        if state_id == 29594 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 29629 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 29589 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 29585 {
            return Some(PaleMossCarpet {
                r#east: East::None,
                r#north: North::None,
                r#bottom: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 29595 {
            return Some(PaleMossCarpet {
                r#south: South::Low,
                r#east: East::None,
                r#bottom: false,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 29632 {
            return Some(PaleMossCarpet {
                r#north: North::Tall,
                r#west: West::Low,
                r#bottom: false,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 29658 {
            return Some(PaleMossCarpet {
                r#north: North::Tall,
                r#east: East::Tall,
                r#bottom: false,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 29566 {
            return Some(PaleMossCarpet {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#bottom: true,
                r#north: North::Low,
            });
        }
        if state_id == 29559 {
            return Some(PaleMossCarpet {
                r#north: North::None,
                r#bottom: true,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 29523 {
            return Some(PaleMossCarpet {
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#bottom: true,
                r#north: North::Tall,
            });
        }
        if state_id == 29634 {
            return Some(PaleMossCarpet {
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#bottom: false,
                r#north: North::Tall,
            });
        }
        if state_id == 29587 {
            return Some(PaleMossCarpet {
                r#east: East::None,
                r#bottom: false,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 29627 {
            return Some(PaleMossCarpet {
                r#south: South::Tall,
                r#west: West::Tall,
                r#bottom: false,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29638 {
            return Some(PaleMossCarpet {
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#bottom: false,
                r#south: South::None,
            });
        }
        if state_id == 29546 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 29574 {
            return Some(PaleMossCarpet {
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#bottom: true,
            });
        }
        if state_id == 29564 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 29624 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 29516 {
            return Some(PaleMossCarpet {
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#bottom: true,
                r#east: East::None,
            });
        }
        if state_id == 29651 {
            return Some(PaleMossCarpet {
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#bottom: false,
            });
        }
        if state_id == 29612 {
            return Some(PaleMossCarpet {
                r#bottom: false,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 29519 {
            return Some(PaleMossCarpet {
                r#south: South::Tall,
                r#bottom: true,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 29563 {
            return Some(PaleMossCarpet {
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Low,
                r#bottom: true,
                r#south: South::Tall,
            });
        }
        if state_id == 29521 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 29515 {
            return Some(PaleMossCarpet {
                r#bottom: true,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        return None;
    }
}
