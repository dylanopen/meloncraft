use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedBlackstoneWall {
    pub waterlogged: bool,
    pub r#south: South,
    pub r#east: East,
    pub r#west: West,
    pub up: bool,
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

impl BlockState for PolishedBlackstoneWall {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::None && self.r#up == false && self.r#east == East::Tall { return 22815; }
        if self.r#west == West::Tall && self.r#up == true && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Tall { return 22593; }
        if self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::None { return 22805; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Tall { return 22890; }
        if self.r#up == false && self.r#south == South::Low && self.r#north == North::None && self.r#east == East::Low && self.r#west == West::Tall && self.r#waterlogged == true { return 22695; }
        if self.r#west == West::Tall && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::None && self.r#up == false && self.r#south == South::None { return 22794; }
        if self.r#south == South::Low && self.r#north == North::None && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low { return 22697; }
        if self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Tall && self.r#east == East::Tall && self.r#west == West::Tall { return 22872; }
        if self.r#up == true && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#west == West::Low && self.r#waterlogged == true { return 22736; }
        if self.r#north == North::None && self.r#south == South::Tall && self.r#west == West::None && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Low { return 22705; }
        if self.r#south == South::None && self.r#up == true && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::None { return 22604; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == true && self.r#east == East::None && self.r#west == West::Low && self.r#south == South::None { return 22640; }
        if self.r#west == West::None && self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::Low && self.r#up == true { return 22714; }
        if self.r#south == South::None && self.r#north == North::None && self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None { return 22570; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#up == true { return 22771; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Low { return 22735; }
        if self.r#west == West::Low && self.r#up == false && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Low { return 22745; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Tall { return 22830; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None { return 22594; }
        if self.r#up == false && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::None && self.r#east == East::Tall { return 22790; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == true && self.r#north == North::Low { return 22834; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::Tall { return 22778; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true && self.r#west == West::None && self.r#east == East::Tall && self.r#waterlogged == true { return 22879; }
        if self.r#up == true && self.r#north == North::None && self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Low { return 22784; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == true { return 22811; }
        if self.r#east == East::Tall && self.r#west == West::None && self.r#up == false && self.r#south == South::Low && self.r#waterlogged == true && self.r#north == North::Tall { return 22873; }
        if self.r#up == true && self.r#east == East::None && self.r#west == West::None && self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::Low { return 22579; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::None && self.r#up == true && self.r#west == West::Low { return 22787; }
        if self.r#east == East::Tall && self.r#west == West::Tall && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::Tall { return 22887; }
        if self.r#waterlogged == false && self.r#south == South::None && self.r#up == false && self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::Low { return 22756; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#west == West::None && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true { return 22807; }
        if self.r#up == true && self.r#east == East::Low && self.r#north == North::Low && self.r#west == West::None && self.r#south == South::Low && self.r#waterlogged == true { return 22723; }
        if self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None { return 22765; }
        if self.r#up == true && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::Low { return 22832; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::None && self.r#up == false && self.r#east == East::None && self.r#west == West::None { return 22573; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Low && self.r#up == false { return 22633; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#up == false && self.r#north == North::Tall && self.r#west == West::Tall && self.r#east == East::Tall { return 22875; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::None && self.r#up == true && self.r#north == North::Low { return 22620; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None && self.r#up == false && self.r#south == South::Tall { return 22710; }
        if self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::None && self.r#west == West::Low { return 22568; }
        if self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Tall && self.r#south == South::Low { return 22731; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Tall && self.r#up == true && self.r#west == West::Tall { return 22656; }
        if self.r#waterlogged == true && self.r#up == false && self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::Tall && self.r#east == East::Low { return 22779; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#up == false && self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::None { return 22802; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall { return 22833; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#up == false && self.r#east == East::Low { return 22708; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == true && self.r#west == West::None && self.r#south == South::None && self.r#waterlogged == false { return 22858; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#up == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == true { return 22850; }
        if self.r#east == East::None && self.r#west == West::Low && self.r#north == North::Low && self.r#up == false && self.r#south == South::Low && self.r#waterlogged == false { return 22625; }
        if self.r#east == East::Tall && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true { return 22796; }
        if self.r#north == North::Tall && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false { return 22882; }
        if self.r#east == East::Low && self.r#west == West::None && self.r#waterlogged == false && self.r#up == false && self.r#south == South::Tall && self.r#north == North::Low { return 22744; }
        if self.r#up == false && self.r#west == West::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Low { return 22840; }
        if self.r#waterlogged == false && self.r#up == false && self.r#west == West::None && self.r#east == East::None && self.r#south == South::None && self.r#north == North::Low { return 22612; }
        if self.r#south == South::None && self.r#waterlogged == true && self.r#up == true && self.r#north == North::Tall && self.r#east == East::None && self.r#west == West::None { return 22639; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#up == true && self.r#south == South::None && self.r#north == North::Tall { return 22643; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low { return 22649; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::Low && self.r#up == true { return 22764; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == false && self.r#south == South::Low { return 22841; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::None && self.r#west == West::Low && self.r#up == false { return 22685; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Tall { return 22866; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == false { return 22614; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Low { return 22836; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::None && self.r#east == East::Low && self.r#up == false { return 22693; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#up == true { return 22689; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == true && self.r#south == South::Tall && self.r#east == East::Tall && self.r#west == West::Low { return 22880; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == false { return 22886; }
        if self.r#south == South::None && self.r#east == East::None && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall { return 22644; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::None && self.r#up == false { return 22757; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#up == true && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::None { return 22822; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == false { return 22885; }
        if self.r#up == false && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::Low { return 22658; }
        if self.r#up == false && self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Tall { return 22814; }
        if self.r#east == East::None && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == true && self.r#up == false && self.r#west == West::Low { return 22586; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::Low && self.r#up == true { return 22653; }
        if self.r#east == East::Low && self.r#south == South::None && self.r#north == North::Low && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true { return 22719; }
        if self.r#waterlogged == false && self.r#up == true && self.r#south == South::None && self.r#north == North::Tall && self.r#east == East::Tall && self.r#west == West::Low { return 22859; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::Low && self.r#north == North::Tall && self.r#up == false { return 22877; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Tall && self.r#up == true { return 22738; }
        if self.r#north == North::Low && self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::Low && self.r#up == true && self.r#east == East::None { return 22618; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Low && self.r#south == South::None && self.r#north == North::Low { return 22823; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == false && self.r#west == West::Tall { return 22782; }
        if self.r#north == North::None && self.r#south == South::None && self.r#west == West::Low && self.r#up == false && self.r#east == East::None && self.r#waterlogged == false { return 22577; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#up == true && self.r#west == West::Tall && self.r#east == East::Tall && self.r#south == South::Tall { return 22845; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#south == South::None && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true { return 22569; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Low && self.r#west == West::None && self.r#up == true && self.r#south == South::Tall { return 22843; }
        if self.r#south == South::Low && self.r#up == true && self.r#north == North::Tall && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::None { return 22651; }
        if self.r#up == true && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::Low && self.r#east == East::Tall && self.r#west == West::None { return 22819; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == false && self.r#west == West::None { return 22861; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::None && self.r#south == South::None && self.r#up == false { return 22648; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None && self.r#up == true { return 22666; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::None && self.r#east == East::Tall && self.r#west == West::Tall && self.r#up == false { return 22863; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Tall && self.r#up == true && self.r#north == North::Tall { return 22668; }
        if self.r#south == South::None && self.r#north == North::Tall && self.r#east == East::Tall && self.r#west == West::Low && self.r#waterlogged == true && self.r#up == false { return 22862; }
        if self.r#west == West::None && self.r#up == true && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == false { return 22678; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false { return 22846; }
        if self.r#west == West::Low && self.r#east == East::Tall && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::Low { return 22874; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::None { return 22627; }
        if self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::None { return 22637; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::None && self.r#up == false && self.r#south == South::None && self.r#north == North::Tall { return 22645; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 22635; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#waterlogged == true && self.r#south == South::None && self.r#up == false && self.r#west == West::Low { return 22754; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == false && self.r#east == East::None && self.r#waterlogged == true { return 22659; }
        if self.r#up == false && self.r#south == South::Tall && self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == false { return 22818; }
        if self.r#east == East::Tall && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low { return 22848; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == false { return 22662; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == false && self.r#west == West::Low && self.r#south == South::None && self.r#east == East::Tall { return 22865; }
        if self.r#south == South::Low && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Tall { return 22769; }
        if self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == false && self.r#west == West::None { return 22597; }
        if self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::None && self.r#west == West::None && self.r#north == North::None { return 22792; }
        if self.r#north == North::Tall && self.r#west == West::None && self.r#up == true && self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::Tall { return 22663; }
        if self.r#up == true && self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::Low && self.r#west == West::Low { return 22715; }
        if self.r#up == true && self.r#north == North::Low && self.r#east == East::Low && self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::None { return 22711; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::None && self.r#up == false { return 22634; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Tall { return 22698; }
        if self.r#up == false && self.r#west == West::None && self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::None { return 22636; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#up == true && self.r#east == East::Low && self.r#west == West::Low && self.r#south == South::Tall { return 22739; }
        if self.r#south == South::None && self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#up == false { return 22681; }
        if self.r#up == false && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::None { return 22729; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#up == false && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::None { return 22574; }
        if self.r#east == East::Low && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::Low { return 22694; }
        if self.r#north == North::None && self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == true { return 22810; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false && self.r#north == North::None && self.r#waterlogged == false { return 22806; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::Low && self.r#up == false && self.r#east == East::None && self.r#waterlogged == false { return 22626; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#west == West::Tall { return 22884; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::None && self.r#up == true && self.r#waterlogged == false { return 22595; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::None && self.r#west == West::None && self.r#up == true { return 22750; }
        if self.r#south == South::Low && self.r#north == North::None && self.r#east == East::None && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true { return 22587; }
        if self.r#up == true && self.r#east == East::Low && self.r#south == South::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Tall { return 22749; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::Tall { return 22780; }
        if self.r#north == North::Tall && self.r#west == West::None && self.r#up == false && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Low { return 22876; }
        if self.r#west == West::Tall && self.r#south == South::None && self.r#up == true && self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == false { return 22788; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None && self.r#up == false { return 22717; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::None && self.r#up == true { return 22630; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None { return 22669; }
        if self.r#west == West::Low && self.r#east == East::None && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Low { return 22589; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Low && self.r#up == true && self.r#west == West::Tall && self.r#north == North::Low { return 22617; }
        if self.r#east == East::Tall && self.r#up == true && self.r#north == North::None && self.r#south == South::Tall && self.r#west == West::Tall && self.r#waterlogged == true { return 22809; }
        if self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == false && self.r#up == true && self.r#south == South::Tall && self.r#west == West::Tall { return 22596; }
        if self.r#east == East::None && self.r#up == false && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Tall { return 22647; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None && self.r#up == true && self.r#north == North::Tall { return 22748; }
        if self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::None && self.r#west == West::None && self.r#waterlogged == false && self.r#up == false { return 22600; }
        if self.r#up == false && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::Tall { return 22741; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false { return 22753; }
        if self.r#south == South::Low && self.r#waterlogged == true && self.r#east == East::Tall && self.r#up == false && self.r#north == North::None && self.r#west == West::Tall { return 22803; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#up == true && self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::Low { return 22655; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#up == false && self.r#east == East::None && self.r#north == North::Low { return 22624; }
        if self.r#west == West::None && self.r#waterlogged == true && self.r#up == true && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::None { return 22687; }
        if self.r#south == South::Tall && self.r#north == North::None && self.r#west == West::Low && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == false { return 22709; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::Low && self.r#up == false { return 22770; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == true && self.r#north == North::Low { return 22632; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::None && self.r#south == South::Tall && self.r#up == false && self.r#west == West::Low { return 22598; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#up == false && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Low { return 22722; }
        if self.r#up == false && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Low { return 22746; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Tall { return 22773; }
        if self.r#east == East::None && self.r#south == South::None && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None { return 22567; }
        if self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Tall && self.r#west == West::Low && self.r#south == South::Low { return 22652; }
        if self.r#north == North::None && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::Tall { return 22791; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None { return 22816; }
        if self.r#west == West::Low && self.r#up == true && self.r#south == South::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::Tall { return 22799; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#up == false && self.r#west == West::Tall { return 22734; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Low && self.r#up == true && self.r#west == West::Tall && self.r#south == South::None { return 22605; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Low && self.r#up == false && self.r#west == West::Low && self.r#east == East::None { return 22622; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == true && self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::None { return 22751; }
        if self.r#up == false && self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Low { return 22804; }
        if self.r#north == North::Low && self.r#up == true && self.r#south == South::Low && self.r#west == West::Low && self.r#east == East::None && self.r#waterlogged == true { return 22616; }
        if self.r#east == East::Low && self.r#west == West::Low && self.r#waterlogged == false && self.r#south == South::None && self.r#up == true && self.r#north == North::None { return 22679; }
        if self.r#up == true && self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::None && self.r#west == West::Tall { return 22680; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == true { return 22860; }
        if self.r#west == West::None && self.r#up == true && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == true { return 22603; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall { return 22674; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::None && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == false { return 22650; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#up == false && self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::Low { return 22696; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low { return 22725; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#north == North::Tall && self.r#west == West::Low && self.r#up == true && self.r#waterlogged == true { return 22772; }
        if self.r#up == false && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Tall { return 22646; }
        if self.r#west == West::None && self.r#up == false && self.r#waterlogged == true && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::None { return 22813; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == true && self.r#south == South::None && self.r#east == East::Low { return 22676; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true && self.r#north == North::None { return 22581; }
        if self.r#west == West::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::Low && self.r#up == true && self.r#east == East::Low { return 22713; }
        if self.r#north == North::Low && self.r#east == East::Tall && self.r#west == West::Low && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false { return 22835; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == true && self.r#south == South::None && self.r#west == West::Low { return 22856; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#up == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#waterlogged == false { return 22667; }
        if self.r#north == North::None && self.r#up == true && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low { return 22677; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Tall { return 22867; }
        if self.r#west == West::None && self.r#up == false && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == false { return 22684; }
        if self.r#up == false && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::Tall { return 22638; }
        if self.r#west == West::None && self.r#up == true && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::None { return 22606; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#up == false && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::Low { return 22590; }
        if self.r#south == South::Low && self.r#west == West::Tall && self.r#east == East::None && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == true { return 22623; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::Tall && self.r#up == false && self.r#west == West::None { return 22672; }
        if self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == false && self.r#north == North::Tall { return 22755; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#east == East::Tall && self.r#west == West::Low && self.r#north == North::None && self.r#up == true { return 22808; }
        if self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None { return 22870; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::None && self.r#up == true && self.r#west == West::None && self.r#waterlogged == true { return 22699; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::Low && self.r#up == false { return 22733; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#up == true && self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Tall { return 22831; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::None && self.r#south == South::None { return 22572; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Tall { return 22670; }
        if self.r#up == false && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::Tall { return 22837; }
        if self.r#south == South::Tall && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == true { return 22774; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::None && self.r#west == West::Low && self.r#south == South::Tall && self.r#up == true { return 22631; }
        if self.r#up == true && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::Tall && self.r#west == West::Tall && self.r#east == East::Tall { return 22812; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::None && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == true { return 22707; }
        if self.r#north == North::Low && self.r#up == true && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Low { return 22726; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == false { return 22853; }
        if self.r#waterlogged == true && self.r#up == true && self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::Tall && self.r#east == East::Low { return 22761; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#up == false && self.r#east == East::Low && self.r#west == West::Tall && self.r#south == South::Tall { return 22743; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#east == East::Low && self.r#up == true { return 22724; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall { return 22888; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false { return 22763; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::Low { return 22869; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == false { return 22878; }
        if self.r#north == North::Low && self.r#west == West::Tall && self.r#east == East::Tall && self.r#waterlogged == true && self.r#up == false && self.r#south == South::None { return 22827; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Low && self.r#waterlogged == false && self.r#up == false && self.r#east == East::Low { return 22732; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::None && self.r#up == false && self.r#east == East::None { return 22578; }
        if self.r#south == South::None && self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == false && self.r#up == false && self.r#west == West::Low { return 22613; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Low && self.r#up == true { return 22691; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#up == false && self.r#west == West::None { return 22801; }
        if self.r#up == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::None { return 22817; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == false { return 22828; }
        if self.r#up == false && self.r#west == West::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::Low && self.r#east == East::None { return 22611; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Low { return 22842; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#up == false && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::None { return 22588; }
        if self.r#north == North::Tall && self.r#west == West::None && self.r#south == South::Low && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true { return 22657; }
        if self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Tall && self.r#east == East::Low { return 22701; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#up == false && self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == true { return 22777; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::None && self.r#up == true && self.r#south == South::None { return 22783; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#up == false { return 22766; }
        if self.r#up == false && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Tall && self.r#west == West::Tall { return 22851; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None { return 22759; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::None && self.r#up == true && self.r#south == South::None { return 22855; }
        if self.r#south == South::Low && self.r#up == false && self.r#west == West::None && self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == true { return 22585; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == false && self.r#east == East::Low { return 22781; }
        if self.r#up == false && self.r#west == West::Low && self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::Low && self.r#south == South::None { return 22682; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::None && self.r#up == false { return 22718; }
        if self.r#south == South::None && self.r#east == East::Low && self.r#west == West::Tall && self.r#waterlogged == false && self.r#up == true && self.r#north == North::Tall { return 22752; }
        if self.r#east == East::Low && self.r#up == true && self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::Tall && self.r#waterlogged == false { return 22776; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None { return 22786; }
        if self.r#south == South::Tall && self.r#west == West::Low && self.r#up == true && self.r#north == North::Low && self.r#east == East::Tall && self.r#waterlogged == true { return 22844; }
        if self.r#up == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::Tall { return 22871; }
        if self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::None { return 22671; }
        if self.r#south == South::Tall && self.r#up == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::None { return 22852; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == true { return 22703; }
        if self.r#east == East::Low && self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::Low && self.r#waterlogged == false && self.r#up == true { return 22727; }
        if self.r#south == South::Low && self.r#east == East::Low && self.r#west == West::Low && self.r#up == true && self.r#north == North::None && self.r#waterlogged == true { return 22688; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#west == West::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#up == true { return 22775; }
        if self.r#west == West::None && self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false { return 22864; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::None && self.r#south == South::None && self.r#east == East::None && self.r#up == true { return 22642; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#up == false && self.r#west == West::None && self.r#east == East::None && self.r#north == North::None { return 22576; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::None && self.r#up == true && self.r#west == West::Tall { return 22608; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == true && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::None { return 22629; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::None && self.r#up == true && self.r#south == South::Low { return 22690; }
        if self.r#west == West::Tall && self.r#waterlogged == true && self.r#north == North::None && self.r#up == false && self.r#south == South::Tall && self.r#east == East::None { return 22599; }
        if self.r#up == true && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::None && self.r#west == West::None { return 22675; }
        if self.r#waterlogged == true && self.r#up == false && self.r#south == South::None && self.r#east == East::None && self.r#north == North::None && self.r#west == West::Tall { return 22575; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#up == true && self.r#north == North::Tall && self.r#west == West::Tall && self.r#south == South::Tall { return 22665; }
        if self.r#east == East::None && self.r#up == false && self.r#north == North::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None { return 22660; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Low && self.r#up == false && self.r#east == East::Tall && self.r#south == South::None { return 22793; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Tall && self.r#up == false && self.r#north == North::Low && self.r#east == East::Tall { return 22854; }
        if self.r#up == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::Low { return 22742; }
        if self.r#north == North::Low && self.r#up == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::None { return 22628; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#up == false && self.r#west == West::Low { return 22673; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#up == true && self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::Low { return 22728; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == false { return 22826; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#up == true { return 22654; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#west == West::Tall && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == false { return 22692; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::Tall && self.r#east == East::Low && self.r#up == false { return 22706; }
        if self.r#waterlogged == true && self.r#up == false && self.r#west == West::Low && self.r#south == South::None && self.r#east == East::None && self.r#north == North::Low { return 22610; }
        if self.r#waterlogged == true && self.r#up == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Low && self.r#west == West::Tall { return 22839; }
        if self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == false { return 22849; }
        if self.r#south == South::None && self.r#up == true && self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Tall { return 22821; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true { return 22868; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#up == true && self.r#west == West::None { return 22591; }
        if self.r#east == East::Low && self.r#up == false && self.r#south == South::None && self.r#west == West::Tall && self.r#north == North::Tall && self.r#waterlogged == false { return 22758; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::Tall { return 22883; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Tall { return 22797; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::Low && self.r#up == true && self.r#south == South::Tall { return 22737; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true { return 22785; }
        if self.r#south == South::Tall && self.r#west == West::Low && self.r#up == false && self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == false { return 22601; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::None && self.r#west == West::Low && self.r#up == true && self.r#south == South::Low { return 22580; }
        if self.r#up == false && self.r#north == North::Tall && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Low { return 22768; }
        if self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::None && self.r#up == false { return 22789; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::Tall { return 22704; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Low && self.r#north == North::Low { return 22829; }
        if self.r#north == North::Tall && self.r#up == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#waterlogged == false { return 22889; }
        if self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Low { return 22825; }
        if self.r#waterlogged == false && self.r#up == true && self.r#east == East::None && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::None { return 22582; }
        if self.r#up == true && self.r#west == West::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Low { return 22712; }
        if self.r#east == East::None && self.r#up == false && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::None { return 22609; }
        if self.r#south == South::Tall && self.r#up == true && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == true { return 22664; }
        if self.r#up == true && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Low && self.r#west == West::Low && self.r#south == South::None { return 22607; }
        if self.r#north == North::Tall && self.r#up == false && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Low { return 22661; }
        if self.r#up == true && self.r#east == East::None && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Low { return 22571; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false { return 22716; }
        if self.r#up == true && self.r#west == West::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::Low { return 22760; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::None && self.r#up == true && self.r#west == West::None { return 22747; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#up == true && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Tall { return 22740; }
        if self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::None && self.r#east == East::Tall && self.r#up == true && self.r#north == North::None { return 22798; }
        if self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#up == false { return 22683; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == true && self.r#west == West::Tall && self.r#south == South::None { return 22824; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Low { return 22583; }
        if self.r#up == true && self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::None { return 22619; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == true && self.r#north == North::None && self.r#waterlogged == false { return 22702; }
        if self.r#south == South::Tall && self.r#up == true && self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Low { return 22592; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == true && self.r#east == East::None && self.r#south == South::Low { return 22584; }
        if self.r#east == East::None && self.r#south == South::None && self.r#north == North::Tall && self.r#west == West::Tall && self.r#up == true && self.r#waterlogged == true { return 22641; }
        if self.r#west == West::Tall && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false { return 22686; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#up == false && self.r#west == West::None && self.r#south == South::None && self.r#waterlogged == false { return 22720; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#up == false && self.r#east == East::Low && self.r#west == West::Low && self.r#south == South::Low { return 22730; }
        if self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::None && self.r#south == South::Low { return 22795; }
        if self.r#up == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::None { return 22700; }
        if self.r#east == East::Tall && self.r#up == true && self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Low { return 22820; }
        if self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Low && self.r#west == West::None { return 22615; }
        if self.r#west == West::Low && self.r#up == false && self.r#south == South::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::Tall { return 22838; }
        if self.r#south == South::Low && self.r#up == true && self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == false { return 22800; }
        if self.r#waterlogged == false && self.r#south == South::Low && self.r#up == true && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::Tall { return 22762; }
        if self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::Low && self.r#west == West::Low { return 22721; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == false && self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::Low { return 22767; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#up == false && self.r#south == South::Low && self.r#east == East::None && self.r#west == West::None { return 22621; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#up == true && self.r#east == East::Tall && self.r#south == South::None && self.r#west == West::Tall { return 22857; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == true { return 22847; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#up == true && self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == true { return 22881; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::None && self.r#west == West::Tall && self.r#up == false { return 22602; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22815 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 22593 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 22805 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 22890 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 22695 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22794 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 22697 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 22872 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 22736 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 22705 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 22604 {
            return Some(PolishedBlackstoneWall {
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 22640 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 22714 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 22570 {
            return Some(PolishedBlackstoneWall {
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 22771 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 22735 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 22745 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 22830 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 22594 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 22790 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 22834 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 22778 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 22879 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22784 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 22811 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 22873 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 22579 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 22787 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 22887 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 22756 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 22807 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22723 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 22765 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 22832 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 22573 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 22633 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 22875 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 22620 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 22710 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 22568 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 22731 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 22656 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22779 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 22802 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 22833 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22708 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 22858 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22850 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 22625 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 22796 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22882 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22744 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 22840 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 22612 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 22639 {
            return Some(PolishedBlackstoneWall {
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 22643 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 22649 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 22764 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 22841 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 22685 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 22866 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 22614 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 22836 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 22693 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 22689 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 22880 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 22886 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 22644 {
            return Some(PolishedBlackstoneWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 22757 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 22822 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 22885 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 22658 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 22814 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 22586 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 22653 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 22719 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22859 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 22877 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 22738 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 22618 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 22823 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 22782 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 22577 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22845 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 22569 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22843 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::None,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 22651 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Low,
                r#up: true,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 22819 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 22861 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 22648 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 22666 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 22863 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 22668 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 22862 {
            return Some(PolishedBlackstoneWall {
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 22678 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22846 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22874 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 22627 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 22637 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 22645 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 22635 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22754 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 22659 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 22818 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22848 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 22662 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 22865 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 22769 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 22597 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 22792 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 22663 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#west: West::None,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 22715 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 22711 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 22634 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 22698 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 22636 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 22739 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 22681 {
            return Some(PolishedBlackstoneWall {
                r#south: South::None,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 22729 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 22574 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 22694 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 22810 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 22806 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22626 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22884 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22595 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22750 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 22587 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22749 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22780 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 22876 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 22788 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#south: South::None,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22717 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 22630 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 22669 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 22589 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 22617 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 22809 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22596 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 22647 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 22748 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 22600 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 22741 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 22753 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 22803 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 22655 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 22624 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 22687 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 22709 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 22770 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 22632 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 22598 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 22722 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 22746 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 22773 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 22567 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 22652 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 22791 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 22816 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 22799 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 22734 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 22605 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 22622 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 22751 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 22804 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 22616 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 22679 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 22680 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 22860 {
            return Some(PolishedBlackstoneWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 22603 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 22674 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 22650 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22696 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 22725 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 22772 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22646 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 22813 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 22676 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 22581 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 22713 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 22835 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22856 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 22667 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 22677 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 22867 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 22684 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#up: false,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22638 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 22606 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 22590 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 22623 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 22672 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 22755 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 22808 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 22870 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 22699 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 22733 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 22831 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 22572 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 22670 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 22837 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 22774 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 22631 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 22812 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 22707 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22726 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 22853 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 22761 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 22743 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 22724 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 22888 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 22763 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22869 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 22878 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 22827 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 22732 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 22578 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 22613 {
            return Some(PolishedBlackstoneWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 22691 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 22801 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 22817 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 22828 {
            return Some(PolishedBlackstoneWall {
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 22611 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 22842 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 22588 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 22657 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22701 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 22777 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22783 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 22766 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 22851 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 22759 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 22855 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 22585 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 22781 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 22682 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 22718 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 22752 {
            return Some(PolishedBlackstoneWall {
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 22776 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 22786 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 22844 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22871 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 22671 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 22852 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 22703 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 22727 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 22688 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 22775 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 22864 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22642 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 22576 {
            return Some(PolishedBlackstoneWall {
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 22608 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22629 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 22690 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 22599 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 22675 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 22575 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 22665 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 22660 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 22793 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 22854 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 22742 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 22628 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 22673 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 22728 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 22826 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 22654 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 22692 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 22706 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 22610 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 22839 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 22849 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 22821 {
            return Some(PolishedBlackstoneWall {
                r#south: South::None,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22868 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22591 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 22758 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 22883 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 22797 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22737 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 22785 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 22601 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22580 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 22768 {
            return Some(PolishedBlackstoneWall {
                r#up: false,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 22789 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 22704 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 22829 {
            return Some(PolishedBlackstoneWall {
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 22889 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 22825 {
            return Some(PolishedBlackstoneWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 22582 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 22712 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 22609 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 22664 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22607 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 22661 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 22571 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 22716 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22760 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 22747 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 22740 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 22798 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 22683 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 22824 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 22583 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 22619 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 22702 {
            return Some(PolishedBlackstoneWall {
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22592 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 22584 {
            return Some(PolishedBlackstoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 22641 {
            return Some(PolishedBlackstoneWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22686 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22720 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22730 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 22795 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 22700 {
            return Some(PolishedBlackstoneWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 22820 {
            return Some(PolishedBlackstoneWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 22615 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 22838 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 22800 {
            return Some(PolishedBlackstoneWall {
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22762 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 22721 {
            return Some(PolishedBlackstoneWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 22767 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 22621 {
            return Some(PolishedBlackstoneWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 22857 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 22847 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 22881 {
            return Some(PolishedBlackstoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22602 {
            return Some(PolishedBlackstoneWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
            });
        }
        return None;
    }
}

