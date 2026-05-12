use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackstoneWall {
    pub waterlogged: bool,
    pub r#east: East,
    pub r#west: West,
    pub r#north: North,
    pub r#south: South,
    pub up: bool,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum South {
    None,
    Low,
    Tall,
}

impl BlockState for BlackstoneWall {
    fn to_id(self) -> i32 {
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 21797; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == false { return 21953; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::None { return 21898; }
        if block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::None { return 21845; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None { return 21754; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Low { return 21894; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::Tall { return 21934; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 21839; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 21791; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 21768; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::None { return 21793; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::None { return 21972; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == false { return 21780; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Tall { return 21925; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall { return 21950; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::None { return 21942; }
        if block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 21960; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 21879; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::None { return 21884; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 21969; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::Tall { return 21920; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 21736; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Low { return 21867; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#up == false { return 21971; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Tall { return 21977; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 21718; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#waterlogged == true { return 21711; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 21760; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false { return 21841; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == true { return 21786; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 21868; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 21883; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::None { return 21896; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::Low { return 21842; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Low { return 21911; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Low { return 21840; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#up == false { return 21826; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false { return 21846; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Tall { return 21782; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::None { return 21799; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == true { return 21854; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 21936; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::None { return 21728; }
        if block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false { return 21829; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 21837; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 21877; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true { return 21974; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 22030; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Tall { return 22019; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 21964; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::Tall { return 21751; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false { return 21778; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Low { return 21866; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 21948; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::Tall { return 21968; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 21970; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 22020; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 21916; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 21820; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 21918; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::Low { return 21825; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 21919; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 21834; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::None { return 21737; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 21905; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 21874; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 21886; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None { return 21952; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::None { return 21821; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 21865; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Low { return 21724; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == false { return 21756; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Low { return 21945; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::None { return 21860; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 21853; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 21890; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 21981; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 21921; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Low { return 21857; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Tall { return 21796; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 21917; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 21855; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 21849; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == false { return 21929; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 21980; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false { return 21727; }
        if block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 21859; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 21915; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 21876; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false { return 21887; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::None { return 21872; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 21927; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None { return 21931; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 21951; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == false { return 21946; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::None { return 21741; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall { return 21895; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::None { return 21933; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Tall { return 21816; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 21899; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 21755; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 21771; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 21712; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true { return 21810; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == true { return 21880; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 22024; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None { return 21713; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#waterlogged == true { return 21710; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::Low { return 21729; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 21750; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::None { return 21947; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 21814; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Tall { return 21932; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Tall { return 21815; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::None { return 21944; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Low { return 21776; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Tall { return 22005; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::Tall { return 22010; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Low { return 21892; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low { return 21897; }
        if block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 21978; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#waterlogged == false { return 21738; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::None { return 21723; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 21901; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::None { return 21989; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Low { return 21835; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 21976; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false { return 22009; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false { return 21982; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Low { return 21966; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall { return 21913; }
        if block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::Tall { return 21733; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 21959; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::None { return 21809; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::Low { return 21878; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == false { return 21769; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::None { return 22007; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall { return 22018; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::Tall { return 21848; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::None { return 21770; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 21988; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 21956; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low { return 21871; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Low { return 21940; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::None { return 21714; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::None { return 21935; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Tall { return 21949; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Tall { return 21844; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::Tall { return 21784; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 21808; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::Tall { return 21838; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::None { return 21998; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false { return 21717; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#up == true { return 21926; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == true { return 21806; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 21790; }
        if block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == false { return 21858; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 22026; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Low { return 21759; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 21761; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 21900; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false { return 22029; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 21803; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall { return 21805; }
        if block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 21836; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::Tall { return 22021; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 22004; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#north == North::Low { return 21979; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 21888; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::Low { return 21758; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == false { return 21996; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 22028; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == true { return 21785; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == true { return 21938; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == true { return 22013; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#waterlogged == false { return 21726; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None { return 21742; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::None { return 21747; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == false { return 21773; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::Tall { return 21994; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Tall { return 21772; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 21745; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None { return 21833; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 21910; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#waterlogged == true { return 21740; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Tall { return 21739; }
        if block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == true { return 21823; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Low { return 21939; }
        if block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 21967; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::Low { return 21973; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 22008; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None { return 21722; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 21817; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#up == true { return 21870; }
        if block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 21957; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 21822; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 21986; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == true { return 22014; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == false { return 21720; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 22017; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 21914; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 21943; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true { return 21795; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 21997; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Tall { return 22003; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 21732; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::None { return 21830; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::None { return 21765; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 21885; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 22027; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false { return 21719; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::None { return 21891; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#up == false { return 21802; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::None { return 21764; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 21862; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 21904; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::None { return 21824; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::None { return 21725; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::None { return 21923; }
        if block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::Low { return 21881; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 21965; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 21813; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Tall { return 21847; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 21908; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#west == West::Tall { return 21763; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true { return 22000; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 22011; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall { return 21922; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 21777; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 22033; }
        if block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == true { return 21902; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == true { return 21794; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#west == West::Tall { return 21832; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::None { return 21752; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None { return 21930; }
        if block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 21963; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 21843; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false { return 21721; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::None { return 21851; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 22001; }
        if block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 21990; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None { return 21928; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::Tall { return 22002; }
        if block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::None { return 21779; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Low { return 21818; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false { return 21801; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall { return 21807; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 21828; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Tall { return 22016; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::Low { return 21907; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::None { return 21749; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Low { return 21767; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 21961; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None { return 21716; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::None { return 21757; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::None { return 21766; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Low { return 21985; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == true { return 21811; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::Low { return 21748; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#waterlogged == false { return 21863; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 22022; }
        if block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 21873; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 22025; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false { return 21715; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None { return 21941; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::None { return 21743; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Low { return 21852; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Low { return 21731; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low { return 21861; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false { return 21864; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None { return 21893; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Low { return 21954; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 21958; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Low { return 21744; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == true { return 22023; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false { return 21983; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Low { return 21774; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::None { return 21831; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low { return 21987; }
        if block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == true { return 21762; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 21995; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 21924; }
        if block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 21984; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None { return 21783; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 22032; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Low { return 21735; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::None { return 21734; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::None { return 21798; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::None { return 21812; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 21781; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 21788; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == false { return 21792; }
        if block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 21955; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true { return 21991; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Tall { return 21787; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == false { return 21800; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::None { return 21804; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 21850; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Low { return 21869; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Tall { return 21789; }
        if block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 21875; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None { return 21819; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Low { return 21906; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false { return 21937; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 21992; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 21889; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Low { return 21975; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::None { return 21746; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true { return 21753; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall { return 21999; }
        if block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 22031; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 21882; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 21903; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::None { return 21827; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 21993; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Low { return 21909; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Tall { return 22006; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::Tall { return 22012; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall { return 22015; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::Tall { return 21730; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::Low { return 21912; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == true { return 21962; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::Low { return 21856; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 21775; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21797 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 21953 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 21898 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 21845 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 21754 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 21894 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 21934 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 21839 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 21791 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 21768 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 21793 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 21972 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 21780 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 21925 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 21950 {
            return Some(BlackstoneWall {
                r#up: true,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 21942 {
            return Some(BlackstoneWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 21960 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 21879 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 21884 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 21969 {
            return Some(BlackstoneWall {
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 21920 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 21736 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 21867 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 21971 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 21977 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 21718 {
            return Some(BlackstoneWall {
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21711 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 21760 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 21841 {
            return Some(BlackstoneWall {
                r#up: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 21786 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 21868 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 21883 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 21896 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 21842 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 21911 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 21840 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 21826 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 21846 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 21782 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 21799 {
            return Some(BlackstoneWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 21854 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 21936 {
            return Some(BlackstoneWall {
                r#up: false,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 21728 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 21829 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 21837 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 21877 {
            return Some(BlackstoneWall {
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 21974 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 22030 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22019 {
            return Some(BlackstoneWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 21964 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 21751 {
            return Some(BlackstoneWall {
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 21778 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 21866 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 21948 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 21968 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 21970 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 22020 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 21916 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 21820 {
            return Some(BlackstoneWall {
                r#up: true,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 21918 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 21825 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 21919 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 21834 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 21737 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 21905 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 21874 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 21886 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 21952 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 21821 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#up: true,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 21865 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 21724 {
            return Some(BlackstoneWall {
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 21756 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 21945 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 21860 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#up: false,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 21853 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 21890 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 21981 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 21921 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 21857 {
            return Some(BlackstoneWall {
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 21796 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 21917 {
            return Some(BlackstoneWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 21855 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 21849 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 21929 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 21980 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 21727 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 21859 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 21915 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 21876 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 21887 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 21872 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 21927 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 21931 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 21951 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 21946 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 21741 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 21895 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 21933 {
            return Some(BlackstoneWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 21816 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 21899 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 21755 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 21771 {
            return Some(BlackstoneWall {
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 21712 {
            return Some(BlackstoneWall {
                r#up: true,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21810 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 21880 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 22024 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 21713 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#south: South::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 21710 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 21729 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 21750 {
            return Some(BlackstoneWall {
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 21947 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 21814 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 21932 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 21815 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 21944 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 21776 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 22005 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 22010 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 21892 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 21897 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 21978 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 21738 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 21723 {
            return Some(BlackstoneWall {
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 21901 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 21989 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 21835 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 21976 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 22009 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 21982 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 21966 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 21913 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 21733 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 21959 {
            return Some(BlackstoneWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 21809 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 21878 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 21769 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 22007 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 22018 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 21848 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 21770 {
            return Some(BlackstoneWall {
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 21988 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21956 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 21871 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 21940 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 21714 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 21935 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 21949 {
            return Some(BlackstoneWall {
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 21844 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21784 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 21808 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 21838 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 21998 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 21717 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 21926 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 21806 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 21790 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21858 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22026 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 21759 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 21761 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 21900 {
            return Some(BlackstoneWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 22029 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 21803 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 21805 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 21836 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 22021 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 22004 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 21979 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 21888 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 21758 {
            return Some(BlackstoneWall {
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 21996 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 22028 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 21785 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 21938 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22013 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 21726 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 21742 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 21747 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 21773 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 21994 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 21772 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21745 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 21833 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 21910 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 21740 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 21739 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21823 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 21939 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 21967 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 21973 {
            return Some(BlackstoneWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 22008 {
            return Some(BlackstoneWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 21722 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 21817 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 21870 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 21957 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 21822 {
            return Some(BlackstoneWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 21986 {
            return Some(BlackstoneWall {
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22014 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 21720 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 22017 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 21914 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 21943 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 21795 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 21997 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 22003 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21732 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 21830 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 21765 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 21885 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 22027 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 21719 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 21891 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 21802 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 21764 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 21862 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21904 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 21824 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 21725 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 21923 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 21881 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 21965 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 21813 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 21847 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 21908 {
            return Some(BlackstoneWall {
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 21763 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 22000 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 22011 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 21922 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 21777 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 22033 {
            return Some(BlackstoneWall {
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 21902 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 21794 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 21832 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 21752 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 21930 {
            return Some(BlackstoneWall {
                r#up: true,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 21963 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 21843 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 21721 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 21851 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 22001 {
            return Some(BlackstoneWall {
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 21990 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 21928 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 22002 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 21779 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 21818 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 21801 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 21807 {
            return Some(BlackstoneWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 21828 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#up: false,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 22016 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 21907 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 21749 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 21767 {
            return Some(BlackstoneWall {
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 21961 {
            return Some(BlackstoneWall {
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 21716 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 21757 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 21766 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 21985 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 21811 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 21748 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 21863 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22022 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 21873 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22025 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 21715 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 21941 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 21743 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 21852 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 21731 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 21861 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 21864 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 21893 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 21954 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 21958 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21744 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 22023 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 21983 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 21774 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 21831 {
            return Some(BlackstoneWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 21987 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 21762 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 21995 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 21924 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 21984 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 21783 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 22032 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 21735 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 21734 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 21798 {
            return Some(BlackstoneWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 21812 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 21781 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 21788 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 21792 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 21955 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 21991 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 21787 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 21800 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 21804 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 21850 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 21869 {
            return Some(BlackstoneWall {
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 21789 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 21875 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 21819 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 21906 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 21937 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 21992 {
            return Some(BlackstoneWall {
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 21889 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 21975 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 21746 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 21753 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 21999 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 22031 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 21882 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 21903 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 21827 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 21993 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 21909 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 22006 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 22012 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 22015 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21730 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 21912 {
            return Some(BlackstoneWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 21962 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 21856 {
            return Some(BlackstoneWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 21775 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

