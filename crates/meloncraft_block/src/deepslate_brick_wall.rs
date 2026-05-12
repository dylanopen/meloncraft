use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateBrickWall {
    pub r#north: North,
    pub waterlogged: bool,
    pub up: bool,
    pub r#west: West,
    pub r#south: South,
    pub r#east: East,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum North {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum East {
    None,
    Low,
    Tall,
}

impl BlockState for DeepslateBrickWall {
    fn to_id(self) -> i32 {
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 29292; }
        if block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false { return 29121; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 29208; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 29209; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::None { return 29274; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Low { return 29193; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Tall { return 29181; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::Tall { return 29229; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 29075; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::None { return 29152; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 29279; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 29254; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 29255; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 29285; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 29352; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 29092; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low { return 29204; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 29046; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::None { return 29054; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == true { return 29263; }
        if block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 29151; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true { return 29068; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == false { return 29147; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 29252; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 29315; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 29319; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 29132; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::None { return 29269; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#up == false { return 29113; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 29228; }
        if block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::None { return 29082; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Tall { return 29073; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 29309; }
        if block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 29091; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Low { return 29219; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 29336; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::None { return 29110; }
        if block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 29186; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == false { return 29078; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false { return 29131; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall { return 29223; }
        if block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::None { return 29125; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 29249; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 29057; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Low { return 29182; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 29355; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 29080; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == true { return 29345; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == false { return 29135; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::None { return 29165; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::None { return 29168; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::None { return 29176; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::Tall { return 29296; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low { return 29174; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == true { return 29313; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::None { return 29272; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::Tall { return 29360; }
        if block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == true { return 29049; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 29251; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall { return 29270; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Low { return 29198; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Low { return 29280; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall { return 29356; }
        if block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == true { return 29107; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall { return 29217; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::None { return 29095; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 29210; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::Tall { return 29271; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 29056; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true { return 29081; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Low { return 29120; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 29129; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Low { return 29096; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::None { return 29299; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 29220; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true { return 29297; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == true { return 29133; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 29298; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 29138; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == false { return 29126; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 29211; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 29225; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 29238; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Low { return 29328; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Tall { return 29366; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 29062; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == true { return 29048; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 29169; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Tall { return 29334; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 29137; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 29141; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 29357; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 29136; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::Tall { return 29350; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::None { return 29077; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 29346; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 29205; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::None { return 29335; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::None { return 29119; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Low { return 29111; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::Low { return 29171; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::None { return 29118; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 29214; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::Low { return 29308; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low { return 29103; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 29288; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 29333; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 29108; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 29112; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 29261; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true { return 29312; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == true { return 29240; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true { return 29260; }
        if block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#up == true { return 29224; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 29295; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 29243; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == false { return 29303; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::None { return 29306; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Low { return 29100; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 29363; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Tall { return 29226; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 29367; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None { return 29154; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true { return 29236; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::None { return 29047; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::Low { return 29207; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 29283; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 29358; }
        if block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true { return 29167; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == true { return 29325; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 29242; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 29314; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Low { return 29164; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 29282; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == false { return 29066; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Low { return 29173; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true { return 29230; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false { return 29294; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false { return 29180; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None { return 29340; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == false { return 29353; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None { return 29293; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true { return 29060; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 29307; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 29258; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 29326; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true { return 29134; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true { return 29277; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == false { return 29101; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::Tall { return 29362; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#up == true { return 29202; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 29098; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::Tall { return 29106; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 29191; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 29291; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None { return 29177; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == false { return 29064; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 29140; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == false { return 29115; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::None { return 29359; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None { return 29178; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 29259; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::Tall { return 29351; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 29344; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Tall { return 29245; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::None { return 29124; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::None { return 29051; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 29105; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 29349; }
        if block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false { return 29185; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Low { return 29195; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 29199; }
        if block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 29227; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::Tall { return 29257; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Tall { return 29071; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 29059; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false { return 29317; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false { return 29187; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 29250; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == false { return 29065; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 29203; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 29109; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::None { return 29122; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Tall { return 29231; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 29268; }
        if block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 29330; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 29232; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true { return 29069; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 29159; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#up == false { return 29175; }
        if block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true { return 29194; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == true { return 29284; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::None { return 29067; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == true { return 29332; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::Low { return 29123; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false { return 29072; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Low { return 29090; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::None { return 29044; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::None { return 29161; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 29127; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None { return 29262; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true { return 29264; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 29234; }
        if block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == true { return 29070; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == true { return 29117; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false { return 29343; }
        if block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true { return 29158; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 29179; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::None { return 29085; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 29163; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false { return 29221; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::None { return 29050; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::None { return 29052; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true { return 29320; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::None { return 29248; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true { return 29086; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Low { return 29216; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false { return 29323; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#west == West::Low { return 29144; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::Low { return 29093; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false { return 29302; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 29322; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 29089; }
        if block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::None { return 29058; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#east == East::None { return 29088; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::None { return 29304; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None { return 29104; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 29235; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 29329; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == false { return 29256; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low { return 29172; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::Low { return 29218; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::Low { return 29324; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 29213; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false { return 29338; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 29347; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 29267; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Low { return 29099; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 29237; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 29166; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#up == true { return 29321; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::None { return 29116; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::None { return 29273; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 29061; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 29266; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::None { return 29192; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Tall { return 29247; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Low { return 29097; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None { return 29155; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 29190; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 29094; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 29045; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::Tall { return 29114; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#west == West::Tall { return 29184; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 29215; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 29244; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None { return 29341; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 29342; }
        if block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == false { return 29365; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 29196; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 29139; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Low { return 29200; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == true { return 29130; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::None { return 29145; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Low { return 29239; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 29348; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false { return 29162; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::Low { return 29197; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 29331; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false { return 29281; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 29156; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low { return 29246; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Low { return 29318; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 29361; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 29301; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 29253; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false { return 29276; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low { return 29087; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 29287; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Low { return 29305; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Tall { return 29233; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 29337; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#north == North::Low { return 29310; }
        if block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 29222; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None { return 29146; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::Tall { return 29311; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::Low { return 29189; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 29079; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false { return 29275; }
        if block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 29148; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 29160; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Low { return 29201; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == true { return 29153; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false { return 29278; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::None { return 29055; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 29316; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#up == false { return 29327; }
        if block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Low { return 29212; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == false { return 29053; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true { return 29083; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 29286; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Low { return 29339; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 29364; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 29063; }
        if block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::None { return 29300; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true { return 29170; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Low { return 29084; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::None { return 29289; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::None { return 29128; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::None { return 29143; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 29142; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true { return 29290; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false { return 29076; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 29206; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::Tall { return 29149; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 29150; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Low { return 29102; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 29241; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false { return 29183; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 29188; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::None { return 29157; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::Tall { return 29074; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 29354; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::Tall { return 29265; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29292 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 29121 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 29208 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 29209 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 29274 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 29193 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29181 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29229 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 29075 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 29152 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 29279 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 29254 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 29255 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 29285 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 29352 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 29092 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 29204 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 29046 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29054 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 29263 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 29151 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 29068 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 29147 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 29252 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29315 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 29319 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 29132 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 29269 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 29113 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 29228 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29082 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 29073 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29309 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 29091 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 29219 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 29336 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 29110 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 29186 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 29078 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 29131 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 29223 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 29125 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 29249 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 29057 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 29182 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 29355 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 29080 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 29345 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 29135 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 29165 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 29168 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 29176 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 29296 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 29174 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 29313 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 29272 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 29360 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 29049 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 29251 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 29270 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 29198 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 29280 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 29356 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 29107 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 29217 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29095 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 29210 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 29271 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 29056 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 29081 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 29120 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 29129 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 29096 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 29299 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 29220 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 29297 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 29133 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 29298 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 29138 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 29126 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 29211 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29225 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 29238 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29328 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 29366 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 29062 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 29048 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 29169 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 29334 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29137 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29141 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 29357 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 29136 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29350 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 29077 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 29346 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 29205 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 29335 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 29119 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 29111 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 29171 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 29118 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 29214 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29308 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 29103 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 29288 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 29333 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 29108 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 29112 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29261 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 29312 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 29240 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 29260 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 29224 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 29295 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 29243 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 29303 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 29306 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 29100 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 29363 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 29226 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 29367 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 29154 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 29236 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 29047 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 29207 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 29283 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 29358 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 29167 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 29325 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 29242 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 29314 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 29164 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 29282 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 29066 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 29173 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 29230 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 29294 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 29180 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 29340 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 29353 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 29293 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 29060 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 29307 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 29258 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 29326 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 29134 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 29277 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 29101 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 29362 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 29202 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 29098 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 29106 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 29191 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 29291 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 29177 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 29064 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 29140 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 29115 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 29359 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 29178 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 29259 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 29351 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 29344 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 29245 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 29124 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 29051 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 29105 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 29349 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 29185 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 29195 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 29199 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 29227 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 29257 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 29071 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 29059 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 29317 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 29187 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 29250 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 29065 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 29203 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 29109 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 29122 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 29231 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 29268 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 29330 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29232 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 29069 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 29159 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 29175 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 29194 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#west: West::None,
                r#up: false,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 29284 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 29067 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 29332 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 29123 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 29072 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 29090 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29044 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 29161 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 29127 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 29262 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 29264 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 29234 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 29070 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 29117 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 29343 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 29158 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 29179 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 29085 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 29163 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 29221 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 29050 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 29052 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 29320 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 29248 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 29086 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 29216 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29323 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 29144 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 29093 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 29302 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 29322 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29089 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 29058 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 29088 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 29304 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 29104 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 29235 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29329 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 29256 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 29172 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 29218 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 29324 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 29213 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 29338 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 29347 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 29267 {
            return Some(DeepslateBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 29099 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 29237 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 29166 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 29321 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 29116 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 29273 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 29061 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 29266 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 29192 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 29247 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 29097 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29155 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 29190 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 29094 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 29045 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#up: true,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 29114 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 29184 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 29215 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 29244 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29341 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 29342 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 29365 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 29196 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29139 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 29200 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 29130 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 29145 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 29239 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 29348 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 29162 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 29197 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 29331 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 29281 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 29156 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 29246 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 29318 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29361 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 29301 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 29253 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29276 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 29087 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29287 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29305 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 29233 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 29337 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 29310 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29222 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 29146 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 29311 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 29189 {
            return Some(DeepslateBrickWall {
                r#south: South::None,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 29079 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 29275 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 29148 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 29160 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 29201 {
            return Some(DeepslateBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 29153 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 29278 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 29055 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 29316 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 29327 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 29212 {
            return Some(DeepslateBrickWall {
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 29053 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 29083 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 29286 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 29339 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 29364 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 29063 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 29300 {
            return Some(DeepslateBrickWall {
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 29170 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 29084 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 29289 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 29128 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 29143 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 29142 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 29290 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 29076 {
            return Some(DeepslateBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 29206 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 29149 {
            return Some(DeepslateBrickWall {
                r#east: East::None,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 29150 {
            return Some(DeepslateBrickWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 29102 {
            return Some(DeepslateBrickWall {
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 29241 {
            return Some(DeepslateBrickWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 29183 {
            return Some(DeepslateBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 29188 {
            return Some(DeepslateBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 29157 {
            return Some(DeepslateBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 29074 {
            return Some(DeepslateBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 29354 {
            return Some(DeepslateBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 29265 {
            return Some(DeepslateBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        return None;
    }
}

