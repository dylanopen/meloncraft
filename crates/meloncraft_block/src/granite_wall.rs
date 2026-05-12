use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraniteWall {
    pub r#west: West,
    pub r#north: North,
    pub up: bool,
    pub r#south: South,
    pub r#east: East,
    pub waterlogged: bool,
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

impl BlockState for GraniteWall {
    fn to_id(self) -> i32 {
        if block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 17875; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#west == West::Tall { return 17848; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 17672; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::None { return 17825; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Low { return 17625; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::Tall { return 17615; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Low { return 17899; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true { return 17812; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::None { return 17613; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Low { return 17745; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Low { return 17721; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None { return 17594; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == true { return 17761; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#up == false { return 17851; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 17778; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Low { return 17642; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 17791; }
        if block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 17861; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Tall { return 17684; }
        if block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == false { return 17635; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#north == North::None { return 17816; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::None { return 17843; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::Low { return 17631; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 17837; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 17904; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true { return 17673; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 17654; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Low { return 17601; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false { return 17683; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 17787; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::Low { return 17653; }
        if block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == true { return 17737; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::None { return 17741; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == false { return 17850; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Tall { return 17887; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Tall { return 17694; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::None { return 17828; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 17691; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 17709; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::None { return 17888; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::Low { return 17643; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 17623; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == false { return 17874; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 17619; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#west == West::None { return 17795; }
        if block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == false { return 17607; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::None { return 17777; }
        if block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::None { return 17819; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 17883; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 17732; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::None { return 17655; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::None { return 17878; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 17724; }
        if block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 17740; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall { return 17885; }
        if block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 17690; }
        if block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 17817; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall { return 17876; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::None { return 17762; }
        if block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 17786; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Low { return 17652; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 17897; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None { return 17588; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 17898; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == false { return 17752; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 17836; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::None { return 17600; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Tall { return 17839; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::None { return 17702; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::None { return 17648; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true { return 17697; }
        if block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true { return 17593; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#north == North::None { return 17835; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#up == false { return 17750; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None { return 17831; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true { return 17869; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 17901; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false { return 17753; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 17803; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall { return 17676; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::None { return 17774; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 17841; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Tall { return 17678; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#north == North::None { return 17703; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::Low { return 17860; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 17884; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Low { return 17708; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 17891; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 17626; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 17905; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 17759; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::None { return 17666; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::None { return 17605; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::None { return 17665; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Tall { return 17866; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false { return 17872; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false { return 17658; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 17815; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::None { return 17636; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 17849; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false { return 17618; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 17757; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 17889; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None { return 17650; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 17754; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 17764; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::None { return 17682; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 17802; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 17767; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Tall { return 17693; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Tall { return 17616; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Tall { return 17660; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::None { return 17801; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == true { return 17830; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None { return 17685; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 17729; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::None { return 17846; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 17880; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::None { return 17838; }
        if block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 17749; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall { return 17731; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 17853; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 17886; }
        if block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true { return 17900; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 17903; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::None { return 17640; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 17722; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true { return 17698; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::None { return 17699; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == false { return 17742; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::None { return 17611; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false { return 17725; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 17769; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false { return 17826; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::None { return 17624; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::Tall { return 17695; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::Low { return 17700; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None { return 17807; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 17773; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Tall { return 17776; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::None { return 17629; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::Tall { return 17596; }
        if block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == true { return 17733; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::None { return 17780; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Tall { return 17668; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low { return 17852; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 17798; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::None { return 17879; }
        if block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true { return 17716; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true { return 17747; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None { return 17771; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Low { return 17604; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::None { return 17833; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::None { return 17842; }
        if block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 17870; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 17789; }
        if block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 17748; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true { return 17768; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None { return 17661; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Low { return 17894; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false { return 17719; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None { return 17808; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#south == South::None { return 17662; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 17680; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 17910; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::None { return 17638; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 17649; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 17677; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 17701; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#east == East::Tall { return 17896; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 17689; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::None { return 17704; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Low { return 17755; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true { return 17770; }
        if block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 17751; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 17598; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 17788; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 17908; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 17686; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None { return 17738; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Tall { return 17664; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 17797; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#south == South::None { return 17734; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low { return 17647; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 17847; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Low { return 17772; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == false { return 17630; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 17669; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::Low { return 17730; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#up == true { return 17735; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::None { return 17591; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::None { return 17804; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false { return 17610; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 17688; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 17758; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == true { return 17809; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true { return 17656; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::None { return 17717; }
        if block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 17727; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 17760; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::None { return 17599; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 17779; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::None { return 17628; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#up == false { return 17739; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Tall { return 17782; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low { return 17799; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::None { return 17810; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low { return 17639; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::Low { return 17736; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Low { return 17818; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall { return 17824; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None { return 17687; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::None { return 17834; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::None { return 17603; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None { return 17627; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true { return 17840; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == false { return 17895; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::Low { return 17707; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == true { return 17589; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::None { return 17822; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low { return 17712; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None { return 17612; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Tall { return 17857; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::Low { return 17608; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::Tall { return 17820; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 17877; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 17746; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 17765; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 17743; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::None { return 17632; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 17651; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#west == West::None { return 17675; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Low { return 17602; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 17671; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false { return 17715; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#south == South::Low { return 17711; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true { return 17796; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Low { return 17622; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Low { return 17706; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::None { return 17696; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 17726; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 17854; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 17855; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true { return 17806; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::None { return 17821; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == true { return 17856; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Low { return 17783; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 17892; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::None { return 17617; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 17763; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::Low { return 17766; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 17858; }
        if block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 17864; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Low { return 17714; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#west == West::None { return 17813; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 17790; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 17873; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::None { return 17606; }
        if block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 17832; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None { return 17646; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::None { return 17597; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 17881; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::None { return 17882; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::Tall { return 17893; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == true { return 17906; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None { return 17663; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Tall { return 17679; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::None { return 17609; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall { return 17667; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 17862; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 17793; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 17907; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Tall { return 17620; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#waterlogged == false { return 17633; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 17645; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 17805; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None { return 17705; }
        if block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 17871; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 17911; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 17657; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Low { return 17829; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Tall { return 17644; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 17659; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true { return 17867; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 17723; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 17890; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true { return 17744; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 17827; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 17681; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 17784; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false { return 17621; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false { return 17728; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::None { return 17595; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low { return 17863; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 17811; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Low { return 17718; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::Low { return 17775; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 17814; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true { return 17902; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 17592; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false { return 17670; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Tall { return 17800; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true { return 17845; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::None { return 17710; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Low { return 17637; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 17785; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::None { return 17634; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 17641; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 17614; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::Tall { return 17674; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == true { return 17590; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true { return 17792; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#up == false { return 17823; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 17713; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true { return 17844; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low { return 17859; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 17865; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 17868; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 17909; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::Low { return 17756; }
        if block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true { return 17720; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 17692; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == true { return 17781; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 17794; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 17875 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 17848 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 17672 {
            return Some(GraniteWall {
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17825 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 17625 {
            return Some(GraniteWall {
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 17615 {
            return Some(GraniteWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 17899 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17812 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17613 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17745 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17721 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 17594 {
            return Some(GraniteWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 17761 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 17851 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 17778 {
            return Some(GraniteWall {
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17642 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 17791 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 17861 {
            return Some(GraniteWall {
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 17684 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17635 {
            return Some(GraniteWall {
                r#east: East::None,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17816 {
            return Some(GraniteWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 17843 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17631 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 17837 {
            return Some(GraniteWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17904 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17673 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17654 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17601 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 17683 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 17787 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17653 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 17737 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17741 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 17850 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 17887 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17694 {
            return Some(GraniteWall {
                r#up: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17828 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17691 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17709 {
            return Some(GraniteWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17888 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 17643 {
            return Some(GraniteWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 17623 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 17874 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 17619 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17795 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 17607 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 17777 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 17819 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 17883 {
            return Some(GraniteWall {
                r#up: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17732 {
            return Some(GraniteWall {
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17655 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 17878 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17724 {
            return Some(GraniteWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17740 {
            return Some(GraniteWall {
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17885 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17690 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 17817 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17876 {
            return Some(GraniteWall {
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17762 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17786 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17652 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17897 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 17588 {
            return Some(GraniteWall {
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17898 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 17752 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 17836 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 17600 {
            return Some(GraniteWall {
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 17839 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17702 {
            return Some(GraniteWall {
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 17648 {
            return Some(GraniteWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 17697 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17593 {
            return Some(GraniteWall {
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17835 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 17750 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 17831 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 17869 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 17901 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17753 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 17803 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 17676 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17774 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17841 {
            return Some(GraniteWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17678 {
            return Some(GraniteWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17703 {
            return Some(GraniteWall {
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 17860 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17884 {
            return Some(GraniteWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17708 {
            return Some(GraniteWall {
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17891 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17626 {
            return Some(GraniteWall {
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17905 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17759 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17666 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 17605 {
            return Some(GraniteWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17665 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 17866 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17872 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 17658 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17815 {
            return Some(GraniteWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17636 {
            return Some(GraniteWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 17849 {
            return Some(GraniteWall {
                r#up: false,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17618 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 17757 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17889 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 17650 {
            return Some(GraniteWall {
                r#up: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 17754 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17764 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17682 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 17802 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17767 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17693 {
            return Some(GraniteWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17616 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17660 {
            return Some(GraniteWall {
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17801 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 17830 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 17685 {
            return Some(GraniteWall {
                r#up: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 17729 {
            return Some(GraniteWall {
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17846 {
            return Some(GraniteWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17880 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17838 {
            return Some(GraniteWall {
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17749 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17731 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17853 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17886 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17900 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17903 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17640 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 17722 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17698 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17699 {
            return Some(GraniteWall {
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 17742 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 17611 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 17725 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17769 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17826 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 17624 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 17695 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17700 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 17807 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 17773 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 17776 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17629 {
            return Some(GraniteWall {
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 17596 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 17733 {
            return Some(GraniteWall {
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 17780 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 17668 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 17852 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17798 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17879 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 17716 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17747 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 17771 {
            return Some(GraniteWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 17604 {
            return Some(GraniteWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 17833 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17842 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 17870 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17789 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17748 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 17768 {
            return Some(GraniteWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17661 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 17894 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 17719 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 17808 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 17662 {
            return Some(GraniteWall {
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17680 {
            return Some(GraniteWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 17910 {
            return Some(GraniteWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17638 {
            return Some(GraniteWall {
                r#up: true,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 17649 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17677 {
            return Some(GraniteWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17701 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17896 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 17689 {
            return Some(GraniteWall {
                r#east: East::None,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17704 {
            return Some(GraniteWall {
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 17755 {
            return Some(GraniteWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17770 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17751 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 17598 {
            return Some(GraniteWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17788 {
            return Some(GraniteWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17908 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 17686 {
            return Some(GraniteWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17738 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 17664 {
            return Some(GraniteWall {
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17797 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 17734 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17647 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17847 {
            return Some(GraniteWall {
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17772 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17630 {
            return Some(GraniteWall {
                r#east: East::None,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 17669 {
            return Some(GraniteWall {
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17730 {
            return Some(GraniteWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 17735 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 17591 {
            return Some(GraniteWall {
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 17804 {
            return Some(GraniteWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 17610 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 17688 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 17758 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17809 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 17656 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17717 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 17727 {
            return Some(GraniteWall {
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17760 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17599 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 17779 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 17628 {
            return Some(GraniteWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 17739 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 17782 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 17799 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17810 {
            return Some(GraniteWall {
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17639 {
            return Some(GraniteWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 17736 {
            return Some(GraniteWall {
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 17818 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 17824 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17687 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 17834 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17603 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 17627 {
            return Some(GraniteWall {
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 17840 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17895 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 17707 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 17589 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17822 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 17712 {
            return Some(GraniteWall {
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17612 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 17857 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 17608 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 17820 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 17877 {
            return Some(GraniteWall {
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17746 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 17765 {
            return Some(GraniteWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17743 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 17632 {
            return Some(GraniteWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 17651 {
            return Some(GraniteWall {
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17675 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 17602 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 17671 {
            return Some(GraniteWall {
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17715 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 17711 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 17796 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 17622 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 17706 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 17696 {
            return Some(GraniteWall {
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 17726 {
            return Some(GraniteWall {
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17854 {
            return Some(GraniteWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17855 {
            return Some(GraniteWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17806 {
            return Some(GraniteWall {
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17821 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17856 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 17783 {
            return Some(GraniteWall {
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17892 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17617 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 17763 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17766 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 17858 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17864 {
            return Some(GraniteWall {
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17714 {
            return Some(GraniteWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17813 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 17790 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17873 {
            return Some(GraniteWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17606 {
            return Some(GraniteWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 17832 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 17646 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 17597 {
            return Some(GraniteWall {
                r#up: false,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17881 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17882 {
            return Some(GraniteWall {
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17893 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 17906 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17663 {
            return Some(GraniteWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 17679 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17609 {
            return Some(GraniteWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 17667 {
            return Some(GraniteWall {
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17862 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17793 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17907 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17620 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17633 {
            return Some(GraniteWall {
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17645 {
            return Some(GraniteWall {
                r#up: false,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17805 {
            return Some(GraniteWall {
                r#south: South::None,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 17705 {
            return Some(GraniteWall {
                r#up: false,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17871 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17911 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17657 {
            return Some(GraniteWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17829 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 17644 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17659 {
            return Some(GraniteWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 17867 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 17723 {
            return Some(GraniteWall {
                r#west: West::None,
                r#up: true,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17890 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17744 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 17827 {
            return Some(GraniteWall {
                r#up: false,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17681 {
            return Some(GraniteWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17784 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17621 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 17728 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 17595 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 17863 {
            return Some(GraniteWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17811 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17718 {
            return Some(GraniteWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 17775 {
            return Some(GraniteWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 17814 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17902 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17592 {
            return Some(GraniteWall {
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17670 {
            return Some(GraniteWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 17800 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17845 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17710 {
            return Some(GraniteWall {
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 17637 {
            return Some(GraniteWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 17785 {
            return Some(GraniteWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17634 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 17641 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17614 {
            return Some(GraniteWall {
                r#north: North::None,
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17674 {
            return Some(GraniteWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17590 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 17792 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17823 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17713 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17844 {
            return Some(GraniteWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 17859 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17865 {
            return Some(GraniteWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17868 {
            return Some(GraniteWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17909 {
            return Some(GraniteWall {
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 17756 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 17720 {
            return Some(GraniteWall {
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17692 {
            return Some(GraniteWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17781 {
            return Some(GraniteWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 17794 {
            return Some(GraniteWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

