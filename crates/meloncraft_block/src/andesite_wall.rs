use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AndesiteWall {
    pub r#north: North,
    pub up: bool,
    pub r#east: East,
    pub waterlogged: bool,
    pub r#west: West,
    pub r#south: South,
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

impl BlockState for AndesiteWall {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Low { return 19053; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Low { return 18905; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 19051; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 19174; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 19147; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == true { return 18933; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::None { return 18993; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 18944; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 18951; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 19200; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 18972; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 19014; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 18980; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == true { return 18960; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 19028; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#west == West::None { return 19127; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 19105; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None { return 19205; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#waterlogged == false { return 18893; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low { return 19008; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 19012; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Tall { return 19072; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 19155; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::None { return 19179; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 18942; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::None { return 19123; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#waterlogged == false { return 18911; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::Low { return 19166; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == false { return 19001; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 19055; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 19057; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::Tall { return 19125; }
        if block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall { return 19141; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true { return 19017; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 19044; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::None { return 18929; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::None { return 18996; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::None { return 19011; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 19124; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#waterlogged == false { return 18900; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall { return 18970; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == false { return 18890; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 19047; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::None { return 19114; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == false { return 18912; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::Low { return 18897; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::Tall { return 19133; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#waterlogged == false { return 18954; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 19168; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::Tall { return 19176; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None { return 18910; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::None { return 18895; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Low { return 18940; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 18953; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false { return 18965; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::Low { return 19073; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None { return 19172; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#south == South::Tall { return 18952; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == false { return 19037; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false { return 18966; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::None { return 18896; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 19079; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 19170; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true { return 18898; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 19045; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true { return 18902; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 19097; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true { return 19119; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 19171; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 19194; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 19185; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 18968; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::None { return 18992; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 19098; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 19167; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::None { return 18994; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Low { return 18987; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#west == West::Low { return 19143; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Low { return 19005; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 19067; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Low { return 19092; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::Low { return 19077; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true { return 19100; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true { return 18999; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 19003; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::Low { return 19007; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true { return 19031; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 19038; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false { return 19082; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 19087; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::Low { return 19064; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true { return 19187; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::None { return 19000; }
        if block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::None { return 19070; }
        if block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 19094; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 18961; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Tall { return 19112; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#up == true { return 19033; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::None { return 19110; }
        if block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == false { return 18963; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::None { return 18889; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 19151; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#south == South::Tall { return 18949; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::Low { return 19091; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 19159; }
        if block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 19163; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::None { return 19137; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#waterlogged == true { return 18974; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#up == false { return 19183; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Low { return 19026; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 19207; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 19039; }
        if block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None { return 18936; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::None { return 19118; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Tall { return 18886; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::None { return 19027; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall { return 19120; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Low { return 18901; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::None { return 19024; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == true { return 19029; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#up == true { return 19088; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 19049; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 19054; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 19129; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false { return 19154; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None { return 19202; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false { return 18906; }
        if block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 19193; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false { return 19145; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == false { return 19115; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 18888; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 18984; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == true { return 19196; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 19090; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Low { return 19116; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Tall { return 18983; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 19164; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 19189; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 19199; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 18958; }
        if block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 19010; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 18924; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::None { return 19140; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::None { return 19101; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 18981; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::None { return 18941; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 18908; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == true { return 18909; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == true { return 18973; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 18982; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#west == West::None { return 19016; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 18904; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 18903; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Tall { return 18937; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Tall { return 19102; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Low { return 18939; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::None { return 19132; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::None { return 18926; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 18892; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == true { return 18947; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 19162; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Tall { return 19181; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 19081; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false { return 18916; }
        if block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None { return 19004; }
        if block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 19128; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::Low { return 19139; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::None { return 19160; }
        if block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == false { return 19121; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Low { return 19134; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None { return 19009; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::Low { return 19086; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == false { return 18955; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == false { return 19075; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true { return 19136; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 19150; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 19061; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None { return 19107; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::Low { return 19156; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::None { return 18894; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None { return 19065; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Tall { return 18985; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Low { return 19080; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 19084; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Low { return 19144; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false { return 18918; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::None { return 18962; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true { return 19066; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None { return 18956; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None { return 19113; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Tall { return 19018; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#up == true { return 18997; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 19165; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == true { return 18934; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 18978; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == true { return 19104; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 19106; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 19063; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 18920; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true { return 19152; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 19135; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 18950; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 19095; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Tall { return 19175; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 19058; }
        if block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 19052; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 19111; }
        if block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 19006; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::None { return 18930; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::Low { return 18923; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::Low { return 19122; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 19013; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Low { return 19117; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#west == West::Low { return 19035; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::None { return 18932; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 19059; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == false { return 19142; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 19099; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true { return 19149; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 19074; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 19188; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#north == North::Low { return 19034; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 19195; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Low { return 19153; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 19206; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#up == true { return 19093; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::None { return 19103; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Low { return 18935; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == false { return 18976; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Tall { return 18913; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::None { return 19076; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low { return 19089; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::None { return 19025; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::None { return 19109; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 19158; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::Tall { return 19169; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true { return 18884; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::None { return 18938; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 18921; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::Low { return 19030; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None { return 18957; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Tall { return 19060; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 19130; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 19040; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall { return 19184; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 18931; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 18943; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None { return 18977; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Low { return 19083; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 19161; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#north == North::Tall { return 19190; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 19021; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 18919; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 18988; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == false { return 19043; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#up == false { return 18891; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false { return 19192; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 19126; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 19173; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 18946; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 19203; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::None { return 18991; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == true { return 19068; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 19197; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 18914; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None { return 18990; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 19041; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Low { return 19048; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true { return 19138; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::None { return 18967; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low { return 19023; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Low { return 18925; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false { return 18989; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 19036; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#south == South::Low { return 18969; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 19096; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#east == East::None { return 18928; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true { return 18964; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Low { return 19085; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 19069; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::None { return 18959; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::Tall { return 19015; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 18986; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == false { return 18887; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#waterlogged == true { return 18927; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::None { return 18907; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 19204; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 18975; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 18979; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true { return 19046; }
        if block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == false { return 18917; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 19177; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 19180; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 19019; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Low { return 19020; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Low { return 19157; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Low { return 18945; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::None { return 19002; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Low { return 18948; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None { return 19022; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::None { return 18899; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Low { return 19182; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#up == true { return 19186; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None { return 18885; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == true { return 19032; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Low { return 19062; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false { return 18971; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::None { return 19148; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low { return 19042; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true { return 18922; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 19056; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == false { return 19178; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 19071; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None { return 18915; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::None { return 19108; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 19191; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 19198; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None { return 18998; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 18995; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Tall { return 19201; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#north == North::Low { return 19146; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::Low { return 19050; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == true { return 19078; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 19131; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 19053 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 18905 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 19051 {
            return Some(AndesiteWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19174 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19147 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18933 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 18993 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 18944 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18951 {
            return Some(AndesiteWall {
                r#up: false,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 19200 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18972 {
            return Some(AndesiteWall {
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19014 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18980 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18960 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 19028 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19127 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 19105 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19205 {
            return Some(AndesiteWall {
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18893 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19008 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19012 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19072 {
            return Some(AndesiteWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 19155 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19179 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18942 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19123 {
            return Some(AndesiteWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18911 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19166 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 19001 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 19055 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 19057 {
            return Some(AndesiteWall {
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19125 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19141 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19017 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 19044 {
            return Some(AndesiteWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18929 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 18996 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 19011 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 19124 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18900 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18970 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18890 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 19047 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19114 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18912 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18897 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 19133 {
            return Some(AndesiteWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18954 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19168 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19176 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 18910 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 18895 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 18940 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 18953 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18965 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 19073 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 19172 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18952 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19037 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 18966 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 18896 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 19079 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19170 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 18898 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 19045 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 18902 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19097 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 19119 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19171 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 19194 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19185 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18968 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18992 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 19098 {
            return Some(AndesiteWall {
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 19167 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 18994 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 18987 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 19143 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 19005 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19067 {
            return Some(AndesiteWall {
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19092 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 19077 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19100 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 18999 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19003 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19007 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 19031 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 19038 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 19082 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 19087 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 19064 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 19187 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 19000 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 19070 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 19094 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18961 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19112 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 19033 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 19110 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18963 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 18889 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 19151 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18949 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19091 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 19159 {
            return Some(AndesiteWall {
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19163 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19137 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 18974 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19183 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 19026 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 19207 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 19039 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18936 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 19118 {
            return Some(AndesiteWall {
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 18886 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19027 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 19120 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18901 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 19024 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 19029 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 19088 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 19049 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 19054 {
            return Some(AndesiteWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 19129 {
            return Some(AndesiteWall {
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19154 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 19202 {
            return Some(AndesiteWall {
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18906 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 19193 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 19145 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 19115 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18888 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18984 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19196 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 19090 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 19116 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 18983 {
            return Some(AndesiteWall {
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19164 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19189 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 19199 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18958 {
            return Some(AndesiteWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19010 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#west: West::None,
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18924 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19140 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 19101 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 18981 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 18941 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 18908 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18909 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 18973 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18982 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19016 {
            return Some(AndesiteWall {
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 18904 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 18903 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18937 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19102 {
            return Some(AndesiteWall {
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18939 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 19132 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18926 {
            return Some(AndesiteWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 18892 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18947 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 19162 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19181 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 19081 {
            return Some(AndesiteWall {
                r#up: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18916 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 19004 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 19128 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19139 {
            return Some(AndesiteWall {
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 19160 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19121 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 19134 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 19009 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 19086 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 18955 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 19075 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 19136 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 19150 {
            return Some(AndesiteWall {
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19061 {
            return Some(AndesiteWall {
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19107 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 19156 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 18894 {
            return Some(AndesiteWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 19065 {
            return Some(AndesiteWall {
                r#up: true,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 18985 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19080 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 19084 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 19144 {
            return Some(AndesiteWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 18918 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 18962 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19066 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 18956 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 19113 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 19018 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18997 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 19165 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 18934 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 18978 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 19104 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 19106 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19063 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18920 {
            return Some(AndesiteWall {
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 19152 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 19135 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18950 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19095 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 19175 {
            return Some(AndesiteWall {
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19058 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 19052 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19111 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 19006 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18930 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 18923 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 19122 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 19013 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 19117 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 19035 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 18932 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 19059 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19142 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 19099 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 19149 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 19074 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19188 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 19034 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 19195 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 19153 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 19206 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19093 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 19103 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 18935 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 18976 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 18913 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19076 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19089 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19025 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 19109 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 19158 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 19169 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 18884 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 18938 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 18921 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 19030 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 18957 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 19060 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19130 {
            return Some(AndesiteWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19040 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19184 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18931 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18943 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 18977 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 19083 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19161 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 19190 {
            return Some(AndesiteWall {
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 19021 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18919 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18988 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19043 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18891 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 19192 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 19126 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19173 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18946 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 19203 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 18991 {
            return Some(AndesiteWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 19068 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 19197 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 18914 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18990 {
            return Some(AndesiteWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 19041 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19048 {
            return Some(AndesiteWall {
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19138 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18967 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 19023 {
            return Some(AndesiteWall {
                r#up: false,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 18925 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 18989 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19036 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 18969 {
            return Some(AndesiteWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19096 {
            return Some(AndesiteWall {
                r#up: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 18928 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 18964 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19085 {
            return Some(AndesiteWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19069 {
            return Some(AndesiteWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 18959 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19015 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 18986 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18887 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18927 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18907 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 19204 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18975 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 18979 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 19046 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18917 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 19177 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19180 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 19019 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 19020 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 19157 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 18945 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 19002 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 18948 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 19022 {
            return Some(AndesiteWall {
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 18899 {
            return Some(AndesiteWall {
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 19182 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 19186 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 18885 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 19032 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 19062 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 18971 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19148 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 19042 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18922 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 19056 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19178 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 19071 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18915 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 19108 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 19191 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19198 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 18998 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 18995 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19201 {
            return Some(AndesiteWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 19146 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 19050 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 19078 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 19131 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        return None;
    }
}

