use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateTileWall {
    pub r#north: North,
    pub r#east: East,
    pub r#south: South,
    pub waterlogged: bool,
    pub up: bool,
    pub r#west: West,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum North {
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

impl BlockState for DeepslateTileWall {
    fn to_id(&self) -> i32 {
        if self.r#up == true && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::None { return 28634; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::Tall && self.r#east == East::None && self.r#waterlogged == false && self.r#up == true { return 28732; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Tall { return 28680; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#up == false && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Low { return 28820; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::Low && self.r#north == North::Low && self.r#up == false { return 28903; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::Tall && self.r#up == false && self.r#east == East::Low && self.r#west == West::None { return 28771; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#up == true && self.r#west == West::Tall { return 28815; }
        if self.r#up == false && self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::Low && self.r#west == West::Low { return 28655; }
        if self.r#up == false && self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::Tall && self.r#north == North::Low { return 28915; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == false && self.r#south == South::Tall && self.r#west == West::None { return 28954; }
        if self.r#up == true && self.r#west == West::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Low { return 28789; }
        if self.r#east == East::Low && self.r#west == West::None && self.r#up == false && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Tall { return 28822; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Low { return 28809; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Low && self.r#west == West::Low && self.r#south == South::None && self.r#up == true { return 28670; }
        if self.r#east == East::Low && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Low { return 28784; }
        if self.r#south == South::None && self.r#up == true && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low { return 28778; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#up == false && self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Tall { return 28907; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 28761; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::Low { return 28840; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::Low { return 28943; }
        if self.r#east == East::None && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall { return 28635; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Low { return 28721; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None && self.r#south == South::None && self.r#up == false && self.r#north == North::Low { return 28675; }
        if self.r#up == false && self.r#north == North::Tall && self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == true { return 28831; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Low && self.r#up == true && self.r#west == West::None { return 28897; }
        if self.r#west == West::None && self.r#up == false && self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::Tall { return 28939; }
        if self.r#north == North::None && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::Low { return 28667; }
        if self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Low && self.r#south == South::None { return 28894; }
        if self.r#west == West::Low && self.r#up == false && self.r#north == North::None && self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == false { return 28763; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false && self.r#north == North::None { return 28772; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Tall && self.r#west == West::Low && self.r#up == false { return 28811; }
        if self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true { return 28773; }
        if self.r#south == South::Tall && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true { return 28730; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::None && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::None { return 28708; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == false && self.r#north == North::None && self.r#west == West::Low { return 28883; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::None && self.r#up == true { return 28753; }
        if self.r#east == East::Low && self.r#south == South::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#up == true { return 28777; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#west == West::Low && self.r#up == false && self.r#south == South::Tall { return 28919; }
        if self.r#east == East::Tall && self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true { return 28941; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == false && self.r#west == West::Tall && self.r#south == South::None { return 28824; }
        if self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::Tall && self.r#east == East::Tall && self.r#west == West::Tall { return 28947; }
        if self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Tall { return 28737; }
        if self.r#south == South::Low && self.r#up == false && self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == false { return 28728; }
        if self.r#west == West::Tall && self.r#waterlogged == true && self.r#east == East::None && self.r#up == false && self.r#south == South::Low && self.r#north == North::None { return 28653; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::None && self.r#up == false { return 28652; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == true { return 28802; }
        if self.r#waterlogged == true && self.r#up == true && self.r#east == East::None && self.r#west == West::None && self.r#north == North::Low && self.r#south == South::Low { return 28681; }
        if self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#south == South::None { return 28921; }
        if self.r#east == East::None && self.r#west == West::None && self.r#up == true && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == false { return 28684; }
        if self.r#south == South::Tall && self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Low && self.r#up == false { return 28952; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Tall { return 28832; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::Tall && self.r#west == West::None && self.r#up == false { return 28882; }
        if self.r#east == East::Low && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None { return 28770; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == true { return 28909; }
        if self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false { return 28895; }
        if self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Low && self.r#up == false { return 28775; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::Low { return 28757; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#up == false && self.r#west == West::None && self.r#south == South::Tall { return 28807; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == false && self.r#west == West::Tall && self.r#east == East::None && self.r#waterlogged == false { return 28740; }
        if self.r#up == false && self.r#north == North::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None { return 28783; }
        if self.r#south == South::None && self.r#north == North::None && self.r#up == true && self.r#east == East::Tall && self.r#west == West::None && self.r#waterlogged == false { return 28852; }
        if self.r#south == South::Tall && self.r#north == North::Low && self.r#west == West::None && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false { return 28912; }
        if self.r#north == North::None && self.r#east == East::None && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall { return 28644; }
        if self.r#south == South::Low && self.r#up == true && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Low { return 28758; }
        if self.r#east == East::Low && self.r#up == true && self.r#west == West::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::Low { return 28803; }
        if self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::Low && self.r#west == West::Tall { return 28902; }
        if self.r#east == East::None && self.r#north == North::None && self.r#south == South::Low && self.r#west == West::None && self.r#waterlogged == true && self.r#up == true { return 28645; }
        if self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#south == South::Tall && self.r#east == East::None && self.r#west == West::Tall { return 28701; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Tall { return 28797; }
        if self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::Low && self.r#west == West::Tall { return 28788; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Tall && self.r#up == false && self.r#west == West::None { return 28879; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low { return 28808; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low { return 28781; }
        if self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == true { return 28697; }
        if self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Tall && self.r#north == North::Tall { return 28956; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None && self.r#south == South::None && self.r#north == North::Tall && self.r#up == false { return 28714; }
        if self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::None && self.r#west == West::None { return 28666; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::Tall && self.r#up == true { return 28734; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#up == true && self.r#east == East::Tall && self.r#north == North::Tall && self.r#waterlogged == false { return 28949; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::None && self.r#up == true { return 28861; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == true && self.r#east == East::Low && self.r#west == West::None && self.r#north == North::Tall { return 28837; }
        if self.r#up == true && self.r#south == South::None && self.r#west == West::Low && self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::None { return 28853; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Low && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false { return 28798; }
        if self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Tall && self.r#east == East::None && self.r#up == false { return 28738; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::Low && self.r#east == East::None && self.r#west == West::Tall && self.r#up == false { return 28677; }
        if self.r#south == South::Low && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall { return 28834; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::None && self.r#up == false { return 28870; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Tall && self.r#up == false && self.r#south == South::Tall { return 28739; }
        if self.r#up == true && self.r#east == East::Low && self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::Tall && self.r#waterlogged == false { return 28842; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == false && self.r#south == South::Low { return 28908; }
        if self.r#west == West::Tall && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Tall { return 28920; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::Tall && self.r#north == North::Tall && self.r#up == true { return 28719; }
        if self.r#east == East::Low && self.r#up == false && self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::Tall && self.r#waterlogged == false { return 28846; }
        if self.r#west == West::Low && self.r#north == North::None && self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == true { return 28661; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::None && self.r#up == true && self.r#north == North::Tall && self.r#south == South::None { return 28813; }
        if self.r#up == false && self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::Tall { return 28953; }
        if self.r#up == false && self.r#west == West::Low && self.r#waterlogged == true && self.r#south == South::Low && self.r#east == East::None && self.r#north == North::Low { return 28688; }
        if self.r#south == South::None && self.r#up == true && self.r#west == West::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Tall { return 28851; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::Tall { return 28729; }
        if self.r#west == West::Low && self.r#south == South::None && self.r#east == East::Low && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == true { return 28814; }
        if self.r#west == West::Low && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Tall && self.r#east == East::None { return 28733; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#up == false && self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::None { return 28641; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#up == false && self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::Tall { return 28942; }
        if self.r#west == West::None && self.r#up == false && self.r#south == South::Tall && self.r#east == East::None && self.r#waterlogged == true && self.r#north == North::None { return 28663; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#up == false && self.r#south == South::Low && self.r#north == North::None && self.r#waterlogged == true { return 28760; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None { return 28872; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#up == true && self.r#north == North::Tall && self.r#west == West::Tall { return 28926; }
        if self.r#north == North::None && self.r#up == true && self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::Tall { return 28647; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::None && self.r#east == East::None && self.r#up == true && self.r#north == North::Low { return 28674; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == true { return 28839; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Low { return 28700; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == true && self.r#east == East::None && self.r#waterlogged == false { return 28720; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::None && self.r#east == East::Low { return 28787; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#up == true && self.r#south == South::Tall && self.r#north == North::Low && self.r#waterlogged == false { return 28698; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true { return 28869; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None && self.r#up == true { return 28658; }
        if self.r#waterlogged == false && self.r#south == South::Low && self.r#north == North::None && self.r#up == false && self.r#east == East::Tall && self.r#west == West::Low { return 28871; }
        if self.r#up == false && self.r#north == North::Low && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Low { return 28691; }
        if self.r#east == East::Tall && self.r#up == false && self.r#west == West::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::None { return 28859; }
        if self.r#up == true && self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::None && self.r#east == East::Tall { return 28887; }
        if self.r#east == East::None && self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true { return 28723; }
        if self.r#up == false && self.r#north == North::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::None { return 28785; }
        if self.r#north == North::Low && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None && self.r#up == true { return 28669; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None && self.r#south == South::Low { return 28717; }
        if self.r#east == East::Low && self.r#south == South::None && self.r#west == West::Low && self.r#waterlogged == false && self.r#up == true && self.r#north == North::None { return 28745; }
        if self.r#east == East::Tall && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == true { return 28875; }
        if self.r#north == North::Low && self.r#east == East::None && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low { return 28679; }
        if self.r#west == West::None && self.r#north == North::None && self.r#east == East::Low && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false { return 28744; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#north == North::None && self.r#up == true && self.r#west == West::None && self.r#south == South::Tall { return 28660; }
        if self.r#south == South::None && self.r#up == false && self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::None { return 28750; }
        if self.r#east == East::Low && self.r#up == false && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Low { return 28759; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Tall { return 28810; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == false { return 28835; }
        if self.r#waterlogged == false && self.r#up == false && self.r#west == West::None && self.r#north == North::Low && self.r#east == East::None && self.r#south == South::None { return 28678; }
        if self.r#up == true && self.r#south == South::None && self.r#north == North::Low && self.r#west == West::Tall && self.r#waterlogged == false && self.r#east == East::Tall { return 28890; }
        if self.r#east == East::Tall && self.r#west == West::None && self.r#north == North::None && self.r#south == South::None && self.r#up == true && self.r#waterlogged == true { return 28849; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::Tall { return 28862; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#east == East::Tall && self.r#up == true && self.r#west == West::None && self.r#waterlogged == true { return 28885; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Low { return 28769; }
        if self.r#up == true && self.r#east == East::Low && self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::Low { return 28801; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None && self.r#up == false && self.r#north == North::Tall { return 28930; }
        if self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None { return 28933; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == true && self.r#south == South::Tall && self.r#west == West::Low { return 28946; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low { return 28793; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low { return 28703; }
        if self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Tall && self.r#west == West::Low { return 28916; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == true && self.r#south == South::None { return 28671; }
        if self.r#up == false && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::None { return 28643; }
        if self.r#up == false && self.r#east == East::Tall && self.r#west == West::Tall && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::None { return 28860; }
        if self.r#south == South::None && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false { return 28715; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall { return 28950; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#up == false && self.r#north == North::None && self.r#west == West::Tall && self.r#south == South::Tall { return 28668; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == true { return 28889; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::None && self.r#south == South::Low { return 28650; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::None && self.r#up == false && self.r#east == East::None && self.r#waterlogged == false { return 28656; }
        if self.r#west == West::Tall && self.r#up == true && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::Low && self.r#north == North::None { return 28866; }
        if self.r#west == West::Tall && self.r#up == false && self.r#south == South::None && self.r#north == North::Low && self.r#east == East::Tall && self.r#waterlogged == false { return 28896; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#west == West::Low && self.r#up == true && self.r#north == North::Low && self.r#waterlogged == true { return 28886; }
        if self.r#west == West::Tall && self.r#south == South::None && self.r#east == East::None && self.r#north == North::None && self.r#up == true && self.r#waterlogged == false { return 28638; }
        if self.r#east == East::Tall && self.r#up == false && self.r#west == West::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::None { return 28856; }
        if self.r#north == North::Low && self.r#up == true && self.r#south == South::Low && self.r#west == West::None && self.r#east == East::Tall && self.r#waterlogged == false { return 28900; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall && self.r#up == true && self.r#south == South::Tall { return 28945; }
        if self.r#south == South::None && self.r#east == East::None && self.r#north == North::Tall && self.r#west == West::Low && self.r#waterlogged == false && self.r#up == true { return 28709; }
        if self.r#north == North::None && self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == true && self.r#up == false && self.r#west == West::None { return 28651; }
        if self.r#up == false && self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::Low { return 28799; }
        if self.r#south == South::Low && self.r#west == West::Tall && self.r#east == East::None && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == false { return 28722; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#up == true && self.r#west == West::Low && self.r#south == South::None && self.r#east == East::None { return 28706; }
        if self.r#north == North::None && self.r#up == true && self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None { return 28657; }
        if self.r#north == North::Low && self.r#west == West::Low && self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Low { return 28682; }
        if self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#south == South::Tall && self.r#north == North::Low && self.r#west == West::None { return 28696; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#south == South::Tall && self.r#up == true && self.r#east == East::None && self.r#waterlogged == false { return 28662; }
        if self.r#north == North::None && self.r#south == South::None && self.r#up == true && self.r#west == West::None && self.r#east == East::Low && self.r#waterlogged == true { return 28741; }
        if self.r#south == South::None && self.r#north == North::Low && self.r#up == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#waterlogged == false { return 28782; }
        if self.r#up == false && self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == false { return 28727; }
        if self.r#east == East::Tall && self.r#up == true && self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::None { return 28873; }
        if self.r#north == North::Low && self.r#up == false && self.r#east == East::Tall && self.r#south == South::Tall && self.r#west == West::Tall && self.r#waterlogged == true { return 28917; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Low { return 28841; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::None && self.r#up == false && self.r#south == South::Low && self.r#east == East::Tall { return 28867; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None && self.r#south == South::None && self.r#up == false && self.r#east == East::Tall { return 28858; }
        if self.r#up == true && self.r#waterlogged == false && self.r#north == North::Low && self.r#south == South::None && self.r#east == East::Tall && self.r#west == West::None { return 28888; }
        if self.r#up == true && self.r#south == South::None && self.r#east == East::None && self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::Low { return 28672; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == false && self.r#east == East::None && self.r#north == North::Low { return 28692; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == false && self.r#east == East::Low && self.r#south == South::Tall { return 28848; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Low && self.r#up == true && self.r#south == South::Tall && self.r#east == East::Tall { return 28913; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == true && self.r#west == West::None { return 28765; }
        if self.r#up == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == true { return 28664; }
        if self.r#south == South::Tall && self.r#north == North::None && self.r#up == true && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::None { return 28768; }
        if self.r#west == West::None && self.r#south == South::None && self.r#east == East::Tall && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true { return 28927; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Tall && self.r#west == West::Tall && self.r#up == false && self.r#south == South::None { return 28932; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#up == true && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::None { return 28649; }
        if self.r#south == South::Low && self.r#west == West::Low && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::Tall { return 28934; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true && self.r#south == South::Tall { return 28845; }
        if self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#east == East::None { return 28646; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None && self.r#east == East::Low && self.r#up == false && self.r#south == South::None { return 28748; }
        if self.r#up == false && self.r#north == North::None && self.r#south == South::None && self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Low { return 28751; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::None && self.r#south == South::None { return 28816; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false { return 28940; }
        if self.r#north == North::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Tall { return 28774; }
        if self.r#up == true && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::None { return 28685; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::None { return 28693; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#up == false && self.r#west == West::Low && self.r#east == East::None && self.r#waterlogged == true { return 28712; }
        if self.r#east == East::Low && self.r#west == West::Tall && self.r#up == true && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::None { return 28746; }
        if self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::None && self.r#west == West::Tall { return 28863; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#up == true && self.r#east == East::None && self.r#south == South::Low && self.r#west == West::None { return 28648; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::None && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false { return 28752; }
        if self.r#up == false && self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Tall { return 28821; }
        if self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 28665; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Tall { return 28847; }
        if self.r#up == false && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::Low { return 28796; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Low && self.r#up == true && self.r#south == South::None && self.r#east == East::Tall { return 28850; }
        if self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == false && self.r#north == North::None && self.r#east == East::Tall { return 28857; }
        if self.r#south == South::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Tall { return 28704; }
        if self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Tall && self.r#north == North::Tall { return 28731; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::None && self.r#up == false { return 28762; }
        if self.r#north == North::Tall && self.r#up == true && self.r#south == South::Low && self.r#west == West::Low && self.r#waterlogged == true && self.r#east == East::Low { return 28826; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#waterlogged == true && self.r#up == true && self.r#north == North::Tall && self.r#east == East::Low { return 28827; }
        if self.r#north == North::Low && self.r#up == false && self.r#south == South::Tall && self.r#east == East::None && self.r#west == West::None && self.r#waterlogged == false { return 28702; }
        if self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::None { return 28736; }
        if self.r#east == East::Tall && self.r#up == true && self.r#north == North::None && self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::Low { return 28864; }
        if self.r#up == true && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::None { return 28673; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low { return 28880; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::None && self.r#north == North::None && self.r#waterlogged == true && self.r#up == true { return 28742; }
        if self.r#east == East::Low && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Tall { return 28838; }
        if self.r#up == true && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Low { return 28686; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low { return 28905; }
        if self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Tall && self.r#north == North::Tall && self.r#up == true { return 28707; }
        if self.r#east == East::Tall && self.r#west == West::None && self.r#north == North::Low && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false { return 28906; }
        if self.r#south == South::None && self.r#waterlogged == true && self.r#up == false && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Low { return 28676; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == false { return 28764; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Low && self.r#east == East::Tall && self.r#up == true { return 28877; }
        if self.r#west == West::Tall && self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true { return 28881; }
        if self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#south == South::Low { return 28935; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Tall && self.r#west == West::Low && self.r#east == East::Tall && self.r#up == true { return 28910; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#south == South::Tall && self.r#west == West::None && self.r#waterlogged == false { return 28948; }
        if self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Low && self.r#south == South::None { return 28922; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::None && self.r#up == false && self.r#east == East::Tall { return 28892; }
        if self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::None { return 28818; }
        if self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None && self.r#up == false { return 28639; }
        if self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Tall && self.r#north == North::None && self.r#west == West::Tall && self.r#up == false { return 28776; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Tall && self.r#up == false { return 28812; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None { return 28854; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#east == East::Low && self.r#up == true && self.r#west == West::None && self.r#waterlogged == false { return 28780; }
        if self.r#south == South::Low && self.r#up == true && self.r#north == North::None && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Low { return 28865; }
        if self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::None && self.r#north == North::Tall { return 28713; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#up == true && self.r#east == East::None && self.r#south == South::Low { return 28718; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == true && self.r#west == West::None { return 28936; }
        if self.r#west == West::Low && self.r#up == true && self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::Tall { return 28901; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#up == true { return 28792; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#south == South::Low && self.r#west == West::Tall && self.r#waterlogged == false { return 28938; }
        if self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Tall { return 28951; }
        if self.r#up == false && self.r#west == West::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Low { return 28689; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true { return 28735; }
        if self.r#waterlogged == true && self.r#up == true && self.r#north == North::None && self.r#west == West::None && self.r#south == South::None && self.r#east == East::None { return 28633; }
        if self.r#south == South::None && self.r#east == East::None && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None { return 28711; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Low && self.r#west == West::Low && self.r#north == North::Low && self.r#up == true { return 28805; }
        if self.r#up == false && self.r#south == South::Low && self.r#east == East::Tall && self.r#west == West::Low && self.r#waterlogged == true && self.r#north == North::Low { return 28904; }
        if self.r#east == East::Tall && self.r#west == West::Low && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Tall { return 28925; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None && self.r#up == true && self.r#south == South::Tall { return 28876; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::Tall && self.r#south == South::None && self.r#north == North::Tall && self.r#up == true { return 28710; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#up == true && self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::Low { return 28790; }
        if self.r#east == East::Low && self.r#up == true && self.r#west == West::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::None { return 28743; }
        if self.r#east == East::None && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::None { return 28637; }
        if self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::Tall { return 28766; }
        if self.r#east == East::Low && self.r#up == true && self.r#west == West::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#north == North::Low { return 28806; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::Low && self.r#east == East::Tall && self.r#up == true && self.r#south == South::Low { return 28898; }
        if self.r#up == true && self.r#east == East::Tall && self.r#south == South::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Tall { return 28923; }
        if self.r#west == West::Low && self.r#east == East::None && self.r#up == true && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == true { return 28694; }
        if self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Low && self.r#south == South::Low { return 28754; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Tall && self.r#up == false && self.r#east == East::Tall && self.r#south == South::None { return 28929; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Low && self.r#up == false && self.r#west == West::None && self.r#east == East::Low { return 28795; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == true && self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::Low { return 28914; }
        if self.r#south == South::None && self.r#east == East::Low && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::None { return 28786; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall { return 28791; }
        if self.r#east == East::None && self.r#up == false && self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == false { return 28716; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == true && self.r#east == East::Low && self.r#west == West::None { return 28828; }
        if self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true { return 28695; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#up == true && self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::Tall { return 28924; }
        if self.r#south == South::Tall && self.r#west == West::Tall && self.r#up == true && self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == true { return 28767; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#up == false && self.r#west == West::Low && self.r#north == North::Tall { return 28928; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#waterlogged == true && self.r#up == false && self.r#west == West::Tall && self.r#east == East::None { return 28725; }
        if self.r#north == North::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None && self.r#east == East::None { return 28640; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Low { return 28899; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Low { return 28800; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == false && self.r#west == West::None && self.r#north == North::Low && self.r#south == South::Tall { return 28918; }
        if self.r#west == West::Tall && self.r#up == false && self.r#north == North::None && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Tall { return 28884; }
        if self.r#south == South::None && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::Tall && self.r#waterlogged == false && self.r#up == false { return 28931; }
        if self.r#up == true && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::Tall && self.r#east == East::Low { return 28830; }
        if self.r#north == North::None && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None { return 28747; }
        if self.r#west == West::Tall && self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Low { return 28893; }
        if self.r#east == East::Low && self.r#up == true && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Low { return 28779; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None && self.r#up == false && self.r#south == South::Low { return 28687; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#up == false && self.r#south == South::Low && self.r#waterlogged == true && self.r#east == East::None { return 28724; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == true && self.r#south == South::Tall { return 28878; }
        if self.r#south == South::Tall && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Tall { return 28843; }
        if self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::Low && self.r#up == false && self.r#north == North::Tall && self.r#east == East::None { return 28726; }
        if self.r#up == false && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::Low { return 28836; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::None && self.r#up == false { return 28654; }
        if self.r#up == true && self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Low { return 28817; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false { return 28868; }
        if self.r#north == North::Tall && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true { return 28833; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::None && self.r#up == false && self.r#west == West::None { return 28690; }
        if self.r#east == East::Tall && self.r#up == false && self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::Tall && self.r#waterlogged == false { return 28944; }
        if self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::None && self.r#west == West::None { return 28636; }
        if self.r#south == South::None && self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Tall && self.r#west == West::None { return 28705; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == true { return 28874; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == false && self.r#north == North::Tall && self.r#west == West::Low && self.r#waterlogged == false { return 28955; }
        if self.r#waterlogged == false && self.r#south == South::Low && self.r#up == true && self.r#north == North::Low && self.r#west == West::Tall && self.r#east == East::Low { return 28794; }
        if self.r#south == South::None && self.r#west == West::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low { return 28823; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Low { return 28699; }
        if self.r#north == North::Low && self.r#up == true && self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == true { return 28683; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None { return 28825; }
        if self.r#waterlogged == true && self.r#up == false && self.r#south == South::None && self.r#east == East::Tall && self.r#west == West::None && self.r#north == North::None { return 28855; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true && self.r#south == South::Tall && self.r#east == East::Tall { return 28911; }
        if self.r#west == West::None && self.r#up == true && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == false { return 28804; }
        if self.r#west == West::None && self.r#waterlogged == false && self.r#up == false && self.r#south == South::None && self.r#east == East::None && self.r#north == North::None { return 28642; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#up == false && self.r#south == South::None && self.r#east == East::Tall && self.r#west == West::None { return 28891; }
        if self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::None { return 28755; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::None && self.r#east == East::Low && self.r#up == true { return 28756; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::Low && self.r#up == true { return 28937; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == true { return 28844; }
        if self.r#west == West::Tall && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::None { return 28749; }
        if self.r#west == West::None && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true { return 28819; }
        if self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall { return 28659; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == true { return 28829; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 28634 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 28732 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 28680 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 28820 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 28903 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 28771 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 28815 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28655 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28915 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 28954 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 28789 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28822 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#west: West::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 28809 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 28670 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 28784 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 28778 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 28907 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 28761 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28840 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 28943 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28635 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28721 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 28675 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 28831 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28897 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 28939 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 28667 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 28894 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 28763 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28772 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 28811 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 28773 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 28730 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28708 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28883 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 28753 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 28777 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 28919 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 28941 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 28824 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 28947 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28737 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28728 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28653 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 28652 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 28802 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 28681 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28921 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 28684 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28952 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 28832 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 28882 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 28770 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 28909 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 28895 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28775 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 28757 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28807 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28740 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28783 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 28852 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28912 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 28644 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28758 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 28803 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 28902 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 28645 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 28701 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 28797 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 28788 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 28879 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 28808 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 28781 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28697 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 28956 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 28714 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 28666 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 28734 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 28949 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28861 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 28837 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 28853 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 28798 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28738 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 28677 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 28834 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 28870 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 28739 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 28842 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28908 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 28920 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 28719 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 28846 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28661 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 28813 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 28953 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 28688 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 28851 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 28729 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28814 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28733 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 28641 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 28942 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 28663 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#up: false,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 28760 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28872 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 28926 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28647 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28674 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 28839 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 28700 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 28720 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28787 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 28698 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28869 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 28658 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 28871 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 28691 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28859 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 28887 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 28723 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 28785 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 28669 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 28717 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 28745 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 28875 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 28679 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28744 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 28660 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28750 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28759 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 28810 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28835 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28678 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 28890 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 28849 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28862 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 28885 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28769 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 28801 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 28930 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 28933 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 28946 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 28793 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28703 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28916 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 28671 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 28643 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 28860 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 28715 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28950 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28668 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28889 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 28650 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 28656 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28866 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 28896 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28886 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28638 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 28856 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 28900 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28945 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 28709 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 28651 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 28799 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28722 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28706 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 28657 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 28682 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 28696 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 28662 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28741 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28782 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28727 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28873 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 28917 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28841 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 28867 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
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
        if state_id == 28888 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 28672 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#south: South::None,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 28692 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 28848 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28913 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 28765 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 28664 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28768 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 28927 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28932 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 28649 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 28934 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 28845 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 28646 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 28748 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 28751 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 28816 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 28940 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 28774 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28685 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 28693 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 28712 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28746 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 28863 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 28648 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 28752 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28821 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28665 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28847 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 28796 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 28850 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 28857 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 28704 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 28731 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 28762 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 28826 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 28827 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 28702 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28736 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 28864 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 28673 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 28880 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 28742 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 28838 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28686 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 28905 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28707 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 28906 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28676 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 28764 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 28877 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 28881 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 28935 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 28910 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 28948 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28922 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 28892 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 28818 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 28639 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 28776 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 28812 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 28854 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 28780 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28865 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28713 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 28718 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 28936 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 28901 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 28792 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 28938 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28951 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 28689 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 28735 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 28633 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#up: true,
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 28711 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 28805 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 28904 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 28925 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 28876 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 28710 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 28790 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 28743 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 28637 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 28766 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28806 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 28898 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 28923 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28694 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28754 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28929 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 28795 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 28914 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 28786 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28791 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28716 {
            return Some(DeepslateTileWall {
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28828 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 28695 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28924 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 28767 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28928 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 28725 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 28640 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 28899 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 28800 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 28918 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28884 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#up: false,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 28931 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 28830 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 28747 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 28893 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 28779 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 28687 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 28724 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 28878 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 28843 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 28726 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 28836 {
            return Some(DeepslateTileWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28654 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 28817 {
            return Some(DeepslateTileWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 28868 {
            return Some(DeepslateTileWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 28833 {
            return Some(DeepslateTileWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 28690 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 28944 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28636 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 28705 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 28874 {
            return Some(DeepslateTileWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 28955 {
            return Some(DeepslateTileWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28794 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 28823 {
            return Some(DeepslateTileWall {
                r#south: South::None,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 28699 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 28683 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28825 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 28855 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 28911 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 28804 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28642 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 28891 {
            return Some(DeepslateTileWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 28755 {
            return Some(DeepslateTileWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 28756 {
            return Some(DeepslateTileWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 28937 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 28844 {
            return Some(DeepslateTileWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 28749 {
            return Some(DeepslateTileWall {
                r#west: West::Tall,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 28819 {
            return Some(DeepslateTileWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 28659 {
            return Some(DeepslateTileWall {
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28829 {
            return Some(DeepslateTileWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        return None;
    }
}

