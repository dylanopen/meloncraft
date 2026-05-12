use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrismarineWall {
    pub r#south: South,
    pub r#east: East,
    pub up: bool,
    pub waterlogged: bool,
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

impl BlockState for PrismarineWall {
    fn to_id(self) -> i32 {
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 16879; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Low { return 16700; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#north == North::None { return 16759; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == false { return 16814; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 16858; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None { return 16803; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Low { return 16788; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::Low { return 16845; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall { return 16646; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false { return 16806; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None { return 16641; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 16687; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 16663; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 16825; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false { return 16835; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Tall { return 16919; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 16869; }
        if block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 16903; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 16922; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::None { return 16627; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 16925; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#east == East::None { return 16718; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 16793; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None { return 16872; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None { return 16631; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == true { return 16640; }
        if block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 16892; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::None { return 16622; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 16912; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 16780; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#waterlogged == false { return 16763; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Tall { return 16651; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Low { return 16768; }
        if block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::None { return 16629; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Low { return 16743; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 16839; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#east == East::Low { return 16829; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::None { return 16679; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == false { return 16660; }
        if block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 16864; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == false { return 16650; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == true { return 16848; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None { return 16734; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low { return 16749; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::None { return 16637; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Low { return 16665; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 16678; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::None { return 16697; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 16786; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false { return 16732; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::Low { return 16681; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::None { return 16618; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::None { return 16727; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 16842; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false { return 16851; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 16914; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall { return 16672; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low { return 16917; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 16833; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None { return 16836; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 16860; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::Low { return 16884; }
        if block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 16820; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 16939; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 16671; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::None { return 16688; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#west == West::None { return 16634; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::Low { return 16764; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 16753; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Low { return 16673; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 16750; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 16937; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true { return 16823; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 16924; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 16826; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None { return 16862; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::None { return 16693; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None { return 16853; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 16861; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Low { return 16658; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::None { return 16649; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 16654; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None { return 16664; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Low { return 16686; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None { return 16638; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 16685; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#up == true { return 16725; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#up == true { return 16715; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 16666; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Low { return 16762; }
        if block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true { return 16636; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::None { return 16620; }
        if block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 16834; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 16900; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 16776; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 16709; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 16808; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::Low { return 16816; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == false { return 16817; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 16889; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 16875; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 16752; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 16923; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low { return 16887; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#east == East::None { return 16717; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true { return 16748; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 16705; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false { return 16619; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 16716; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Tall { return 16849; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 16863; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Low { return 16782; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 16723; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 16648; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 16904; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::None { return 16670; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::None { return 16694; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 16674; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 16689; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#up == false { return 16682; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 16813; }
        if block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::None { return 16871; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Tall { return 16758; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::None { return 16784; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::None { return 16799; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::Low { return 16662; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::Tall { return 16690; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::None { return 16739; }
        if block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 16901; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 16810; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 16798; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::None { return 16691; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 16811; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::None { return 16703; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 16740; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Tall { return 16906; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 16882; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#west == West::Tall { return 16702; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false { return 16935; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall { return 16729; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Low { return 16773; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall { return 16840; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 16760; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 16683; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 16809; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Low { return 16779; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 16897; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 16625; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Tall { return 16805; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::None { return 16695; }
        if block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 16633; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false { return 16731; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 16647; }
        if block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false { return 16626; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true { return 16652; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Low { return 16708; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::None { return 16623; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == true { return 16642; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == true { return 16704; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == false { return 16795; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::None { return 16865; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Low { return 16893; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 16899; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::None { return 16905; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::None { return 16915; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::None { return 16804; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == true { return 16656; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Low { return 16628; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 16907; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None { return 16769; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::Low { return 16661; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true { return 16712; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 16771; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 16908; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::None { return 16645; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::Low { return 16711; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 16785; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#waterlogged == false { return 16883; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall { return 16932; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 16857; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Low { return 16737; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 16659; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 16894; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 16741; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Low { return 16677; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::None { return 16910; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 16929; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::None { return 16852; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Tall { return 16896; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 16801; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::Low { return 16675; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 16774; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 16878; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 16735; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 16891; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == true { return 16918; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 16855; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 16854; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None { return 16722; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false { return 16655; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall { return 16911; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Tall { return 16921; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 16707; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 16846; }
        if block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true { return 16653; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Low { return 16632; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 16792; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall { return 16890; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None { return 16667; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == true { return 16824; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 16669; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Low { return 16766; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 16706; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 16930; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == true { return 16724; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::Low { return 16744; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None { return 16692; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == false { return 16867; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 16787; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 16791; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Low { return 16881; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == true { return 16621; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Low { return 16796; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 16819; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None { return 16909; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false { return 16728; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::None { return 16643; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 16698; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::None { return 16874; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::None { return 16841; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 16936; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 16928; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 16630; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Tall { return 16913; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Tall { return 16926; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 16888; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 16797; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == true { return 16837; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None { return 16746; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::Low { return 16781; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::Low { return 16850; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 16714; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Tall { return 16866; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 16800; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::None { return 16847; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None { return 16832; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall { return 16933; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 16770; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true { return 16859; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true { return 16880; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 16898; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false { return 16790; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false { return 16751; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#up == true { return 16931; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::None { return 16778; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None { return 16617; }
        if block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::None { return 16676; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == true { return 16775; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 16916; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#west == West::Low { return 16812; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None { return 16745; }
        if block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false { return 16699; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 16794; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false { return 16877; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None { return 16870; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 16895; }
        if block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::None { return 16726; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#west == West::None { return 16721; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::None { return 16802; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 16777; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 16807; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Low { return 16886; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 16696; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::None { return 16624; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false { return 16719; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::None { return 16756; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 16815; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Low { return 16635; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == false { return 16902; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 16827; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#waterlogged == true { return 16736; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 16755; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Low { return 16920; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 16713; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 16680; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::None { return 16720; }
        if block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 16754; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Low { return 16772; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 16828; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#west == West::Tall { return 16831; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == true { return 16885; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low { return 16710; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 16738; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == false { return 16843; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Tall { return 16684; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::None { return 16616; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 16789; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#up == true { return 16873; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 16938; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false { return 16733; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::Tall { return 16657; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 16761; }
        if block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::None { return 16856; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 16818; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall { return 16822; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true { return 16838; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 16876; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::None { return 16730; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall { return 16934; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#west == West::Low { return 16767; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 16821; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::None { return 16668; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#up == true { return 16765; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 16639; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Low { return 16644; }
        if block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 16747; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Low { return 16701; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::None { return 16757; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 16783; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Low { return 16844; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 16927; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 16830; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false { return 16742; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 16868; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16879 {
            return Some(PrismarineWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 16700 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 16759 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 16814 {
            return Some(PrismarineWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 16858 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 16803 {
            return Some(PrismarineWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 16788 {
            return Some(PrismarineWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 16845 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 16646 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 16806 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 16641 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 16687 {
            return Some(PrismarineWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 16663 {
            return Some(PrismarineWall {
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 16825 {
            return Some(PrismarineWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 16835 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 16919 {
            return Some(PrismarineWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 16869 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 16903 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 16922 {
            return Some(PrismarineWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 16627 {
            return Some(PrismarineWall {
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 16925 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 16718 {
            return Some(PrismarineWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 16793 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 16872 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 16631 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 16640 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 16892 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 16622 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#west: West::None,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 16912 {
            return Some(PrismarineWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 16780 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 16763 {
            return Some(PrismarineWall {
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 16651 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 16768 {
            return Some(PrismarineWall {
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 16629 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 16743 {
            return Some(PrismarineWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 16839 {
            return Some(PrismarineWall {
                r#south: South::None,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 16829 {
            return Some(PrismarineWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 16679 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 16660 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 16864 {
            return Some(PrismarineWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 16650 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 16848 {
            return Some(PrismarineWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 16734 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 16749 {
            return Some(PrismarineWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 16637 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 16665 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 16678 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 16697 {
            return Some(PrismarineWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 16786 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 16732 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 16681 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 16618 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 16727 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 16842 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16851 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 16914 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 16672 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 16917 {
            return Some(PrismarineWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16833 {
            return Some(PrismarineWall {
                r#south: South::None,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 16836 {
            return Some(PrismarineWall {
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 16860 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16884 {
            return Some(PrismarineWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 16820 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 16939 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 16671 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 16688 {
            return Some(PrismarineWall {
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 16634 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 16764 {
            return Some(PrismarineWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 16753 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 16673 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 16750 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 16937 {
            return Some(PrismarineWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 16823 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 16924 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 16826 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 16862 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 16693 {
            return Some(PrismarineWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 16853 {
            return Some(PrismarineWall {
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 16861 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 16658 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 16649 {
            return Some(PrismarineWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 16654 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 16664 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 16686 {
            return Some(PrismarineWall {
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 16638 {
            return Some(PrismarineWall {
                r#up: false,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 16685 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 16725 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 16715 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 16666 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16762 {
            return Some(PrismarineWall {
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 16636 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 16620 {
            return Some(PrismarineWall {
                r#up: true,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 16834 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 16900 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 16776 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16709 {
            return Some(PrismarineWall {
                r#up: false,
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16808 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 16816 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 16817 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 16889 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16875 {
            return Some(PrismarineWall {
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 16752 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 16923 {
            return Some(PrismarineWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16887 {
            return Some(PrismarineWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 16717 {
            return Some(PrismarineWall {
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 16748 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 16705 {
            return Some(PrismarineWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16619 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 16716 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 16849 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16863 {
            return Some(PrismarineWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 16782 {
            return Some(PrismarineWall {
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 16723 {
            return Some(PrismarineWall {
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 16648 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 16904 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 16670 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 16694 {
            return Some(PrismarineWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 16674 {
            return Some(PrismarineWall {
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 16689 {
            return Some(PrismarineWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 16682 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 16813 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 16871 {
            return Some(PrismarineWall {
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 16758 {
            return Some(PrismarineWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 16784 {
            return Some(PrismarineWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 16799 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 16662 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 16690 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 16739 {
            return Some(PrismarineWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 16901 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 16810 {
            return Some(PrismarineWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16798 {
            return Some(PrismarineWall {
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16691 {
            return Some(PrismarineWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 16811 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 16703 {
            return Some(PrismarineWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 16740 {
            return Some(PrismarineWall {
                r#up: true,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 16906 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 16882 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16702 {
            return Some(PrismarineWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 16935 {
            return Some(PrismarineWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 16729 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16773 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 16840 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 16760 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 16683 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 16809 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 16779 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 16897 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 16625 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 16805 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 16695 {
            return Some(PrismarineWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 16633 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 16731 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 16647 {
            return Some(PrismarineWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 16626 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 16652 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 16708 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: false,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 16623 {
            return Some(PrismarineWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 16642 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 16704 {
            return Some(PrismarineWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 16795 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 16865 {
            return Some(PrismarineWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 16893 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 16899 {
            return Some(PrismarineWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 16905 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 16915 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 16804 {
            return Some(PrismarineWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 16656 {
            return Some(PrismarineWall {
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 16628 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 16907 {
            return Some(PrismarineWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 16769 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 16661 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 16712 {
            return Some(PrismarineWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 16771 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 16908 {
            return Some(PrismarineWall {
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 16645 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 16711 {
            return Some(PrismarineWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 16785 {
            return Some(PrismarineWall {
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 16883 {
            return Some(PrismarineWall {
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 16932 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 16857 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 16737 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 16659 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 16894 {
            return Some(PrismarineWall {
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 16741 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16677 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 16910 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 16929 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 16852 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 16896 {
            return Some(PrismarineWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 16801 {
            return Some(PrismarineWall {
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16675 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 16774 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 16878 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 16735 {
            return Some(PrismarineWall {
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16891 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 16918 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 16855 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 16854 {
            return Some(PrismarineWall {
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 16722 {
            return Some(PrismarineWall {
                r#up: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 16655 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 16911 {
            return Some(PrismarineWall {
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 16921 {
            return Some(PrismarineWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16707 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16846 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 16653 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 16632 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 16792 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 16890 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 16667 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 16824 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 16669 {
            return Some(PrismarineWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16766 {
            return Some(PrismarineWall {
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 16706 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 16930 {
            return Some(PrismarineWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16724 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 16744 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 16692 {
            return Some(PrismarineWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 16867 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 16787 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 16791 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 16881 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 16621 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 16796 {
            return Some(PrismarineWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 16819 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 16909 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 16728 {
            return Some(PrismarineWall {
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 16643 {
            return Some(PrismarineWall {
                r#up: true,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 16698 {
            return Some(PrismarineWall {
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 16874 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 16841 {
            return Some(PrismarineWall {
                r#up: false,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 16936 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 16928 {
            return Some(PrismarineWall {
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 16630 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16913 {
            return Some(PrismarineWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 16926 {
            return Some(PrismarineWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 16888 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16797 {
            return Some(PrismarineWall {
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 16837 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 16746 {
            return Some(PrismarineWall {
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 16781 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 16850 {
            return Some(PrismarineWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16714 {
            return Some(PrismarineWall {
                r#up: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 16866 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 16800 {
            return Some(PrismarineWall {
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 16847 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 16832 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 16933 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16770 {
            return Some(PrismarineWall {
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16859 {
            return Some(PrismarineWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 16880 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 16898 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 16790 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 16751 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 16931 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 16778 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 16617 {
            return Some(PrismarineWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 16676 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 16775 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 16916 {
            return Some(PrismarineWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16812 {
            return Some(PrismarineWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 16745 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 16699 {
            return Some(PrismarineWall {
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 16794 {
            return Some(PrismarineWall {
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 16877 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 16870 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 16895 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 16726 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: true,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 16721 {
            return Some(PrismarineWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 16802 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 16777 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16807 {
            return Some(PrismarineWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 16886 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 16696 {
            return Some(PrismarineWall {
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 16624 {
            return Some(PrismarineWall {
                r#up: false,
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 16719 {
            return Some(PrismarineWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 16756 {
            return Some(PrismarineWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 16815 {
            return Some(PrismarineWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 16635 {
            return Some(PrismarineWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 16902 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 16827 {
            return Some(PrismarineWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 16736 {
            return Some(PrismarineWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 16755 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 16920 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 16713 {
            return Some(PrismarineWall {
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 16680 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 16720 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 16754 {
            return Some(PrismarineWall {
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 16772 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 16828 {
            return Some(PrismarineWall {
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 16831 {
            return Some(PrismarineWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 16885 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 16710 {
            return Some(PrismarineWall {
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 16738 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16843 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 16684 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 16616 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 16789 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 16873 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 16938 {
            return Some(PrismarineWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 16733 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 16657 {
            return Some(PrismarineWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 16761 {
            return Some(PrismarineWall {
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 16856 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 16818 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 16822 {
            return Some(PrismarineWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 16838 {
            return Some(PrismarineWall {
                r#west: West::None,
                r#north: North::None,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 16876 {
            return Some(PrismarineWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 16730 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 16934 {
            return Some(PrismarineWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 16767 {
            return Some(PrismarineWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 16821 {
            return Some(PrismarineWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 16668 {
            return Some(PrismarineWall {
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 16765 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 16639 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 16644 {
            return Some(PrismarineWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 16747 {
            return Some(PrismarineWall {
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 16701 {
            return Some(PrismarineWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 16757 {
            return Some(PrismarineWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 16783 {
            return Some(PrismarineWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16844 {
            return Some(PrismarineWall {
                r#up: true,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 16927 {
            return Some(PrismarineWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 16830 {
            return Some(PrismarineWall {
                r#up: false,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 16742 {
            return Some(PrismarineWall {
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 16868 {
            return Some(PrismarineWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        return None;
    }
}

