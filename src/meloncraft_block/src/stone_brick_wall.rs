use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoneBrickWall {
    pub r#west: West,
    pub r#east: East,
    pub waterlogged: bool,
    pub up: bool,
    pub r#south: South,
    pub r#north: North,
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
pub enum South {
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

impl BlockState for StoneBrickWall {
    fn to_id(&self) -> i32 {
        if self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Tall && self.r#up == true { return 18179; }
        if self.r#north == North::Low && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false { return 17982; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::None && self.r#north == North::Tall { return 17994; }
        if self.r#north == North::None && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::None && self.r#up == true && self.r#south == South::Low { return 17927; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#up == true && self.r#south == South::None && self.r#west == West::Low && self.r#waterlogged == true { return 18093; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#up == false && self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Tall { return 18186; }
        if self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Tall && self.r#west == West::Tall { return 18112; }
        if self.r#south == South::Low && self.r#up == true && self.r#north == North::Low && self.r#west == West::Tall && self.r#waterlogged == true && self.r#east == East::Tall { return 18178; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::None { return 18153; }
        if self.r#east == East::Tall && self.r#up == false && self.r#south == South::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Low { return 18150; }
        if self.r#north == North::Low && self.r#west == West::Tall && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Low { return 18187; }
        if self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::None && self.r#up == true { return 17985; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::None { return 18071; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true && self.r#south == South::Low { return 17961; }
        if self.r#north == North::Tall && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false { return 18018; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::None && self.r#up == true { return 17977; }
        if self.r#up == false && self.r#south == South::None && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::None { return 17995; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None && self.r#north == North::Low { return 18063; }
        if self.r#east == East::None && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::None { return 17918; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Tall && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == true { return 17979; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == true && self.r#west == West::None && self.r#waterlogged == true { return 18104; }
        if self.r#north == North::Tall && self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true { return 18230; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == true && self.r#south == South::None { return 18025; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::None && self.r#up == false { return 17946; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Tall && self.r#up == false && self.r#east == East::Tall { return 18199; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::None && self.r#north == North::None && self.r#up == true { return 17925; }
        if self.r#east == East::Low && self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true { return 18080; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false && self.r#west == West::Low && self.r#waterlogged == false { return 18126; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::None && self.r#south == South::None && self.r#up == true && self.r#waterlogged == true { return 17914; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::Low && self.r#up == false && self.r#north == North::Tall { return 18003; }
        if self.r#up == true && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Low { return 18024; }
        if self.r#up == false && self.r#south == South::None && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Low { return 18031; }
        if self.r#up == true && self.r#north == North::Low && self.r#south == South::Low && self.r#west == West::Low && self.r#east == East::Low && self.r#waterlogged == true { return 18069; }
        if self.r#north == North::Tall && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Low { return 18110; }
        if self.r#up == true && self.r#south == South::Low && self.r#north == North::Tall && self.r#west == West::Tall && self.r#east == East::Tall && self.r#waterlogged == true { return 18214; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#up == false && self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::Low { return 17968; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall && self.r#up == true && self.r#south == South::Tall && self.r#east == East::None { return 18011; }
        if self.r#west == West::Low && self.r#south == South::None && self.r#up == false && self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == true { return 17919; }
        if self.r#north == North::Low && self.r#west == West::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Low && self.r#up == true { return 18083; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::None && self.r#east == East::None && self.r#north == North::Low && self.r#up == true { return 17960; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#up == false && self.r#west == West::Low && self.r#south == South::Low { return 18075; }
        if self.r#west == West::None && self.r#north == North::Low && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#east == East::None { return 17954; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::None { return 18202; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == true && self.r#north == North::Tall { return 18105; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::None && self.r#up == false { return 17991; }
        if self.r#west == West::Low && self.r#up == true && self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::Tall && self.r#waterlogged == true { return 18201; }
        if self.r#south == South::None && self.r#north == North::Tall && self.r#east == East::Low && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == true { return 18100; }
        if self.r#east == East::None && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Tall { return 17944; }
        if self.r#east == East::Low && self.r#up == true && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::None { return 18095; }
        if self.r#north == North::None && self.r#west == West::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#south == South::Low { return 18141; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::None && self.r#north == North::Tall && self.r#up == true && self.r#west == West::None { return 18092; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == false { return 18002; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#up == true && self.r#east == East::Tall && self.r#south == South::None && self.r#west == West::Low { return 18132; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == false { return 18160; }
        if self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None { return 18041; }
        if self.r#south == South::Low && self.r#north == North::None && self.r#east == East::Low && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == false { return 18036; }
        if self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Tall { return 17980; }
        if self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::Tall && self.r#west == West::Low { return 18159; }
        if self.r#up == true && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Low { return 18021; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::None && self.r#up == false { return 18148; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::Tall && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Tall { return 18010; }
        if self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None { return 17981; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::Low && self.r#east == East::None && self.r#up == true { return 17973; }
        if self.r#south == South::Low && self.r#east == East::Low && self.r#waterlogged == false && self.r#up == false && self.r#west == West::None && self.r#north == North::Low { return 18077; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#up == false && self.r#north == North::None { return 17932; }
        if self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::None && self.r#up == false { return 18005; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Tall && self.r#west == West::None && self.r#waterlogged == false && self.r#up == true { return 18119; }
        if self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::Low && self.r#up == true { return 18085; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::Low && self.r#up == false { return 18079; }
        if self.r#up == true && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::Low && self.r#west == West::Low && self.r#north == North::Tall { return 18216; }
        if self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#south == South::Low && self.r#north == North::None && self.r#west == West::Tall { return 18037; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == true && self.r#west == West::None && self.r#waterlogged == true { return 18008; }
        if self.r#up == true && self.r#east == East::Low && self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::Low && self.r#waterlogged == false { return 18073; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::None && self.r#up == true { return 17913; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#up == true && self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::Low { return 17999; }
        if self.r#up == false && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == false { return 18030; }
        if self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::None && self.r#north == North::None { return 17939; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::None && self.r#north == North::Tall && self.r#east == East::None { return 17989; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::None && self.r#up == false && self.r#east == East::Low { return 18098; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false && self.r#north == North::None && self.r#waterlogged == false { return 18053; }
        if self.r#south == South::None && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::None && self.r#up == true && self.r#north == North::Low { return 17951; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == false { return 18207; }
        if self.r#west == West::Low && self.r#north == North::None && self.r#south == South::Tall && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true { return 18045; }
        if self.r#waterlogged == true && self.r#up == false && self.r#west == West::Tall && self.r#east == East::Tall && self.r#south == South::Low && self.r#north == North::Tall { return 18220; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall { return 18217; }
        if self.r#up == true && self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::None && self.r#east == East::Tall { return 18140; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#up == false && self.r#south == South::None { return 18065; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::None { return 18096; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None { return 18026; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#east == East::None && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false { return 17969; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::None && self.r#east == East::Low && self.r#up == true && self.r#west == West::None { return 18032; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == false && self.r#east == East::Low && self.r#west == West::None { return 18086; }
        if self.r#north == North::None && self.r#up == true && self.r#west == West::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Low { return 18145; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false { return 18183; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::Tall && self.r#up == true { return 17976; }
        if self.r#south == South::Tall && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == false && self.r#east == East::Tall { return 18235; }
        if self.r#up == true && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::None && self.r#south == South::None && self.r#north == North::None { return 17915; }
        if self.r#south == South::Tall && self.r#up == true && self.r#north == North::None && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::None { return 18152; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true { return 17943; }
        if self.r#west == West::Tall && self.r#up == true && self.r#south == South::Low && self.r#east == East::None && self.r#waterlogged == true && self.r#north == North::Low { return 17962; }
        if self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::None && self.r#up == true { return 17917; }
        if self.r#up == false && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Low { return 18198; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true && self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::None { return 17938; }
        if self.r#north == North::Tall && self.r#up == false && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None { return 17993; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::Low { return 17997; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == true { return 18121; }
        if self.r#east == East::None && self.r#up == false && self.r#north == North::Low && self.r#west == West::None && self.r#south == South::None && self.r#waterlogged == false { return 17957; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None { return 18182; }
        if self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::Tall && self.r#west == West::Tall { return 17947; }
        if self.r#south == South::Tall && self.r#north == North::None && self.r#west == West::None && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true { return 18050; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::None && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true { return 17949; }
        if self.r#west == West::Tall && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Low && self.r#up == true && self.r#east == East::Low { return 18061; }
        if self.r#up == true && self.r#east == East::Low && self.r#west == West::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::Tall { return 18120; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#north == North::Low && self.r#up == true && self.r#west == West::Tall && self.r#south == South::None { return 17953; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Tall && self.r#up == true { return 18228; }
        if self.r#north == North::Low && self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall { return 17974; }
        if self.r#north == North::None && self.r#west == West::Low && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall { return 18054; }
        if self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Tall && self.r#north == North::Tall { return 18004; }
        if self.r#south == South::Tall && self.r#west == West::Low && self.r#up == true && self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == false { return 18192; }
        if self.r#south == South::Tall && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == false && self.r#east == East::Low { return 18123; }
        if self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::Low && self.r#south == South::None { return 18165; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Tall { return 18127; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::None && self.r#south == South::Low && self.r#up == true { return 18212; }
        if self.r#south == South::Low && self.r#east == East::None && self.r#up == false && self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::Tall { return 18006; }
        if self.r#east == East::None && self.r#up == true && self.r#west == West::Low && self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::None { return 17952; }
        if self.r#west == West::Low && self.r#east == East::None && self.r#waterlogged == true && self.r#up == false && self.r#south == South::None && self.r#north == North::Low { return 17955; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None { return 18173; }
        if self.r#west == West::Low && self.r#up == true && self.r#east == East::Low && self.r#south == South::None && self.r#north == North::Low && self.r#waterlogged == false { return 18060; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::None && self.r#up == false { return 17967; }
        if self.r#south == South::None && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::Tall { return 18097; }
        if self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::None && self.r#east == East::Low && self.r#up == true && self.r#north == North::None { return 18035; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::Low && self.r#up == true && self.r#west == West::Tall { return 18106; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false { return 18084; }
        if self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == false && self.r#east == East::Tall && self.r#south == South::Tall { return 18233; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall { return 18115; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Tall { return 18103; }
        if self.r#up == false && self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Tall { return 18208; }
        if self.r#north == North::None && self.r#up == true && self.r#west == West::Tall && self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::Tall { return 17941; }
        if self.r#north == North::Low && self.r#east == East::None && self.r#south == South::None && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == true { return 17956; }
        if self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::Tall && self.r#up == false && self.r#east == East::Low && self.r#west == West::Low { return 18099; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == true && self.r#west == West::None { return 18188; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#up == false && self.r#west == West::None { return 18113; }
        if self.r#up == true && self.r#south == South::Tall && self.r#west == West::None && self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::Tall { return 18224; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#west == West::None && self.r#east == East::None && self.r#up == true && self.r#waterlogged == false { return 17987; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#west == West::None && self.r#south == South::None { return 18200; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#up == true && self.r#west == West::None && self.r#south == South::Tall && self.r#waterlogged == true { return 18116; }
        if self.r#up == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#waterlogged == true && self.r#south == South::Tall && self.r#north == North::Tall { return 18118; }
        if self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Tall { return 18019; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#up == false && self.r#west == West::None && self.r#north == North::None && self.r#south == South::None { return 17921; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::Low && self.r#up == false { return 18015; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::None && self.r#waterlogged == true && self.r#up == false { return 18122; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::None && self.r#south == South::None && self.r#up == false { return 18134; }
        if self.r#east == East::None && self.r#north == North::None && self.r#up == true && self.r#west == West::None && self.r#south == South::Tall && self.r#waterlogged == true { return 17936; }
        if self.r#up == false && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::Low { return 18146; }
        if self.r#west == West::None && self.r#waterlogged == true && self.r#up == false && self.r#north == North::None && self.r#east == East::None && self.r#south == South::Low { return 17930; }
        if self.r#north == North::None && self.r#up == true && self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::Low && self.r#west == West::Low { return 17928; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == false { return 18007; }
        if self.r#up == false && self.r#south == South::None && self.r#north == North::None && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::None { return 18137; }
        if self.r#up == true && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::Low && self.r#west == West::Tall && self.r#waterlogged == true { return 18190; }
        if self.r#east == East::Tall && self.r#west == West::None && self.r#up == false && self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == true { return 18206; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None { return 18014; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall { return 17998; }
        if self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::Low { return 17971; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true { return 18016; }
        if self.r#up == true && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::None { return 18046; }
        if self.r#west == West::Tall && self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false { return 18052; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::None && self.r#up == true { return 18058; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#west == West::Low && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false { return 18078; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true { return 18142; }
        if self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Tall && self.r#north == North::Low { return 18172; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Low && self.r#up == false { return 18043; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::Tall { return 18091; }
        if self.r#north == North::None && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall { return 18158; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == true && self.r#west == West::Tall { return 18181; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None { return 18185; }
        if self.r#up == false && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::Low { return 18194; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::None && self.r#up == true && self.r#north == North::Tall { return 18001; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::None && self.r#west == West::Tall && self.r#up == true { return 18133; }
        if self.r#south == South::Tall && self.r#up == false && self.r#north == North::None && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Low { return 18162; }
        if self.r#waterlogged == false && self.r#up == false && self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::None && self.r#west == West::Tall { return 18067; }
        if self.r#north == North::Low && self.r#east == East::None && self.r#south == South::Low && self.r#west == West::Tall && self.r#up == true && self.r#waterlogged == false { return 17965; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Low && self.r#west == West::Low && self.r#east == East::Tall && self.r#up == true { return 18177; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::None && self.r#west == West::None && self.r#up == true { return 17912; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::None { return 17984; }
        if self.r#east == East::None && self.r#up == false && self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::Tall && self.r#waterlogged == false { return 18017; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::Tall && self.r#up == false && self.r#west == West::Low && self.r#north == North::Low { return 18090; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low { return 17931; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::None && self.r#up == false && self.r#west == West::Tall { return 17920; }
        if self.r#west == West::None && self.r#up == false && self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::Low { return 18218; }
        if self.r#east == East::None && self.r#south == South::None && self.r#waterlogged == false && self.r#up == false && self.r#north == North::Low && self.r#west == West::Tall { return 17959; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#up == false && self.r#north == North::Low && self.r#east == East::Tall { return 18184; }
        if self.r#waterlogged == false && self.r#south == South::None && self.r#north == North::Tall && self.r#west == West::None && self.r#up == false && self.r#east == East::Tall { return 18209; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Tall { return 18223; }
        if self.r#south == South::Low && self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#up == false && self.r#west == West::None { return 18038; }
        if self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None && self.r#south == South::None { return 18131; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall { return 17929; }
        if self.r#west == West::Low && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Tall && self.r#north == North::None { return 18156; }
        if self.r#west == West::Tall && self.r#south == South::None && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Tall { return 18094; }
        if self.r#north == North::None && self.r#up == false && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Tall && self.r#east == East::Tall { return 18139; }
        if self.r#north == North::Tall && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#waterlogged == true && self.r#up == false { return 18231; }
        if self.r#north == North::None && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::None { return 17945; }
        if self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::None { return 18170; }
        if self.r#north == North::None && self.r#south == South::None && self.r#west == West::None && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Tall { return 18128; }
        if self.r#east == East::None && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Low && self.r#up == true { return 17948; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Low { return 18171; }
        if self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Low && self.r#west == West::None { return 17975; }
        if self.r#east == East::Tall && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Low { return 18221; }
        if self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::Tall { return 18229; }
        if self.r#south == South::Low && self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall { return 17926; }
        if self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == true && self.r#east == East::Tall { return 18157; }
        if self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::None && self.r#east == East::Tall { return 18149; }
        if self.r#west == West::None && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true { return 18056; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Low { return 18204; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::None { return 18066; }
        if self.r#up == true && self.r#east == East::None && self.r#west == West::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::Low { return 18000; }
        if self.r#north == North::Low && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::Low { return 18189; }
        if self.r#south == South::None && self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == true && self.r#up == false && self.r#west == West::Tall { return 18028; }
        if self.r#up == true && self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::Low { return 18072; }
        if self.r#north == North::Low && self.r#up == false && self.r#east == East::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Low { return 18087; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true && self.r#east == East::Low && self.r#south == South::Tall && self.r#north == North::Low { return 18082; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::None && self.r#up == true { return 18013; }
        if self.r#up == true && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::Low { return 18107; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Low && self.r#south == South::None { return 18064; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::Low && self.r#up == true && self.r#west == West::None { return 18068; }
        if self.r#up == false && self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::Low { return 17983; }
        if self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#north == North::None { return 18048; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low { return 18102; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::Tall && self.r#up == true && self.r#south == South::None && self.r#waterlogged == true { return 18130; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::None && self.r#east == East::None && self.r#west == West::Tall && self.r#up == true { return 17950; }
        if self.r#up == true && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None && self.r#south == South::Low { return 17924; }
        if self.r#south == South::Low && self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Low && self.r#up == false { return 17970; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::Low && self.r#east == East::Low && self.r#west == West::Low && self.r#up == false { return 18039; }
        if self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::None && self.r#west == West::None { return 17933; }
        if self.r#waterlogged == false && self.r#up == true && self.r#south == South::None && self.r#east == East::Low && self.r#west == West::None && self.r#north == North::Low { return 18059; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#up == false && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Low { return 18051; }
        if self.r#up == true && self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::None && self.r#south == South::Low { return 18143; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#up == true && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::None { return 17996; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#west == West::Tall && self.r#north == North::None { return 18136; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == true && self.r#west == West::None { return 18227; }
        if self.r#west == West::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == false { return 18211; }
        if self.r#east == East::Tall && self.r#up == true && self.r#west == West::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#south == South::Low { return 18144; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::None && self.r#up == false && self.r#north == North::None && self.r#waterlogged == false { return 17923; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Tall && self.r#up == true && self.r#north == North::Low { return 18166; }
        if self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::Low && self.r#up == false && self.r#east == East::None && self.r#waterlogged == true { return 17978; }
        if self.r#waterlogged == false && self.r#up == true && self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::Low && self.r#east == East::None { return 18012; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 18088; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::Tall && self.r#waterlogged == true && self.r#east == East::Low && self.r#up == false { return 18124; }
        if self.r#west == West::None && self.r#up == false && self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == false { return 18101; }
        if self.r#west == West::None && self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true { return 18176; }
        if self.r#west == West::Tall && self.r#up == true && self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Tall { return 18205; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == false { return 18234; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::None { return 18027; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::None && self.r#north == North::Low && self.r#west == West::Low && self.r#up == false { return 17958; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#up == true && self.r#east == East::None && self.r#south == South::Tall && self.r#north == North::None { return 17940; }
        if self.r#west == West::None && self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == false && self.r#south == South::None { return 17990; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::None && self.r#up == false && self.r#south == South::None && self.r#west == West::Low { return 18138; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::None && self.r#east == East::Low && self.r#south == South::Low && self.r#up == false { return 18042; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::Low && self.r#up == false && self.r#west == West::None && self.r#south == South::None { return 18029; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#up == false && self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == true { return 18076; }
        if self.r#up == true && self.r#west == West::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Low { return 18191; }
        if self.r#east == East::Tall && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Low { return 18174; }
        if self.r#south == South::Tall && self.r#up == false && self.r#east == East::None && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::None { return 17942; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::None && self.r#up == true { return 17988; }
        if self.r#west == West::None && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::Tall { return 18155; }
        if self.r#north == North::Low && self.r#east == East::Tall && self.r#west == West::Low && self.r#up == true && self.r#south == South::None && self.r#waterlogged == false { return 18168; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None && self.r#up == true { return 18129; }
        if self.r#north == North::Tall && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true { return 18009; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::Tall { return 18215; }
        if self.r#west == West::None && self.r#north == North::Low && self.r#east == East::None && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false { return 17963; }
        if self.r#east == East::None && self.r#north == North::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::None { return 17916; }
        if self.r#up == true && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::None && self.r#east == East::Low { return 18020; }
        if self.r#up == true && self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Tall { return 18169; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == true && self.r#up == false && self.r#west == West::Low { return 18135; }
        if self.r#up == false && self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::Tall { return 17935; }
        if self.r#south == South::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == false && self.r#north == North::None && self.r#west == West::Tall { return 18163; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Low { return 18180; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::Tall && self.r#up == false && self.r#east == East::None && self.r#west == West::Tall { return 17992; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == true && self.r#south == South::Tall && self.r#west == West::Tall { return 18193; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::None && self.r#up == true && self.r#west == West::Tall { return 18034; }
        if self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Tall { return 18055; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::None && self.r#up == true && self.r#west == West::None { return 18044; }
        if self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Tall && self.r#south == South::Tall { return 18154; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true { return 18111; }
        if self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::Low { return 18117; }
        if self.r#north == North::Low && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Tall && self.r#up == false { return 18197; }
        if self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == false && self.r#south == South::None { return 17922; }
        if self.r#east == East::Tall && self.r#up == true && self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Low { return 18213; }
        if self.r#south == South::None && self.r#up == true && self.r#north == North::Low && self.r#west == West::None && self.r#east == East::Tall && self.r#waterlogged == true { return 18164; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Low && self.r#up == true && self.r#west == West::Low { return 17964; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::Tall && self.r#up == false && self.r#north == North::None { return 18040; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall { return 18109; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Low { return 18114; }
        if self.r#east == East::Tall && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == false { return 18195; }
        if self.r#west == West::None && self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false { return 18167; }
        if self.r#up == false && self.r#south == South::Tall && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Low { return 18196; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Tall { return 18222; }
        if self.r#west == West::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == false { return 18232; }
        if self.r#up == false && self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::Low && self.r#waterlogged == false { return 18125; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#south == South::Tall && self.r#up == false && self.r#west == West::None { return 18161; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Low { return 18210; }
        if self.r#up == true && self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::Low && self.r#south == South::None && self.r#east == East::Low { return 18057; }
        if self.r#up == true && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Tall && self.r#west == West::Tall { return 17986; }
        if self.r#south == South::Low && self.r#up == false && self.r#east == East::None && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Low { return 17934; }
        if self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low { return 17937; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::Low && self.r#west == West::Tall && self.r#south == South::None && self.r#up == true { return 18022; }
        if self.r#west == West::None && self.r#up == true && self.r#north == North::None && self.r#south == South::None && self.r#east == East::Low && self.r#waterlogged == false { return 18023; }
        if self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::Low && self.r#up == true && self.r#north == North::Tall && self.r#west == West::Low { return 18108; }
        if self.r#up == true && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Tall && self.r#west == West::None && self.r#south == South::None { return 18203; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Low && self.r#up == false { return 18074; }
        if self.r#west == West::Tall && self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#up == true { return 18226; }
        if self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Low && self.r#west == West::None { return 17966; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#west == West::Tall && self.r#east == East::Low && self.r#waterlogged == true && self.r#up == true { return 18070; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low { return 18175; }
        if self.r#up == true && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None && self.r#south == South::Tall { return 18047; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::Tall && self.r#north == North::None { return 18049; }
        if self.r#north == North::Low && self.r#west == West::Low && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == true { return 18081; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false { return 18089; }
        if self.r#west == West::Low && self.r#north == North::None && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Tall { return 18147; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == false && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::Low { return 18151; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#up == true && self.r#south == South::Tall && self.r#west == West::None { return 17972; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true { return 18225; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == false && self.r#south == South::Low && self.r#east == East::Tall && self.r#west == West::Low { return 18219; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Low { return 18033; }
        if self.r#west == West::None && self.r#up == false && self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::Low { return 18062; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 18179 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 17982 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17994 {
            return Some(StoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17927 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 18093 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18186 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 18112 {
            return Some(StoneBrickWall {
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18178 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 18153 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18150 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18187 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 17985 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18071 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17961 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 18018 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17977 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 17995 {
            return Some(StoneBrickWall {
                r#up: false,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 18063 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 17918 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 17979 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18104 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18230 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18025 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 17946 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 18199 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 17925 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 18080 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18126 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17914 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18003 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 18024 {
            return Some(StoneBrickWall {
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18031 {
            return Some(StoneBrickWall {
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 18069 {
            return Some(StoneBrickWall {
                r#up: true,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18110 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 18214 {
            return Some(StoneBrickWall {
                r#up: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17968 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 18011 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 17919 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18083 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 17960 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 18075 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17954 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 18202 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18105 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17991 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 18201 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18100 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17944 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18095 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18141 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 18092 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 18002 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 18132 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 18160 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 18041 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18036 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17980 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18159 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 18021 {
            return Some(StoneBrickWall {
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18148 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 18010 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17981 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17973 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 18077 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 17932 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 18005 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 18119 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 18085 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 18079 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 18216 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 18037 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18008 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18073 {
            return Some(StoneBrickWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17913 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17999 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18030 {
            return Some(StoneBrickWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17939 {
            return Some(StoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 17989 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 18098 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 18053 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17951 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 18207 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 18045 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18220 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 18217 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18140 {
            return Some(StoneBrickWall {
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18065 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 18096 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 18026 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17969 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18032 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 18086 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 18145 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 18183 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 17976 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 18235 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 17915 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 18152 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17943 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17962 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 17917 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 18198 {
            return Some(StoneBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17938 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 17993 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17997 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 18121 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17957 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18182 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17947 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18050 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::None,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17949 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18061 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 18120 {
            return Some(StoneBrickWall {
                r#up: true,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17953 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18228 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 17974 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18054 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 18004 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18192 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18123 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 18165 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 18127 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18212 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18006 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17952 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 17955 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 18173 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18060 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17967 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 18097 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 18035 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 18106 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18084 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 18233 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18115 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18103 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18208 {
            return Some(StoneBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 17941 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17956 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18099 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 18188 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 18113 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 18224 {
            return Some(StoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17987 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 18200 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 18116 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18118 {
            return Some(StoneBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18019 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17921 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 18015 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 18122 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 18134 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 17936 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18146 {
            return Some(StoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17930 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#up: false,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 17928 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 18007 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 18137 {
            return Some(StoneBrickWall {
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18190 {
            return Some(StoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18206 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18014 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17998 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17971 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 18016 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18046 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18052 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 18058 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18078 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18142 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18172 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18043 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 18091 {
            return Some(StoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18158 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18181 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18185 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18194 {
            return Some(StoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18001 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 18133 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 18162 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18067 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 17965 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 18177 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 17912 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 17984 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 18017 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18090 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17931 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17920 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18218 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17959 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 18184 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 18209 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 18223 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18038 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 18131 {
            return Some(StoneBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 17929 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18156 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18094 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 18139 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 18231 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 17945 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 18170 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18128 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 17948 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 18171 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17975 {
            return Some(StoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 18221 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 18229 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17926 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18157 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 18149 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18056 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18204 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 18066 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 18000 {
            return Some(StoneBrickWall {
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 18189 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 18028 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18072 {
            return Some(StoneBrickWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 18087 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18082 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18013 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 18107 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18064 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 18068 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 17983 {
            return Some(StoneBrickWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18048 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18102 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18130 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17950 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 17924 {
            return Some(StoneBrickWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 17970 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 18039 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17933 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 18059 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 18051 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18143 {
            return Some(StoneBrickWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 17996 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18136 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18227 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 18211 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 18144 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 17923 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18166 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 17978 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18012 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 18088 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18124 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 18101 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18176 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18205 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18234 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 18027 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 17958 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17940 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17990 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 18138 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 18042 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 18029 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 18076 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18191 {
            return Some(StoneBrickWall {
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18174 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 17942 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 17988 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18155 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18168 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18129 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18009 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18215 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17963 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17916 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 18020 {
            return Some(StoneBrickWall {
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 18169 {
            return Some(StoneBrickWall {
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 18135 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 17935 {
            return Some(StoneBrickWall {
                r#up: false,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 18163 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18180 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17992 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18193 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18034 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18055 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18044 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 18154 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18111 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18117 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 18197 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 17922 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 18213 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18164 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17964 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 18040 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 18109 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18114 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 18195 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 18167 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 18196 {
            return Some(StoneBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18222 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 18232 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 18125 {
            return Some(StoneBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18161 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 18210 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 18057 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 17986 {
            return Some(StoneBrickWall {
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 17934 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 17937 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18022 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18023 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18108 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 18203 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 18074 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 18226 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17966 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 18070 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 18175 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18047 {
            return Some(StoneBrickWall {
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18049 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18081 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 18089 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18147 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 18151 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 17972 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18225 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 18219 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 18033 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 18062 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#up: false,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        return None;
    }
}

