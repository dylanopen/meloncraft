use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedTuffWall {
    pub r#east: East,
    pub waterlogged: bool,
    pub r#north: North,
    pub up: bool,
    pub r#west: West,
    pub r#south: South,
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

impl BlockState for PolishedTuffWall {
    fn to_id(&self) -> i32 {
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::Tall && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true { return 23954; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#up == false { return 23912; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::Tall { return 24067; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::None { return 23853; }
        if self.r#west == West::Tall && self.r#up == true && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == true { return 23930; }
        if self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Tall && self.r#south == South::None { return 23972; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::None && self.r#east == East::Tall && self.r#up == false && self.r#north == North::None { return 23970; }
        if self.r#up == true && self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::Low && self.r#east == East::None && self.r#west == West::Tall { return 23786; }
        if self.r#south == South::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Tall && self.r#east == East::Low && self.r#up == false { return 23948; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::Low && self.r#up == false { return 24032; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::None && self.r#south == South::Tall && self.r#up == false && self.r#west == West::Low { return 23782; }
        if self.r#up == false && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Low { return 23794; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::Low && self.r#north == North::Low { return 23926; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::Low && self.r#up == true && self.r#west == West::Tall { return 23798; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#west == West::Low && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == false { return 24040; }
        if self.r#east == East::None && self.r#south == South::None && self.r#up == true && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Low { return 23749; }
        if self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low { return 23812; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::None && self.r#north == North::None && self.r#up == true { return 23764; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#up == false && self.r#west == West::None && self.r#south == South::Low && self.r#waterlogged == true { return 23946; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#up == true && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::Tall { return 23941; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::None && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false { return 23975; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::None && self.r#up == false && self.r#north == North::Tall && self.r#south == South::Low { return 23843; }
        if self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::None && self.r#east == East::Low { return 23916; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#up == false && self.r#west == West::Tall && self.r#north == North::None && self.r#waterlogged == true { return 23780; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Low { return 23805; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::None { return 23867; }
        if self.r#south == South::Low && self.r#west == West::Low && self.r#east == East::None && self.r#up == true && self.r#north == North::Low && self.r#waterlogged == true { return 23797; }
        if self.r#up == false && self.r#west == West::None && self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::Low && self.r#south == South::Low { return 23874; }
        if self.r#up == false && self.r#north == North::Tall && self.r#east == East::Low && self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::Low { return 23947; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == false && self.r#west == West::Low { return 23815; }
        if self.r#north == North::None && self.r#up == false && self.r#east == East::Tall && self.r#west == West::Tall && self.r#waterlogged == false && self.r#south == South::Low { return 23987; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#up == true && self.r#east == East::Low && self.r#south == South::Tall { return 23956; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::Low && self.r#up == false && self.r#south == South::Low && self.r#west == West::None { return 23913; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::Low && self.r#up == true && self.r#north == North::Low { return 23905; }
        if self.r#up == true && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::None && self.r#south == South::None && self.r#east == East::Tall { return 23964; }
        if self.r#south == South::None && self.r#north == North::Low && self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall { return 24005; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None && self.r#up == true { return 24003; }
        if self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::Tall { return 24060; }
        if self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::Low && self.r#up == false && self.r#south == South::Low { return 23877; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false { return 23771; }
        if self.r#west == West::None && self.r#east == East::None && self.r#north == North::None && self.r#up == false && self.r#south == South::None && self.r#waterlogged == true { return 23754; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#up == true && self.r#south == South::None && self.r#west == West::Tall && self.r#waterlogged == false { return 23825; }
        if self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::None { return 23969; }
        if self.r#south == South::Tall && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Tall { return 23924; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::None { return 24045; }
        if self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#up == true && self.r#north == North::None && self.r#west == West::Tall { return 23858; }
        if self.r#west == West::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::None && self.r#up == true { return 23788; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#up == true && self.r#east == East::None && self.r#north == North::None && self.r#south == South::Tall { return 23776; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None { return 23823; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::None && self.r#up == false { return 23866; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::None && self.r#north == North::Low && self.r#up == true { return 23896; }
        if self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::None && self.r#up == false && self.r#north == North::Low { return 23790; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::Low && self.r#up == true && self.r#south == South::None { return 24001; }
        if self.r#east == East::Tall && self.r#up == false && self.r#south == South::None && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Tall { return 24011; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#up == false && self.r#north == North::None && self.r#south == South::None && self.r#east == East::None { return 23757; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == true && self.r#north == North::Low { return 24029; }
        if self.r#up == true && self.r#east == East::None && self.r#west == West::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::Low { return 23833; }
        if self.r#up == false && self.r#west == West::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::Low && self.r#east == East::Tall { return 24055; }
        if self.r#east == East::Tall && self.r#up == true && self.r#south == South::None && self.r#west == West::Tall && self.r#waterlogged == true && self.r#north == North::None { return 23966; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Tall && self.r#up == false && self.r#west == West::Low { return 23854; }
        if self.r#up == false && self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::Tall { return 23816; }
        if self.r#east == East::Low && self.r#west == West::Tall && self.r#up == true && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == true { return 23870; }
        if self.r#south == South::None && self.r#up == true && self.r#east == East::Low && self.r#north == North::Low && self.r#west == West::None && self.r#waterlogged == false { return 23895; }
        if self.r#west == West::Low && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::Low { return 23935; }
        if self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Low { return 23873; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#up == false && self.r#west == West::None && self.r#waterlogged == true && self.r#north == North::Tall { return 24054; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == true && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::Tall { return 23952; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Low { return 23968; }
        if self.r#waterlogged == true && self.r#up == false && self.r#south == South::None && self.r#north == North::None && self.r#east == East::Low && self.r#west == West::None { return 23862; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::None { return 23765; }
        if self.r#up == false && self.r#west == West::Tall && self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::None { return 23939; }
        if self.r#west == West::Tall && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false && self.r#north == North::Tall { return 23960; }
        if self.r#south == South::Low && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None { return 23981; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low { return 24023; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Tall { return 24044; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::None && self.r#east == East::Low && self.r#up == false && self.r#south == South::Tall { return 23925; }
        if self.r#north == North::None && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Tall { return 23999; }
        if self.r#east == East::Low && self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::None && self.r#north == North::Low && self.r#up == false { return 23901; }
        if self.r#up == true && self.r#east == East::Low && self.r#west == West::None && self.r#north == North::None && self.r#south == South::Tall && self.r#waterlogged == false { return 23883; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == true && self.r#west == West::None && self.r#east == East::Tall { return 24024; }
        if self.r#east == East::None && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::None { return 23817; }
        if self.r#up == true && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::Tall { return 23849; }
        if self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::Low { return 24017; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::Tall && self.r#west == West::None && self.r#waterlogged == false && self.r#up == true { return 24063; }
        if self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == true && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Low { return 23845; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Low && self.r#east == East::Low && self.r#up == false { return 23887; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Tall && self.r#up == true { return 23976; }
        if self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::Low { return 23959; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Low && self.r#west == West::Tall && self.r#east == East::Tall && self.r#up == false { return 24020; }
        if self.r#west == West::Low && self.r#up == false && self.r#north == North::None && self.r#south == South::Tall && self.r#east == East::Tall && self.r#waterlogged == false { return 23998; }
        if self.r#north == North::Tall && self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == true { return 24064; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Low && self.r#east == East::None && self.r#north == North::None && self.r#up == false { return 23766; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#west == West::None && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true { return 23994; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::None && self.r#up == true && self.r#west == West::Low { return 23773; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::Low && self.r#up == true && self.r#south == South::None && self.r#waterlogged == false { return 23789; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::None { return 23980; }
        if self.r#south == South::None && self.r#up == true && self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::None { return 23859; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::Low && self.r#up == false { return 23879; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Tall && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true { return 23888; }
        if self.r#south == South::None && self.r#east == East::None && self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::None && self.r#up == true { return 23784; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#up == true && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::None { return 23821; }
        if self.r#up == false && self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::Tall { return 23996; }
        if self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == false && self.r#west == West::None && self.r#waterlogged == true { return 24030; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == true && self.r#west == West::None { return 24048; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == true && self.r#north == North::Low && self.r#west == West::None { return 24027; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::None && self.r#north == North::None { return 23864; }
        if self.r#waterlogged == true && self.r#up == false && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Tall { return 23923; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#up == true && self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::Low { return 23869; }
        if self.r#south == South::None && self.r#north == North::None && self.r#east == East::Tall && self.r#waterlogged == true && self.r#up == false && self.r#west == West::Low { return 23971; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::Tall { return 24052; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true { return 23774; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Tall && self.r#up == false { return 23804; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::None && self.r#up == false && self.r#west == West::None { return 23793; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#up == true && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::None { return 23813; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::None { return 23829; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#north == North::Tall && self.r#east == East::Low && self.r#up == false && self.r#west == West::Low { return 23962; }
        if self.r#north == North::Low && self.r#west == West::None && self.r#up == true && self.r#east == East::Tall && self.r#south == South::Low && self.r#waterlogged == true { return 24012; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Low { return 23906; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::None && self.r#up == true && self.r#west == West::Tall && self.r#south == South::Tall { return 23777; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == true && self.r#east == East::None && self.r#west == West::Tall { return 23846; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == true { return 23872; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Tall { return 23978; }
        if self.r#south == South::Tall && self.r#up == true && self.r#east == East::None && self.r#west == West::None && self.r#north == North::Low && self.r#waterlogged == true { return 23808; }
        if self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None && self.r#south == South::Low { return 23979; }
        if self.r#north == North::None && self.r#west == West::None && self.r#south == South::None && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == false { return 23865; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::None && self.r#waterlogged == false && self.r#up == false { return 23795; }
        if self.r#up == true && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::None && self.r#west == West::None && self.r#north == North::Tall { return 23847; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true { return 24062; }
        if self.r#up == true && self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::None { return 23919; }
        if self.r#south == South::None && self.r#west == West::Low && self.r#east == East::None && self.r#waterlogged == false && self.r#north == North::None && self.r#up == true { return 23752; }
        if self.r#up == false && self.r#west == West::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Tall { return 23830; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Low { return 24049; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Low && self.r#up == false && self.r#south == South::None && self.r#west == West::Low { return 23791; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#up == false && self.r#west == West::Low && self.r#east == East::Low && self.r#waterlogged == true { return 23875; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#up == false && self.r#east == East::Tall && self.r#west == West::Low { return 24007; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::None && self.r#up == false && self.r#south == South::None { return 23759; }
        if self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Low && self.r#up == false { return 23902; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::None && self.r#up == true && self.r#north == North::Tall && self.r#south == South::Tall { return 23844; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::None && self.r#up == false && self.r#east == East::Tall && self.r#north == North::Low { return 24009; }
        if self.r#up == true && self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::Low { return 23800; }
        if self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 24008; }
        if self.r#up == true && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::Low { return 23904; }
        if self.r#up == true && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::Tall { return 24065; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#north == North::None && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true { return 23982; }
        if self.r#west == West::Low && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == true && self.r#up == true && self.r#east == East::Low { return 23857; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Low && self.r#up == false && self.r#south == South::Low { return 23986; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low { return 24046; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Low { return 24043; }
        if self.r#south == South::Low && self.r#north == North::None && self.r#west == West::Low && self.r#east == East::Tall && self.r#waterlogged == true && self.r#up == true { return 23977; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false { return 23842; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false { return 23907; }
        if self.r#up == false && self.r#east == East::Tall && self.r#west == West::Low && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Low { return 24019; }
        if self.r#south == South::None && self.r#east == East::Low && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall { return 23894; }
        if self.r#up == true && self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::Tall { return 23909; }
        if self.r#north == North::Low && self.r#west == West::Tall && self.r#up == false && self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::None { return 23792; }
        if self.r#east == East::None && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Tall && self.r#up == false { return 23831; }
        if self.r#north == North::Tall && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::None { return 23932; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#up == false && self.r#south == South::Low && self.r#east == East::None && self.r#north == North::Tall { return 23839; }
        if self.r#north == North::Tall && self.r#west == West::Tall && self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::Low && self.r#up == true { return 23945; }
        if self.r#up == false && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Low { return 23963; }
        if self.r#up == true && self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None { return 23856; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::None && self.r#east == East::None && self.r#north == North::None && self.r#up == false { return 23758; }
        if self.r#up == true && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::Low { return 23836; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == false && self.r#west == West::None && self.r#east == East::Low { return 23961; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None { return 23967; }
        if self.r#waterlogged == false && self.r#up == true && self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::None { return 23811; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::None && self.r#up == true { return 23753; }
        if self.r#east == East::Low && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false { return 23920; }
        if self.r#waterlogged == true && self.r#up == false && self.r#east == East::Low && self.r#west == West::None && self.r#north == North::Low && self.r#south == South::Tall { return 23922; }
        if self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::Tall && self.r#west == West::Low { return 24022; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#up == true && self.r#north == North::None && self.r#east == East::Tall && self.r#waterlogged == false { return 23992; }
        if self.r#up == true && self.r#north == North::Tall && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Low { return 23943; }
        if self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Low && self.r#east == East::Low { return 23863; }
        if self.r#up == false && self.r#west == West::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::Tall { return 24031; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Tall { return 23889; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#west == West::Low && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true { return 23983; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#west == West::Low && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == true { return 24025; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Low && self.r#up == true { return 23809; }
        if self.r#south == South::Tall && self.r#up == true && self.r#west == West::None && self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == true { return 23880; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::None && self.r#up == false && self.r#east == East::Low && self.r#west == West::None { return 23934; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::Tall && self.r#up == false && self.r#south == South::Low && self.r#west == West::Low { return 24058; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::None && self.r#west == West::None && self.r#up == true && self.r#waterlogged == true { return 23868; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Low { return 24028; }
        if self.r#up == true && self.r#south == South::None && self.r#west == West::None && self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::Tall { return 24039; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true { return 23768; }
        if self.r#south == South::None && self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::None && self.r#north == North::Tall { return 23820; }
        if self.r#south == South::None && self.r#east == East::Low && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low { return 23893; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::Tall && self.r#up == true && self.r#west == West::Low { return 23848; }
        if self.r#waterlogged == false && self.r#east == East::Low && self.r#up == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#north == North::None { return 23884; }
        if self.r#north == North::Tall && self.r#up == false && self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Low { return 24070; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#up == false && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::None { return 23878; }
        if self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::Tall { return 23915; }
        if self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::None && self.r#west == West::Tall { return 24002; }
        if self.r#north == North::Low && self.r#up == false && self.r#west == West::Tall && self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == false { return 24035; }
        if self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::Low { return 23927; }
        if self.r#south == South::Low && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::None { return 23940; }
        if self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false { return 24034; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#west == West::None && self.r#up == false && self.r#south == South::Low && self.r#waterlogged == false { return 23949; }
        if self.r#up == true && self.r#south == South::Low && self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Low { return 23761; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#east == East::None && self.r#up == true && self.r#west == West::Tall { return 23837; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Tall && self.r#up == false && self.r#north == North::Low && self.r#east == East::Low { return 23900; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Tall && self.r#up == true && self.r#west == West::Low && self.r#south == South::None { return 23965; }
        if self.r#east == East::None && self.r#south == South::None && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall { return 23750; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::Tall && self.r#up == false { return 23852; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::Tall { return 23835; }
        if self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::None && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false { return 23770; }
        if self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Low { return 23876; }
        if self.r#up == true && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Low { return 23882; }
        if self.r#up == true && self.r#north == North::Low && self.r#west == West::None && self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == true { return 23892; }
        if self.r#south == South::None && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 23936; }
        if self.r#waterlogged == false && self.r#up == false && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::None { return 23938; }
        if self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::None && self.r#east == East::Low { return 23955; }
        if self.r#west == West::Tall && self.r#south == South::Tall && self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true { return 24068; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None { return 23796; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::Low && self.r#up == true && self.r#west == West::None && self.r#north == North::Low { return 23799; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == false { return 24071; }
        if self.r#west == West::Tall && self.r#south == South::Tall && self.r#up == false && self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == false { return 23783; }
        if self.r#up == true && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Tall { return 23990; }
        if self.r#east == East::None && self.r#west == West::None && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::Tall { return 23850; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::None && self.r#up == true && self.r#south == South::None && self.r#east == East::Tall { return 24000; }
        if self.r#east == East::Low && self.r#up == true && self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::None { return 23931; }
        if self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None { return 23997; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Low { return 23827; }
        if self.r#north == North::Low && self.r#west == West::Tall && self.r#up == true && self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::Low { return 23801; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::None && self.r#east == East::Low && self.r#up == true { return 23928; }
        if self.r#south == South::None && self.r#north == North::Low && self.r#up == true && self.r#west == West::None && self.r#east == East::None && self.r#waterlogged == false { return 23787; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::None && self.r#up == false && self.r#north == North::None && self.r#south == South::Tall { return 23781; }
        if self.r#west == West::Tall && self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false { return 23819; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::None { return 23891; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::Low && self.r#west == West::Tall && self.r#east == East::Tall && self.r#up == false { return 23984; }
        if self.r#up == true && self.r#north == North::None && self.r#south == South::Tall && self.r#east == East::Tall && self.r#west == West::None && self.r#waterlogged == true { return 23988; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::Tall && self.r#north == North::None { return 23991; }
        if self.r#south == South::None && self.r#north == North::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall { return 24041; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::Low && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true { return 24056; }
        if self.r#east == East::None && self.r#up == false && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall { return 23756; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::None && self.r#up == true && self.r#west == West::None { return 23751; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::Low && self.r#up == true { return 23885; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#up == true && self.r#south == South::None && self.r#west == West::Low { return 24037; }
        if self.r#up == true && self.r#north == North::None && self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::Low { return 23871; }
        if self.r#up == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::None && self.r#north == North::None { return 23861; }
        if self.r#east == East::Tall && self.r#west == West::None && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Low { return 24018; }
        if self.r#waterlogged == false && self.r#up == false && self.r#east == East::Low && self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::Tall { return 23951; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::None && self.r#up == true && self.r#north == North::Tall { return 24036; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#up == true && self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == true { return 23832; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true { return 23840; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::Low && self.r#up == true && self.r#east == East::None { return 23785; }
        if self.r#north == North::None && self.r#west == West::None && self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == false && self.r#south == South::None { return 23973; }
        if self.r#north == North::Low && self.r#west == West::None && self.r#east == East::None && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true { return 23802; }
        if self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Tall { return 23921; }
        if self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::Low { return 23944; }
        if self.r#north == North::Tall && self.r#west == West::Tall && self.r#waterlogged == false && self.r#up == true && self.r#south == South::None && self.r#east == East::Low { return 23933; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#west == West::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true { return 23918; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::Tall && self.r#north == North::None && self.r#west == West::Low && self.r#up == true { return 23989; }
        if self.r#east == East::Tall && self.r#up == false && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Low { return 24010; }
        if self.r#north == North::None && self.r#west == West::None && self.r#waterlogged == true && self.r#up == true && self.r#south == South::Tall && self.r#east == East::None { return 23772; }
        if self.r#north == North::Tall && self.r#up == false && self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None { return 23841; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Tall && self.r#up == false && self.r#north == North::Low { return 24033; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#west == West::Low && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false { return 23890; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None { return 23814; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low { return 23899; }
        if self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::Low && self.r#west == West::Low { return 23908; }
        if self.r#north == North::Low && self.r#west == West::None && self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::None && self.r#up == false { return 24006; }
        if self.r#south == South::None && self.r#up == false && self.r#east == East::Tall && self.r#north == North::None && self.r#west == West::Low && self.r#waterlogged == false { return 23974; }
        if self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::Low && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == false { return 24016; }
        if self.r#south == South::Tall && self.r#up == false && self.r#east == East::None && self.r#west == West::None && self.r#north == North::None && self.r#waterlogged == true { return 23778; }
        if self.r#up == false && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::None { return 23803; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#up == false && self.r#north == North::Low && self.r#south == South::Low && self.r#west == West::Tall { return 23807; }
        if self.r#waterlogged == false && self.r#up == true && self.r#east == East::None && self.r#north == North::Tall && self.r#west == West::Low && self.r#south == South::None { return 23824; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#up == false && self.r#west == West::None { return 23898; }
        if self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::Tall { return 23958; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None { return 24042; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == false && self.r#east == East::None { return 23855; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#west == West::Tall && self.r#up == true && self.r#waterlogged == false && self.r#south == South::None { return 23897; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == true && self.r#east == East::Tall && self.r#south == South::Tall && self.r#north == North::None { return 23993; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#up == false { return 23886; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#up == true { return 23917; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == false { return 23950; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#up == true && self.r#east == East::Tall && self.r#south == South::None && self.r#waterlogged == false { return 24004; }
        if self.r#waterlogged == false && self.r#up == false && self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::Tall && self.r#west == West::None { return 24057; }
        if self.r#south == South::None && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Low && self.r#east == East::Low { return 23929; }
        if self.r#north == North::Tall && self.r#west == West::Tall && self.r#south == South::None && self.r#up == false && self.r#east == East::None && self.r#waterlogged == true { return 23828; }
        if self.r#west == West::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#up == false && self.r#north == North::Low { return 23806; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Tall && self.r#up == true && self.r#east == East::None { return 23810; }
        if self.r#east == East::Low && self.r#up == false && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low { return 23903; }
        if self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Tall && self.r#north == North::Tall { return 24047; }
        if self.r#up == true && self.r#west == West::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Low { return 24053; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#up == true { return 24061; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::Tall { return 24069; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::None && self.r#up == false && self.r#south == South::None { return 23826; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#up == true && self.r#east == East::None && self.r#west == West::Tall && self.r#south == South::Low { return 23834; }
        if self.r#east == East::None && self.r#south == South::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::None { return 23748; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#north == North::None { return 23775; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#south == South::Low && self.r#up == true && self.r#north == North::Low && self.r#east == East::Tall { return 24013; }
        if self.r#up == false && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Low && self.r#south == South::None { return 23755; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#up == true && self.r#west == West::Low && self.r#north == North::None && self.r#waterlogged == true { return 23881; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::Tall && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == true { return 23995; }
        if self.r#up == true && self.r#north == North::Tall && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low { return 23942; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None { return 24015; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == true && self.r#west == West::Tall { return 24026; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#up == true && self.r#west == West::Tall { return 24050; }
        if self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Tall { return 24059; }
        if self.r#south == South::Low && self.r#east == East::None && self.r#west == West::None && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true { return 23760; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#up == false { return 23838; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#up == false && self.r#east == East::Low && self.r#south == South::None && self.r#north == North::Tall { return 23937; }
        if self.r#north == North::Low && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low { return 24014; }
        if self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Low && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == false { return 23818; }
        if self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == true && self.r#up == false { return 23767; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None { return 23910; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::Low { return 23914; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None { return 23769; }
        if self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::None { return 23851; }
        if self.r#up == true && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::None && self.r#west == West::Low && self.r#east == East::Low { return 23860; }
        if self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Tall && self.r#west == West::Low { return 23953; }
        if self.r#north == North::None && self.r#up == true && self.r#east == East::None && self.r#west == West::None && self.r#south == South::Low && self.r#waterlogged == false { return 23763; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::None && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false { return 23985; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::None { return 23822; }
        if self.r#east == East::Tall && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false { return 24051; }
        if self.r#south == South::Low && self.r#north == North::None && self.r#up == true && self.r#east == East::None && self.r#west == West::Tall && self.r#waterlogged == true { return 23762; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::None { return 24021; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Low { return 23911; }
        if self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Tall && self.r#south == South::Tall { return 23957; }
        if self.r#north == North::Tall && self.r#west == West::Tall && self.r#south == South::None && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Tall { return 24038; }
        if self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == false { return 24066; }
        if self.r#up == false && self.r#north == North::None && self.r#east == East::None && self.r#west == West::Low && self.r#south == South::Tall && self.r#waterlogged == true { return 23779; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23954 {
            return Some(PolishedTuffWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23912 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 24067 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 23853 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 23930 {
            return Some(PolishedTuffWall {
                r#west: West::Tall,
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 23972 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 23970 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 23786 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 23948 {
            return Some(PolishedTuffWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 24032 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 23782 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 23794 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 23926 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 23798 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24040 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 23749 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 23812 {
            return Some(PolishedTuffWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 23764 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 23946 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 23941 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 23975 {
            return Some(PolishedTuffWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23843 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 23916 {
            return Some(PolishedTuffWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 23780 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 23805 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 23867 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 23797 {
            return Some(PolishedTuffWall {
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 23874 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 23947 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 23815 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 23987 {
            return Some(PolishedTuffWall {
                r#north: North::None,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 23956 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 23913 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 23905 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 23964 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 24005 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24003 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 24060 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 23877 {
            return Some(PolishedTuffWall {
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 23771 {
            return Some(PolishedTuffWall {
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23754 {
            return Some(PolishedTuffWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 23825 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 23969 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 23924 {
            return Some(PolishedTuffWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 24045 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 23858 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 23788 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 23776 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 23823 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 23866 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 23896 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 23790 {
            return Some(PolishedTuffWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 24001 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 24011 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 23757 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 24029 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 23833 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 24055 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 23966 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 23854 {
            return Some(PolishedTuffWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 23816 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 23870 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 23895 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 23935 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 23873 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 24054 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 23952 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 23968 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 23862 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 23765 {
            return Some(PolishedTuffWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 23939 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 23960 {
            return Some(PolishedTuffWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 23981 {
            return Some(PolishedTuffWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 24023 {
            return Some(PolishedTuffWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 24044 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 23925 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 23999 {
            return Some(PolishedTuffWall {
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 23901 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 23883 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24024 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 23817 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 23849 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 24017 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 24063 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 23845 {
            return Some(PolishedTuffWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 23887 {
            return Some(PolishedTuffWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 23976 {
            return Some(PolishedTuffWall {
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 23959 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 24020 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 23998 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24064 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 23766 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 23994 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23773 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 23789 {
            return Some(PolishedTuffWall {
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 23980 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 23859 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 23879 {
            return Some(PolishedTuffWall {
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 23888 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 23784 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 23821 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 23996 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 24030 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24048 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 24027 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 23864 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 23923 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 23869 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 23971 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 24052 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 23774 {
            return Some(PolishedTuffWall {
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23804 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 23793 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 23813 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 23829 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 23962 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 24012 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 23906 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 23777 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 23846 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 23872 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 23978 {
            return Some(PolishedTuffWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 23808 {
            return Some(PolishedTuffWall {
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 23979 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 23865 {
            return Some(PolishedTuffWall {
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 23795 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 23847 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 24062 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 23919 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 23752 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 23830 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 24049 {
            return Some(PolishedTuffWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 23791 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 23875 {
            return Some(PolishedTuffWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 24007 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 23759 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 23902 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 23844 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 24009 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 23800 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 24008 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23904 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 24065 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 23982 {
            return Some(PolishedTuffWall {
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23857 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 23986 {
            return Some(PolishedTuffWall {
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 24046 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 24043 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 23977 {
            return Some(PolishedTuffWall {
                r#south: South::Low,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 23842 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23907 {
            return Some(PolishedTuffWall {
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 24019 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 23894 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23909 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 23792 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 23831 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 23932 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 23839 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 23945 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 23963 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 23856 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 23758 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 23836 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 23961 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 23967 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 23811 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 23753 {
            return Some(PolishedTuffWall {
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 23920 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23922 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 24022 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 23992 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 23943 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 23863 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 24031 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 23889 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 23983 {
            return Some(PolishedTuffWall {
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 24025 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 23809 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 23880 {
            return Some(PolishedTuffWall {
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 23934 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 24058 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 23868 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 24028 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 24039 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 23768 {
            return Some(PolishedTuffWall {
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23820 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 23893 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 23848 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 23884 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 24070 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 23878 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 23915 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 24002 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 24035 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 23927 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 23940 {
            return Some(PolishedTuffWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 24034 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23949 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 23761 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 23837 {
            return Some(PolishedTuffWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23900 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 23965 {
            return Some(PolishedTuffWall {
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 23750 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23852 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 23835 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 23770 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23876 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 23882 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 23892 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 23936 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23938 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 23955 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 24068 {
            return Some(PolishedTuffWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23796 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 23799 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 24071 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 23783 {
            return Some(PolishedTuffWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 23990 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 23850 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 24000 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#up: true,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 23931 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 23997 {
            return Some(PolishedTuffWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 23827 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 23801 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 23928 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 23787 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 23781 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 23819 {
            return Some(PolishedTuffWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23891 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 23984 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 23988 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 23991 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 24041 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24056 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 23756 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23751 {
            return Some(PolishedTuffWall {
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 23885 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 24037 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 23871 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 23861 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 24018 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 23951 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 24036 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 23832 {
            return Some(PolishedTuffWall {
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 23840 {
            return Some(PolishedTuffWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23785 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 23973 {
            return Some(PolishedTuffWall {
                r#north: North::None,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 23802 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23921 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 23944 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 23933 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 23918 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23989 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 24010 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 23772 {
            return Some(PolishedTuffWall {
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 23841 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 24033 {
            return Some(PolishedTuffWall {
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 23890 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23814 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 23899 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 23908 {
            return Some(PolishedTuffWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 24006 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 23974 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 24016 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 23778 {
            return Some(PolishedTuffWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 23803 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 23807 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 23824 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 23898 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 23958 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 24042 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 23855 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 23897 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 23993 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 23886 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 23917 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 23950 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 24004 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 24057 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 23929 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 23828 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 23806 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 23810 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 23903 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 24047 {
            return Some(PolishedTuffWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 24053 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 24061 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 24069 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 23826 {
            return Some(PolishedTuffWall {
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 23834 {
            return Some(PolishedTuffWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 23748 {
            return Some(PolishedTuffWall {
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 23775 {
            return Some(PolishedTuffWall {
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 24013 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 23755 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 23881 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 23995 {
            return Some(PolishedTuffWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23942 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 24015 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 24026 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24050 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24059 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 23760 {
            return Some(PolishedTuffWall {
                r#south: South::Low,
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23838 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 23937 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 24014 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 23818 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 23767 {
            return Some(PolishedTuffWall {
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 23910 {
            return Some(PolishedTuffWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 23914 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 23769 {
            return Some(PolishedTuffWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 23851 {
            return Some(PolishedTuffWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 23860 {
            return Some(PolishedTuffWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 23953 {
            return Some(PolishedTuffWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 23763 {
            return Some(PolishedTuffWall {
                r#north: North::None,
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 23985 {
            return Some(PolishedTuffWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 23822 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 24051 {
            return Some(PolishedTuffWall {
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23762 {
            return Some(PolishedTuffWall {
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 24021 {
            return Some(PolishedTuffWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 23911 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 23957 {
            return Some(PolishedTuffWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 24038 {
            return Some(PolishedTuffWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 24066 {
            return Some(PolishedTuffWall {
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 23779 {
            return Some(PolishedTuffWall {
                r#up: false,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

