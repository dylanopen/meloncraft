use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateTileWall {
    pub waterlogged: bool,
    pub r#west: West,
    pub up: bool,
    pub r#north: North,
    pub r#south: South,
    pub r#east: East,
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

impl BlockState for DeepslateTileWall {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#up == true { return 28682; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 28792; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 28794; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low { return 28796; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 28809; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::None { return 28743; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::None { return 28643; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 28700; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::None { return 28639; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::None { return 28735; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::None { return 28857; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None { return 28644; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#waterlogged == false { return 28685; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::None { return 28729; }
        if block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 28789; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::None { return 28897; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::None { return 28654; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false { return 28638; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::None { return 28720; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::None { return 28641; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 28688; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 28923; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 28736; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 28704; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::None { return 28744; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#up == false { return 28799; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 28776; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::None { return 28696; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 28761; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low { return 28826; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 28713; }
        if block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 28764; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::None { return 28817; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == false { return 28859; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false { return 28943; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 28806; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::Tall { return 28941; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::None { return 28860; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 28835; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 28710; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::None { return 28717; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 28745; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::Low { return 28784; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 28807; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#waterlogged == true { return 28819; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::None { return 28892; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 28895; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 28950; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == false { return 28661; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::None { return 28881; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 28679; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None { return 28867; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true { return 28753; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 28742; }
        if block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::None { return 28778; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#waterlogged == true { return 28863; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true { return 28670; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 28686; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#south == South::None { return 28928; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Low { return 28861; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false { return 28697; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Tall { return 28824; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 28767; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 28666; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 28927; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Low { return 28691; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None { return 28652; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 28782; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 28934; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 28911; }
        if block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::None { return 28665; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 28940; }
        if block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 28755; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 28889; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 28805; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Tall { return 28821; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 28797; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 28718; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Tall { return 28723; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == false { return 28800; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None { return 28707; }
        if block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None { return 28649; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 28732; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low { return 28772; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 28838; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Tall { return 28845; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Low { return 28803; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None { return 28886; }
        if block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == true { return 28891; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true { return 28922; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Low { return 28724; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::Low { return 28813; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None { return 28942; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 28902; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::None { return 28822; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 28877; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false { return 28854; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false { return 28716; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true { return 28633; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Low { return 28777; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Low { return 28829; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Low { return 28637; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::None { return 28873; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#up == false { return 28918; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall { return 28930; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 28715; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low { return 28804; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None { return 28768; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None { return 28882; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 28919; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true { return 28671; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None { return 28828; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#east == East::Tall { return 28951; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::None { return 28885; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 28757; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false { return 28703; }
        if block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::Tall { return 28876; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::Low { return 28673; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 28770; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::None { return 28783; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 28832; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::None { return 28779; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == false { return 28944; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Tall { return 28668; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::None { return 28706; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 28841; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#waterlogged == false { return 28931; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::None { return 28675; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 28820; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false { return 28751; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 28702; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Tall { return 28875; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::Tall { return 28773; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::Tall { return 28858; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 28872; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 28851; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false { return 28846; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::None { return 28662; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == true { return 28694; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 28726; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == false { return 28893; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == false { return 28771; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None { return 28855; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 28722; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 28763; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Low { return 28901; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 28933; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::None { return 28853; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 28655; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None { return 28687; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Low { return 28692; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Tall { return 28740; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Low { return 28870; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::None { return 28711; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 28907; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 28765; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall { return 28905; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true { return 28780; }
        if block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == true { return 28840; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 28775; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 28839; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::Low { return 28795; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 28848; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::None { return 28738; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Low { return 28640; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Low { return 28754; }
        if block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false { return 28847; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 28866; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Low { return 28749; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false { return 28756; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 28801; }
        if block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false { return 28752; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::Tall { return 28921; }
        if block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false { return 28774; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::Tall { return 28739; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 28830; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true { return 28647; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false { return 28836; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#west == West::None { return 28816; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 28818; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::None { return 28747; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 28811; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == false { return 28714; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Tall { return 28733; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true { return 28837; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 28672; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 28833; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 28925; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true { return 28709; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == true { return 28699; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == false { return 28916; }
        if block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == false { return 28879; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 28812; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true { return 28862; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::Low { return 28769; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 28909; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None { return 28636; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall { return 28953; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 28728; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 28734; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 28815; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall { return 28725; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 28698; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 28825; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#west == West::None { return 28684; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#west == West::Tall { return 28674; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 28681; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None { return 28786; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 28658; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 28917; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::Low { return 28871; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#waterlogged == true { return 28676; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::Low { return 28748; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true { return 28926; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::None { return 28677; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == false { return 28760; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None { return 28814; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == true { return 28827; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None { return 28929; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::Low { return 28664; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Low { return 28721; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 28856; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 28945; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == false { return 28894; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#waterlogged == false { return 28708; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Low { return 28669; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 28878; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low { return 28683; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == true { return 28938; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Low { return 28823; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Low { return 28810; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 28884; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Low { return 28935; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 28693; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == false { return 28869; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#west == West::None { return 28936; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 28781; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == false { return 28955; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 28939; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 28766; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 28946; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 28952; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#south == South::Low { return 28648; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 28898; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 28653; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::None { return 28750; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 28914; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 28880; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::None { return 28635; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false { return 28883; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 28730; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false { return 28642; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true { return 28660; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 28645; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 28906; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::None { return 28690; }
        if block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 28954; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 28843; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 28949; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::None { return 28762; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::None { return 28634; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::Low { return 28791; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 28887; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == true { return 28842; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Low { return 28865; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 28788; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 28913; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low { return 28678; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 28890; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true { return 28657; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 28834; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::None { return 28741; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::Tall { return 28695; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 28850; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 28659; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::None { return 28656; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 28932; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::None { return 28651; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 28908; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::None { return 28948; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Low { return 28790; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::None { return 28680; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 28793; }
        if block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::None { return 28852; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::None { return 28746; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == false { return 28888; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low { return 28844; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::None { return 28759; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == true { return 28864; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 28920; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall { return 28947; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Tall { return 28831; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 28727; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 28719; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::None { return 28849; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 28915; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low { return 28868; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 28912; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 28701; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::None { return 28705; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false { return 28758; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 28900; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true { return 28924; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == true { return 28937; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Tall { return 28956; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::None { return 28650; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 28785; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 28899; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Low { return 28904; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Low { return 28667; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 28798; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Tall { return 28874; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::None { return 28731; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::Low { return 28903; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 28689; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 28712; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Low { return 28787; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#west == West::Low { return 28646; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 28808; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 28802; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Tall { return 28896; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::None { return 28737; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 28910; }
        if block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::None { return 28663; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 28682 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 28792 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 28794 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 28796 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 28809 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28743 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 28643 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 28700 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 28639 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 28735 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 28857 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 28644 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 28685 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28729 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 28789 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 28897 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 28654 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 28638 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28720 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 28641 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 28688 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28923 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28736 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28704 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28744 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 28799 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 28776 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28696 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 28761 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28826 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28713 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28764 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28817 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 28859 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 28943 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 28806 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 28941 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 28860 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 28835 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28710 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28717 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 28745 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 28784 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28807 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 28819 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28892 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 28895 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28950 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28661 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28881 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 28679 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 28867 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 28753 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 28742 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28778 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 28863 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28670 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 28686 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 28928 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 28861 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28697 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 28824 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 28767 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28666 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 28927 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 28691 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 28652 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 28782 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 28934 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28911 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 28665 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 28940 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28755 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28889 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28805 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28821 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 28797 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28718 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28723 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 28800 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 28707 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 28649 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 28732 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28772 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 28838 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 28845 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 28803 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 28886 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 28891 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 28922 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 28724 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 28813 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 28942 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 28902 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 28822 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 28877 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28854 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#south: South::None,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28716 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 28633 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28777 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 28829 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28637 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 28873 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 28918 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 28930 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 28715 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28804 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 28768 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 28882 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 28919 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28671 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28828 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 28951 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 28885 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 28757 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28703 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28876 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 28673 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 28770 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 28783 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 28832 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 28779 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 28944 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 28668 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28706 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 28841 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28931 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28675 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 28820 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 28751 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#up: false,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28702 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28875 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28773 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28858 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 28872 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28851 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28846 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28662 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 28694 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 28726 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28893 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 28771 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 28855 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 28722 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28763 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28901 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 28933 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28853 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 28655 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28687 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 28692 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28740 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28870 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 28711 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 28907 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 28765 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 28905 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 28780 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 28840 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 28775 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28839 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28795 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 28848 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28738 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 28640 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 28754 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 28847 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 28866 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28749 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 28756 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 28801 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28752 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28921 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 28774 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28739 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 28830 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28647 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28836 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 28816 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 28818 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28747 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 28811 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28714 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 28733 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28837 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 28672 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28833 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 28925 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28709 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 28699 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28916 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 28879 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 28812 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28862 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 28769 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28909 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 28636 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 28953 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28728 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 28734 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 28815 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28725 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28698 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28825 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28684 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 28674 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 28681 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 28786 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 28658 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28917 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28871 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 28676 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28748 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 28926 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 28677 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 28760 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 28814 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 28827 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 28929 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 28664 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 28721 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28856 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 28945 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28894 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 28708 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#up: true,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28669 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 28878 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#north: North::None,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 28683 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28938 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 28823 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 28810 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 28884 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28935 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 28693 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28869 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 28936 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 28781 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28955 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 28939 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28766 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28946 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 28952 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 28648 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 28898 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 28653 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#east: East::None,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28750 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 28914 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28880 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28635 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 28883 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 28730 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 28642 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28660 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 28645 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28906 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 28690 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 28954 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 28843 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 28949 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 28762 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 28634 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 28791 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28887 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28842 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 28865 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 28788 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 28913 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 28678 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 28890 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 28657 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 28834 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28741 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 28695 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 28850 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28659 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 28656 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 28932 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 28651 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 28908 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28948 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 28790 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 28680 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 28793 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 28852 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 28746 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 28888 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28844 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 28759 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 28864 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 28920 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28947 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 28831 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 28727 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28719 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 28849 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 28915 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28868 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28912 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28701 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 28705 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 28758 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28900 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28924 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 28937 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 28956 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 28650 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 28785 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28899 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28904 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 28667 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 28798 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 28874 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28731 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 28903 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 28689 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28712 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 28787 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28646 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28808 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28802 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 28896 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 28737 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 28910 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28663 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::None,
                r#west: West::None,
            });
        }
        return None;
    }
}

