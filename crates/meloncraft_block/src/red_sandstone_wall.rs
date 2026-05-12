use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedSandstoneWall {
    pub r#south: South,
    pub waterlogged: bool,
    pub r#west: West,
    pub up: bool,
    pub r#east: East,
    pub r#north: North,
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

impl BlockState for RedSandstoneWall {
    fn to_id(self) -> i32 {
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::Low { return 17174; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 17193; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 17105; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#west == West::None { return 17234; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 17239; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::Low { return 17006; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::Tall { return 17184; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 17128; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall { return 17043; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 17064; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall { return 17113; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Tall { return 17241; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false { return 17035; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true { return 17256; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Tall { return 16966; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 17254; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true { return 17231; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 17235; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::None { return 16943; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false { return 17175; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true { return 16967; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 16986; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Low { return 16960; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::None { return 16977; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::Low { return 17142; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 17210; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 17109; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None { return 17007; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false { return 17127; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 17215; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 17134; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 17226; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::None { return 17022; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false { return 17131; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Low { return 17058; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::None { return 17163; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 17016; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == false { return 17151; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Tall { return 17021; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == false { return 16947; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None { return 17017; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 17099; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::None { return 16982; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == true { return 17060; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#west == West::None { return 17144; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Low { return 17057; }
        if block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false { return 17082; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 17247; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == false { return 16949; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Low { return 17010; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None { return 17216; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::Low { return 17000; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::None { return 16940; }
        if block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 17252; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::Tall { return 17160; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Low { return 17077; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 17033; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 16989; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None { return 17044; }
        if block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == true { return 17183; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == false { return 16985; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None { return 17038; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 17102; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 17029; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Tall { return 17117; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 17122; }
        if block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 17153; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 16995; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 17073; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 17009; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 17152; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#west == West::Low { return 17208; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::None { return 17086; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == false { return 17008; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None { return 17020; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true { return 17170; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 17198; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Low { return 17024; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 17225; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#west == West::None { return 17261; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None { return 17089; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 17031; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 17205; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 17224; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::None { return 17083; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None { return 17087; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::Low { return 17004; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::None { return 17037; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 17039; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true { return 16942; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 16991; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 17146; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall { return 17092; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall { return 17169; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == true { return 17036; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 17116; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 17130; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low { return 17061; }
        if block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Low { return 17030; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::None { return 16945; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Low { return 17207; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true { return 17255; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::None { return 17051; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 17212; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == true { return 16980; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Low { return 17104; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 17214; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None { return 16941; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 17034; }
        if block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::None { return 17059; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true { return 17124; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::Low { return 17166; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Tall { return 17158; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::Low { return 17108; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::Low { return 17121; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None { return 17132; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::None { return 17190; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true { return 17053; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None { return 16973; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 17180; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::None { return 16988; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 17027; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None { return 17065; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 17154; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::None { return 17202; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 17003; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::Tall { return 17204; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Low { return 17066; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true { return 17228; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 17047; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 17232; }
        if block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#up == false { return 17091; }
        if block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None { return 17074; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false { return 17138; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 17248; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#west == West::None { return 17075; }
        if block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 17201; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::None { return 16984; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#waterlogged == false { return 17015; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Low { return 17103; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == true { return 17014; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Low { return 17085; }
        if block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 17182; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 16969; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true { return 16954; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::None { return 16951; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false { return 16961; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Low { return 16950; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true { return 17242; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 17181; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Tall { return 17194; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::Tall { return 17019; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#east == East::Tall { return 17209; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall { return 16990; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Tall { return 16972; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#up == true { return 17013; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Low { return 17093; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 17095; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 17114; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall { return 17176; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Low { return 17005; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::Tall { return 17238; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::None { return 17164; }
        if block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 17101; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::None { return 17129; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 17220; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::Tall { return 17236; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 17257; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 16952; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 17263; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#south == South::Low { return 17139; }
        if block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 16968; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Tall { return 16971; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 16998; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low { return 17137; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#up == true { return 17072; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::None { return 16958; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Tall { return 16964; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::None { return 17071; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 17148; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None { return 16948; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::None { return 17162; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false { return 17094; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None { return 17178; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 17217; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == South::Tall { return 17187; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 17026; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Tall { return 17157; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::None { return 16979; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::None { return 17167; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true { return 17192; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 16962; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::None { return 16997; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 17052; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Low { return 17088; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::None { return 17219; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::None { return 17025; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::Tall { return 17068; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Low { return 17100; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall { return 17150; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 17028; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true { return 17111; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall { return 17200; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 17221; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Low { return 16959; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 17107; }
        if block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None { return 16965; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall { return 17230; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Low { return 17001; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 17227; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::Low { return 16956; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false { return 17078; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::None { return 17090; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low { return 17110; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall { return 17140; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 17203; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Low { return 17213; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 17250; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::None { return 17237; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None { return 17042; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Tall { return 17045; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Tall { return 17243; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::None { return 16955; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None { return 17199; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false { return 17211; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#west == West::Low { return 17244; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 16953; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 17002; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 17040; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 16999; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::None { return 17081; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false { return 17185; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low { return 17136; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 17253; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 17259; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true { return 17168; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == false { return 16946; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false { return 17080; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::None { return 17156; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#up == false { return 17260; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall { return 17155; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false { return 16963; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == true { return 17233; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 17196; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::None { return 17186; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Tall { return 16957; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::None { return 17229; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::Low { return 17054; }
        if block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 17173; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Tall { return 17041; }
        if block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None { return 17159; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::Low { return 17050; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 17143; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true { return 17048; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 17206; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Tall { return 17147; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 17223; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#up == false { return 17246; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::None { return 17258; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Low { return 17262; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true { return 17197; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Low { return 17119; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Tall { return 17126; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Low { return 17069; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Low { return 17106; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall { return 16993; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 17171; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Low { return 17079; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Low { return 17084; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low { return 17249; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall { return 17032; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 17012; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::Low { return 16983; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::None { return 17055; }
        if block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low { return 17076; }
        if block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Low { return 17133; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None { return 17161; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 17062; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false { return 17189; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Low { return 16974; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 17115; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None { return 17123; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::None { return 17165; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::None { return 17177; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::None { return 16970; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 17049; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low { return 17097; }
        if block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false { return 17070; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == false { return 17046; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Low { return 16978; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::None { return 16981; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Tall { return 17056; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 17118; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::Tall { return 17179; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Low { return 16994; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Tall { return 17125; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 17141; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == true { return 17063; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Tall { return 17135; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Low { return 17112; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall { return 17149; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None { return 16987; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None { return 16976; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Tall { return 17195; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 17218; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 17191; }
        if block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == South::Tall { return 17222; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 17120; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 17240; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == false { return 17188; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#up == false { return 16996; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 17018; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 17245; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 17251; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Tall { return 17145; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == false { return 17023; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::None { return 16944; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == false { return 17011; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Tall { return 16975; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Low { return 17067; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == true { return 17098; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#west == West::Low { return 16992; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == true { return 17096; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Low { return 17172; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 17174 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17193 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 17105 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17234 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 17239 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 17006 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 17184 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 17128 {
            return Some(RedSandstoneWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17043 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17064 {
            return Some(RedSandstoneWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17113 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17241 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17035 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 17256 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 16966 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17254 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 17231 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17235 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 16943 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 17175 {
            return Some(RedSandstoneWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 16967 {
            return Some(RedSandstoneWall {
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 16986 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 16960 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 16977 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 17142 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 17210 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17109 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17007 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 17127 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 17215 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 17134 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 17226 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17022 {
            return Some(RedSandstoneWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 17131 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17058 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 17163 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17016 {
            return Some(RedSandstoneWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17151 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 17021 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 16947 {
            return Some(RedSandstoneWall {
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 17017 {
            return Some(RedSandstoneWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 17099 {
            return Some(RedSandstoneWall {
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16982 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 17060 {
            return Some(RedSandstoneWall {
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 17144 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 17057 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 17082 {
            return Some(RedSandstoneWall {
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 17247 {
            return Some(RedSandstoneWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 16949 {
            return Some(RedSandstoneWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 17010 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 17216 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17000 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 16940 {
            return Some(RedSandstoneWall {
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 17252 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 17160 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 17077 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 17033 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 16989 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17044 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 17183 {
            return Some(RedSandstoneWall {
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 16985 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 17038 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 17102 {
            return Some(RedSandstoneWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17029 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17117 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17122 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17153 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16995 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17073 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17009 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17152 {
            return Some(RedSandstoneWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17208 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 17086 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 17008 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 17020 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 17170 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 17198 {
            return Some(RedSandstoneWall {
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17024 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 17225 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17261 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 17089 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 17031 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17205 {
            return Some(RedSandstoneWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17224 {
            return Some(RedSandstoneWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17083 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 17087 {
            return Some(RedSandstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17004 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17037 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 17039 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 16942 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 16991 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 17146 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17092 {
            return Some(RedSandstoneWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17169 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17036 {
            return Some(RedSandstoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17116 {
            return Some(RedSandstoneWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17130 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17061 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 17030 {
            return Some(RedSandstoneWall {
                r#north: North::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 16945 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 17207 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 17255 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17051 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 17212 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 16980 {
            return Some(RedSandstoneWall {
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 17104 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17214 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 16941 {
            return Some(RedSandstoneWall {
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 17034 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17059 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 17124 {
            return Some(RedSandstoneWall {
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17166 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 17158 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 17108 {
            return Some(RedSandstoneWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 17121 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 17132 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 17190 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 17053 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 16973 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 17180 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 16988 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 17027 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17065 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 17154 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17202 {
            return Some(RedSandstoneWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17003 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17204 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 17066 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17228 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17047 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17232 {
            return Some(RedSandstoneWall {
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17091 {
            return Some(RedSandstoneWall {
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17074 {
            return Some(RedSandstoneWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 17138 {
            return Some(RedSandstoneWall {
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 17248 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17075 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 17201 {
            return Some(RedSandstoneWall {
                r#north: North::Low,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 16984 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 17015 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17103 {
            return Some(RedSandstoneWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17014 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17085 {
            return Some(RedSandstoneWall {
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 17182 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 16969 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 16954 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 16951 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 16961 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 16950 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 17242 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 17181 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17194 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 17019 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17209 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 16990 {
            return Some(RedSandstoneWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16972 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17013 {
            return Some(RedSandstoneWall {
                r#south: South::None,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17093 {
            return Some(RedSandstoneWall {
                r#south: South::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17095 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17114 {
            return Some(RedSandstoneWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17176 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17005 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 17238 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 17164 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17101 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17129 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17220 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17236 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17257 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 16952 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 17263 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 17139 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 16968 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 16971 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 16998 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17137 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17072 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 16958 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 16964 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17071 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 17148 {
            return Some(RedSandstoneWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 16948 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 17162 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 17094 {
            return Some(RedSandstoneWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 17178 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 17217 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17187 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17026 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17157 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 16979 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 17167 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 17192 {
            return Some(RedSandstoneWall {
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 16962 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 16997 {
            return Some(RedSandstoneWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 17052 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17088 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17219 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 17025 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 17068 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 17100 {
            return Some(RedSandstoneWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17150 {
            return Some(RedSandstoneWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17028 {
            return Some(RedSandstoneWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17111 {
            return Some(RedSandstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 17200 {
            return Some(RedSandstoneWall {
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 17221 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16959 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17107 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 16965 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 17230 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17001 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17227 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 16956 {
            return Some(RedSandstoneWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 17078 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 17090 {
            return Some(RedSandstoneWall {
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 17110 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17140 {
            return Some(RedSandstoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17203 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17213 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 17250 {
            return Some(RedSandstoneWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17237 {
            return Some(RedSandstoneWall {
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 17042 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17045 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17243 {
            return Some(RedSandstoneWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 16955 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 17199 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 17211 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17244 {
            return Some(RedSandstoneWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 16953 {
            return Some(RedSandstoneWall {
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17002 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17040 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 16999 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17081 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 17185 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17136 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17253 {
            return Some(RedSandstoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17259 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17168 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 16946 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 17080 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 17156 {
            return Some(RedSandstoneWall {
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17260 {
            return Some(RedSandstoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 17155 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 16963 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 17233 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17196 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 17186 {
            return Some(RedSandstoneWall {
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 16957 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#up: true,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 17229 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 17054 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 17173 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 17041 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17159 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 17050 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 17143 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17048 {
            return Some(RedSandstoneWall {
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17206 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17147 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17223 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17246 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 17258 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 17262 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 17197 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 17119 {
            return Some(RedSandstoneWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 17126 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17069 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 17106 {
            return Some(RedSandstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 16993 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17171 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17079 {
            return Some(RedSandstoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 17084 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 17249 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 17032 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17012 {
            return Some(RedSandstoneWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 16983 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 17055 {
            return Some(RedSandstoneWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 17076 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17133 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 17161 {
            return Some(RedSandstoneWall {
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 17062 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17189 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 16974 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 17115 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17123 {
            return Some(RedSandstoneWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17165 {
            return Some(RedSandstoneWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 17177 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 16970 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 17049 {
            return Some(RedSandstoneWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17097 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17070 {
            return Some(RedSandstoneWall {
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17046 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 16978 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 16981 {
            return Some(RedSandstoneWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 17056 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17118 {
            return Some(RedSandstoneWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17179 {
            return Some(RedSandstoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 16994 {
            return Some(RedSandstoneWall {
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 17125 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 17141 {
            return Some(RedSandstoneWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17063 {
            return Some(RedSandstoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 17135 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17112 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 17149 {
            return Some(RedSandstoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 16987 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 16976 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17195 {
            return Some(RedSandstoneWall {
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 17218 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17191 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17222 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17120 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17240 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#up: true,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17188 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 16996 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 17018 {
            return Some(RedSandstoneWall {
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17245 {
            return Some(RedSandstoneWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17251 {
            return Some(RedSandstoneWall {
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17145 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Low,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17023 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 16944 {
            return Some(RedSandstoneWall {
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 17011 {
            return Some(RedSandstoneWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 16975 {
            return Some(RedSandstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17067 {
            return Some(RedSandstoneWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 17098 {
            return Some(RedSandstoneWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 16992 {
            return Some(RedSandstoneWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 17096 {
            return Some(RedSandstoneWall {
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 17172 {
            return Some(RedSandstoneWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        return None;
    }
}

