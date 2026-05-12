use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoneBrickWall {
    pub waterlogged: bool,
    pub r#west: West,
    pub r#east: East,
    pub r#south: South,
    pub r#north: North,
    pub up: bool,
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
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::None { return 17967; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == true { return 17973; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true { return 17929; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 18180; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::None { return 17921; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 17995; }
        if block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#up == true { return 18168; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#west == West::None { return 17999; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Tall { return 17917; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 18046; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Low { return 17955; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == false { return 18088; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::Tall { return 18095; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 18178; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 17982; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 18216; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false { return 18138; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Tall { return 18034; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Low { return 18032; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 18103; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Tall { return 18146; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None { return 17991; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == false { return 17944; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 18081; }
        if block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 18079; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Low { return 18198; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#up == false { return 18127; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == false { return 17940; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Low { return 17928; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true { return 17941; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 18220; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Low { return 18098; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::None { return 17923; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::None { return 17916; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Low { return 18192; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 17953; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 18018; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Tall { return 18125; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 17934; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::None { return 17930; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == false { return 17947; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 18205; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Low { return 18042; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true { return 18140; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall { return 18214; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Low { return 18097; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Low { return 18062; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::Low { return 17950; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == false { return 18139; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::None { return 18201; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 18229; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#north == North::None { return 18036; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None { return 18163; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::None { return 18157; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 18108; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 18202; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 18226; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == false { return 18065; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 18001; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None { return 18119; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::None { return 18176; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 18015; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == true { return 18096; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Low { return 18086; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#east == East::None { return 17958; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 17977; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Low { return 18031; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 18007; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == true { return 18045; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == false { return 18122; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Tall { return 18197; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#up == true { return 18121; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::None { return 17918; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 18004; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 18181; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None { return 17925; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 17998; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None { return 18128; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::Low { return 18204; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false { return 18075; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == false { return 17983; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == false { return 18053; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 18215; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::None { return 17919; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 18211; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::None { return 17957; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None { return 17913; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#north == North::None { return 18141; }
        if block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false { return 17989; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == false { return 17975; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::None { return 18026; }
        if block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall { return 18061; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#waterlogged == true { return 18068; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 18072; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 18235; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 18002; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true { return 17937; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 18120; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == false { return 17933; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::None { return 17963; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Low { return 18054; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::None { return 18131; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Low { return 18069; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Low { return 17922; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None { return 18143; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::None { return 17969; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 18024; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 18084; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 18085; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 18094; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::Low { return 18077; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Tall { return 18172; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 18144; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 18174; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 18129; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::Tall { return 18043; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 18187; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 17976; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 18219; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::Low { return 17994; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 18222; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Low { return 18105; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 18136; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 18228; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 18232; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true { return 18056; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 18193; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Tall { return 17945; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true { return 17972; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#west == West::Low { return 18153; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Low { return 18006; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 18195; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::Low { return 18021; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Low { return 18090; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true { return 18152; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 18166; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#west == West::None { return 18212; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 18225; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall { return 18199; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low { return 18117; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::Low { return 17971; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 17984; }
        if block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true { return 18134; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::None { return 18093; }
        if block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None { return 18059; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 18067; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 18231; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 18066; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == false { return 18149; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None { return 18092; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 18184; }
        if block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None { return 18161; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false { return 18078; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::Tall { return 18106; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#up == false { return 18150; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low { return 18071; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Tall { return 18009; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 18191; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 18037; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#waterlogged == true { return 18188; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::None { return 18000; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::None { return 18041; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 18080; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Low { return 17951; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == false { return 18023; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == true { return 17938; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::None { return 17980; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == true { return 18058; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false { return 18135; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Tall { return 18113; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false { return 18210; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 18177; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 18022; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 18151; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 18019; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::None { return 17986; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 18208; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 18087; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 18130; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None { return 17931; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == false { return 17968; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 17978; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 17946; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == true { return 18107; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None { return 17920; }
        if block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 17935; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == false { return 18148; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::None { return 18160; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::None { return 18167; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 17962; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true { return 17988; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 18082; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::None { return 18025; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#waterlogged == true { return 17990; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 18112; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::None { return 18200; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 18124; }
        if block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::None { return 18057; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#up == false { return 18114; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::None { return 18209; }
        if block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 18014; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::None { return 18137; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == false { return 18186; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None { return 18203; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::None { return 17970; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::None { return 18020; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::None { return 18038; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::None { return 18063; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 18104; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::None { return 17966; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 17997; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::None { return 18030; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::Tall { return 18118; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall { return 18162; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None { return 18008; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#north == North::Tall { return 18005; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None { return 18224; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 17912; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low { return 18003; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::None { return 18218; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::None { return 17954; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 17952; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#west == West::None { return 18158; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true { return 18048; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall { return 18016; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 18189; }
        if block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Low { return 18116; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::None { return 18170; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::None { return 17942; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 18010; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 18073; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 18230; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 18159; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 18154; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#waterlogged == false { return 17927; }
        if block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == false { return 17959; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 17965; }
        if block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#up == false { return 18111; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#up == false { return 18115; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Tall { return 17943; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::None { return 17914; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::None { return 17992; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall { return 18145; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::None { return 18165; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 18217; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 18012; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 18039; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false { return 18049; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#west == West::Low { return 17961; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::None { return 17915; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::None { return 18047; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#north == North::None { return 18055; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Low { return 17949; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Low { return 18033; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == false { return 18091; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall { return 18142; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Tall { return 18133; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::Tall { return 18155; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall { return 17936; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low { return 17960; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::None { return 17996; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true { return 18179; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Tall { return 18206; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low { return 18099; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == true { return 17964; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low { return 17981; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::Low { return 18101; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 18156; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 17932; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == false { return 18029; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 18052; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 18074; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::Tall { return 18164; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 18169; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::None { return 18173; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 18110; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::None { return 17985; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 18040; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall { return 18076; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 18017; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 18183; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 18190; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 18194; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 18221; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false { return 18064; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::None { return 17924; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false { return 18028; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 18035; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 18070; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 17956; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None { return 18227; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#up == false { return 18100; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::Tall { return 18132; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 18013; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false { return 17993; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::None { return 18147; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 18213; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 18223; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::None { return 18011; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 18234; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 18175; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Tall { return 17926; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 17974; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 18233; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == false { return 18027; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Low { return 18060; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None { return 18182; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#up == false { return 18089; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true { return 18109; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Low { return 17948; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Tall { return 17987; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false { return 18051; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#waterlogged == true { return 18044; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false { return 18102; }
        if block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true { return 18050; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 17979; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 18126; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 18171; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall { return 18196; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#south == South::None { return 18207; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Low { return 18083; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 18185; }
        if block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true { return 17939; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::Low { return 18123; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 17967 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 17973 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17929 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18180 {
            return Some(StoneBrickWall {
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17921 {
            return Some(StoneBrickWall {
                r#up: false,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 17995 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18168 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 17999 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 17917 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18046 {
            return Some(StoneBrickWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17955 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 18088 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 18095 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18178 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17982 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18216 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 18138 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18034 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18032 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 18103 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 18146 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 17991 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 17944 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 18081 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 18079 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18198 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 18127 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 17940 {
            return Some(StoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17928 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 17941 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 18220 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18098 {
            return Some(StoneBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 17923 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 17916 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 18192 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 17953 {
            return Some(StoneBrickWall {
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18018 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18125 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17934 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17930 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 17947 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 18205 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18042 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18140 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18214 {
            return Some(StoneBrickWall {
                r#up: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 18097 {
            return Some(StoneBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 18062 {
            return Some(StoneBrickWall {
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 17950 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 18139 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 18201 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 18229 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18036 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 18163 {
            return Some(StoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18157 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 18108 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18202 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18226 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18065 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 18001 {
            return Some(StoneBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 18119 {
            return Some(StoneBrickWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18176 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18015 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18096 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18086 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 17958 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 17977 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18031 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 18007 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18045 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18122 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 18197 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18121 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 17918 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 18004 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18181 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17925 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 17998 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18128 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 18204 {
            return Some(StoneBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 18075 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 17983 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 18053 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18215 {
            return Some(StoneBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17919 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 18211 {
            return Some(StoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17957 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 17913 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 18141 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 17989 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17975 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18026 {
            return Some(StoneBrickWall {
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 18061 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18068 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18072 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18235 {
            return Some(StoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18002 {
            return Some(StoneBrickWall {
                r#up: false,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17937 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18120 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17933 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 17963 {
            return Some(StoneBrickWall {
                r#up: true,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18054 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 18131 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 18069 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 17922 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 18143 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17969 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 18024 {
            return Some(StoneBrickWall {
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18084 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18085 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18094 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18077 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 18172 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 18144 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18174 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 18129 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18043 {
            return Some(StoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 18187 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 17976 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18219 {
            return Some(StoneBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 17994 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 18222 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 18105 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 18136 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18228 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 18232 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18056 {
            return Some(StoneBrickWall {
                r#up: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18193 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17945 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17972 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18153 {
            return Some(StoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 18006 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 18195 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18021 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 18090 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 18152 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 18166 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18212 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18225 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 18199 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18117 {
            return Some(StoneBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17971 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 17984 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18134 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18093 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 18059 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 18067 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18231 {
            return Some(StoneBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18066 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18149 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 18092 {
            return Some(StoneBrickWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18184 {
            return Some(StoneBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18161 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18078 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 18106 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 18150 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 18071 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 18009 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18191 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 18037 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18188 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18000 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 18041 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 18080 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17951 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 18023 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17938 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 17980 {
            return Some(StoneBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 18058 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 18135 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 18113 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 18210 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18177 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 18022 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18151 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18019 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17986 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18208 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18087 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18130 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17931 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 17968 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 17978 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17946 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18107 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17920 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 17935 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18148 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 18160 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18167 {
            return Some(StoneBrickWall {
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 17962 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17988 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 18082 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18025 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 17990 {
            return Some(StoneBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18112 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18200 {
            return Some(StoneBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18124 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18057 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 18114 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 18209 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 18014 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 18137 {
            return Some(StoneBrickWall {
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 18186 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 18203 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17970 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 18020 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 18038 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 18063 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 18104 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17966 {
            return Some(StoneBrickWall {
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 17997 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 18030 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 18118 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 18162 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18008 {
            return Some(StoneBrickWall {
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18005 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18224 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17912 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18003 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 18218 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17954 {
            return Some(StoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 17952 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18158 {
            return Some(StoneBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 18048 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 18016 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18189 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18116 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 18170 {
            return Some(StoneBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 17942 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 18010 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18073 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18230 {
            return Some(StoneBrickWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
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
        if state_id == 18154 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 17927 {
            return Some(StoneBrickWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17959 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 17965 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 18111 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 18115 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 17943 {
            return Some(StoneBrickWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17914 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17992 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18145 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 18165 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 18217 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18012 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 18039 {
            return Some(StoneBrickWall {
                r#up: false,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 18049 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17961 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 17915 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 18047 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18055 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17949 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 18033 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 18091 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 18142 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 18133 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18155 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 17936 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17960 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 17996 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 18179 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18206 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18099 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17964 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 17981 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 18101 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 18156 {
            return Some(StoneBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17932 {
            return Some(StoneBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18029 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 18052 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18074 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18164 {
            return Some(StoneBrickWall {
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18169 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 18173 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 18110 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 17985 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18040 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 18076 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18017 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 18183 {
            return Some(StoneBrickWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18190 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18194 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18221 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 18064 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 17924 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 18028 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 18035 {
            return Some(StoneBrickWall {
                r#up: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18070 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17956 {
            return Some(StoneBrickWall {
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 18227 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18100 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 18132 {
            return Some(StoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18013 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17993 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 18147 {
            return Some(StoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18213 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18223 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18011 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 18234 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18175 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17926 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17974 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18233 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 18027 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 18060 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 18182 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 18089 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 18109 {
            return Some(StoneBrickWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17948 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 17987 {
            return Some(StoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18051 {
            return Some(StoneBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 18044 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18102 {
            return Some(StoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18050 {
            return Some(StoneBrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17979 {
            return Some(StoneBrickWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18126 {
            return Some(StoneBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18171 {
            return Some(StoneBrickWall {
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18196 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 18207 {
            return Some(StoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 18083 {
            return Some(StoneBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 18185 {
            return Some(StoneBrickWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 17939 {
            return Some(StoneBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 18123 {
            return Some(StoneBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        return None;
    }
}

