use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedDeepslateWall {
    pub r#east: East,
    pub up: bool,
    pub r#west: West,
    pub r#north: North,
    pub waterlogged: bool,
    pub r#south: South,
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

impl BlockState for PolishedDeepslateWall {
    fn to_id(self) -> i32 {
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 28351; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 28425; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 28284; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::None { return 28510; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::None { return 28268; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 28538; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#waterlogged == false { return 28244; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None { return 28521; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low { return 28523; }
        if block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 28498; }
        if block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == false { return 28243; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Tall { return 28251; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Tall { return 28438; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == false { return 28228; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::None { return 28234; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::Low { return 28293; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low { return 28238; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false { return 28483; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Low { return 28385; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall { return 28522; }
        if block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 28423; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::None { return 28276; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == false { return 28457; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true { return 28330; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 28450; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Low { return 28280; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 28464; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 28479; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true { return 28242; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 28254; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::None { return 28373; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#west == West::None { return 28402; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 28416; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true { return 28440; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None { return 28473; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 28537; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None { return 28322; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::Tall { return 28360; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::Tall { return 28314; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::None { return 28478; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#west == West::None { return 28513; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None { return 28225; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 28418; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 28265; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::None { return 28269; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 28387; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 28509; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 28317; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Low { return 28352; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 28295; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::None { return 28267; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 28367; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 28436; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::None { return 28287; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true { return 28501; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 28398; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 28475; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall { return 28390; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::None { return 28341; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#waterlogged == true { return 28444; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::Low { return 28544; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::None { return 28411; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#waterlogged == false { return 28292; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == false { return 28458; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 28535; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low { return 28347; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None { return 28363; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 28256; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 28308; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 28514; }
        if block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == true { return 28453; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == true { return 28318; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 28285; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == true { return 28306; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 28353; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Tall { return 28338; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 28391; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false { return 28249; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 28320; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 28296; }
        if block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 28272; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true { return 28359; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::None { return 28408; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 28410; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#west == West::Low { return 28472; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 28428; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == false { return 28505; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 28328; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true { return 28430; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::None { return 28231; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::None { return 28460; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 28530; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 28365; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::None { return 28439; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 28229; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 28264; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low { return 28289; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Tall { return 28506; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Tall { return 28278; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::None { return 28340; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low { return 28388; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true { return 28470; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::None { return 28339; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::None { return 28356; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 28394; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 28395; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 28508; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 28532; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 28329; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall { return 28224; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 28253; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::None { return 28274; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 28326; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::Low { return 28526; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#west == West::None { return 28465; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true { return 28462; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Low { return 28381; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 28476; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::None { return 28241; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Low { return 28382; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::None { return 28441; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#waterlogged == true { return 28222; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 28496; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::None { return 28261; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 28315; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 28286; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == true { return 28491; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true { return 28343; }
        if block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#up == true { return 28355; }
        if block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::None { return 28404; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::Tall { return 28469; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Low { return 28331; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 28319; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#up == false { return 28401; }
        if block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 28247; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 28536; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::Low { return 28361; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::None { return 28409; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 28375; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#up == false { return 28433; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 28454; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false { return 28291; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 28349; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Low { return 28452; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::None { return 28461; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::None { return 28370; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true { return 28455; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 28298; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false { return 28257; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 28283; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::None { return 28233; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == false { return 28312; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::None { return 28443; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 28226; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 28487; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::Tall { return 28519; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::None { return 28414; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 28511; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Low { return 28380; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == false { return 28493; }
        if block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::None { return 28252; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Low { return 28383; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::Low { return 28232; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::Tall { return 28497; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#up == true { return 28503; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true { return 28301; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Low { return 28342; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false { return 28447; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Tall { return 28545; }
        if block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 28290; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::Low { return 28345; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#waterlogged == false { return 28449; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::None { return 28259; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 28500; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 28230; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#south == South::None { return 28302; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall { return 28542; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 28451; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 28543; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 28277; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 28397; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 28541; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true { return 28358; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 28424; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::None { return 28405; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None { return 28337; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == true { return 28237; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 28275; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#waterlogged == false { return 28334; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::Low { return 28393; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 28486; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::None { return 28303; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == true { return 28431; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 28240; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::Low { return 28307; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 28323; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true { return 28299; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 28456; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == false { return 28468; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::Low { return 28481; }
        if block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 28489; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true { return 28348; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Low { return 28527; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 28311; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == false { return 28529; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#north == North::None { return 28364; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::None { return 28304; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#waterlogged == true { return 28420; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == true { return 28490; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 28539; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::None { return 28336; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Low { return 28316; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 28482; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 28260; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false { return 28313; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::None { return 28495; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 28504; }
        if block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::None { return 28300; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 28389; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None { return 28366; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 28250; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low { return 28399; }
        if block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false { return 28245; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 28422; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 28413; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 28485; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false { return 28494; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 28502; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 28515; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 28534; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::Low { return 28239; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Low { return 28499; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == true { return 28248; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::None { return 28273; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true { return 28512; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true { return 28446; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low { return 28333; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Low { return 28346; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::Tall { return 28466; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::None { return 28350; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 28520; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false { return 28305; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true { return 28332; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 28434; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Low { return 28492; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall { return 28445; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None { return 28223; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 28255; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::Low { return 28262; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#up == true { return 28415; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Tall { return 28419; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 28467; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::None { return 28518; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 28427; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == false { return 28327; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == false { return 28477; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Tall { return 28297; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::Low { return 28378; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 28396; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::None { return 28531; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Low { return 28369; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 28362; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall { return 28386; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 28266; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 28235; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall { return 28324; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 28325; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 28400; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::None { return 28429; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 28442; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 28421; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true { return 28294; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 28471; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 28517; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#up == true { return 28344; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::Low { return 28417; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 28403; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::None { return 28335; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == true { return 28463; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 28376; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 28474; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 28392; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true { return 28236; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#up == true { return 28309; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 28371; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 28281; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None { return 28288; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::Low { return 28368; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true { return 28270; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low { return 28310; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 28263; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 28321; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::None { return 28412; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 28432; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::None { return 28435; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None { return 28354; }
        if block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::None { return 28258; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 28279; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 28437; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 28484; }
        if block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None { return 28282; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall { return 28246; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true { return 28488; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 28540; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false { return 28480; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 28426; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 28407; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 28528; }
        if block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == false { return 28372; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::None { return 28227; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false { return 28384; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::Tall { return 28406; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#up == true { return 28271; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#up == false { return 28459; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None { return 28525; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == false { return 28533; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Low { return 28374; }
        if block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 28377; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::Low { return 28379; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 28507; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#up == false { return 28516; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#west == West::None { return 28357; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 28524; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == false { return 28448; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 28351 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28425 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 28284 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28510 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 28268 {
            return Some(PolishedDeepslateWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 28538 {
            return Some(PolishedDeepslateWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28244 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28521 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 28523 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28498 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28243 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28251 {
            return Some(PolishedDeepslateWall {
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28438 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 28228 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 28234 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 28293 {
            return Some(PolishedDeepslateWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 28238 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28483 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28385 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28522 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 28423 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28276 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 28457 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 28330 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28450 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28280 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 28464 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 28479 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28242 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28254 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28373 {
            return Some(PolishedDeepslateWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 28402 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 28416 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28440 {
            return Some(PolishedDeepslateWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28473 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 28537 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28322 {
            return Some(PolishedDeepslateWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 28360 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 28314 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 28478 {
            return Some(PolishedDeepslateWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 28513 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 28225 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 28418 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 28265 {
            return Some(PolishedDeepslateWall {
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 28269 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 28387 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 28509 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28317 {
            return Some(PolishedDeepslateWall {
                r#west: West::Tall,
                r#up: false,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 28352 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28295 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 28267 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 28367 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 28436 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28287 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 28501 {
            return Some(PolishedDeepslateWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 28398 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28475 {
            return Some(PolishedDeepslateWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28390 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28341 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 28444 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28544 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 28411 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 28292 {
            return Some(PolishedDeepslateWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28458 {
            return Some(PolishedDeepslateWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 28535 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28347 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 28363 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 28256 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28308 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28514 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 28453 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 28318 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 28285 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 28306 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28353 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28338 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28391 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28249 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28320 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28296 {
            return Some(PolishedDeepslateWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28272 {
            return Some(PolishedDeepslateWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 28359 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 28408 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 28410 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28472 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 28428 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28505 {
            return Some(PolishedDeepslateWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 28328 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28430 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 28231 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 28460 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 28530 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 28365 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28439 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 28229 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 28264 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 28289 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 28506 {
            return Some(PolishedDeepslateWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 28278 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28340 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 28388 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28470 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28339 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 28356 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 28394 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28395 {
            return Some(PolishedDeepslateWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 28508 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28532 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 28329 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28224 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28253 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28274 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 28326 {
            return Some(PolishedDeepslateWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 28526 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28465 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 28462 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 28381 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 28476 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 28241 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 28382 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28441 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 28222 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28496 {
            return Some(PolishedDeepslateWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28261 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 28315 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28286 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28491 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 28343 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 28355 {
            return Some(PolishedDeepslateWall {
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 28404 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 28469 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28331 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 28319 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 28401 {
            return Some(PolishedDeepslateWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 28247 {
            return Some(PolishedDeepslateWall {
                r#west: West::Low,
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28536 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28361 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28409 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 28375 {
            return Some(PolishedDeepslateWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28433 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 28454 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 28291 {
            return Some(PolishedDeepslateWall {
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28349 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 28452 {
            return Some(PolishedDeepslateWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28461 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 28370 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 28455 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 28298 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28257 {
            return Some(PolishedDeepslateWall {
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 28283 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 28233 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 28312 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 28443 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 28226 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28487 {
            return Some(PolishedDeepslateWall {
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28519 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 28414 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 28511 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 28380 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 28493 {
            return Some(PolishedDeepslateWall {
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 28252 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 28383 {
            return Some(PolishedDeepslateWall {
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 28232 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 28497 {
            return Some(PolishedDeepslateWall {
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 28503 {
            return Some(PolishedDeepslateWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 28301 {
            return Some(PolishedDeepslateWall {
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28342 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28447 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 28545 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 28290 {
            return Some(PolishedDeepslateWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 28345 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 28449 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28259 {
            return Some(PolishedDeepslateWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 28500 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28230 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28302 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 28542 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28451 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28543 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 28277 {
            return Some(PolishedDeepslateWall {
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 28397 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28541 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 28358 {
            return Some(PolishedDeepslateWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 28424 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28405 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 28337 {
            return Some(PolishedDeepslateWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 28237 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 28275 {
            return Some(PolishedDeepslateWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28334 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28393 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 28486 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28303 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 28431 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 28240 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28307 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28323 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28299 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 28456 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 28468 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 28481 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28489 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 28348 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28527 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28311 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28529 {
            return Some(PolishedDeepslateWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 28364 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 28304 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 28420 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28490 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 28539 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28336 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 28316 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 28482 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 28260 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 28313 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 28495 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 28504 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28300 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 28389 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 28366 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 28250 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#up: true,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28399 {
            return Some(PolishedDeepslateWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 28245 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28422 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 28413 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28485 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 28494 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 28502 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28515 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28534 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28239 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 28499 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 28248 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 28273 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 28512 {
            return Some(PolishedDeepslateWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 28446 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28333 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 28346 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28466 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28350 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 28520 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28305 {
            return Some(PolishedDeepslateWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28332 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28434 {
            return Some(PolishedDeepslateWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28492 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 28445 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 28223 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 28255 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28262 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 28415 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 28419 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28467 {
            return Some(PolishedDeepslateWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28518 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 28427 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 28327 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 28477 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 28297 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 28378 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 28396 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28531 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 28369 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 28362 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28386 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28266 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28235 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 28324 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28325 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 28400 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28429 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 28442 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28421 {
            return Some(PolishedDeepslateWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 28294 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 28471 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28517 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 28344 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 28417 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28403 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 28335 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 28463 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 28376 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28474 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28392 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28236 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 28309 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 28371 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28281 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 28288 {
            return Some(PolishedDeepslateWall {
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 28368 {
            return Some(PolishedDeepslateWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 28270 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28310 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28263 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28321 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28412 {
            return Some(PolishedDeepslateWall {
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 28432 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28435 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 28354 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 28258 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 28279 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 28437 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 28484 {
            return Some(PolishedDeepslateWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 28282 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 28246 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28488 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 28540 {
            return Some(PolishedDeepslateWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 28480 {
            return Some(PolishedDeepslateWall {
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 28426 {
            return Some(PolishedDeepslateWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 28407 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28528 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 28372 {
            return Some(PolishedDeepslateWall {
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 28227 {
            return Some(PolishedDeepslateWall {
                r#east: East::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 28384 {
            return Some(PolishedDeepslateWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 28406 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 28271 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 28459 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 28525 {
            return Some(PolishedDeepslateWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 28533 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 28374 {
            return Some(PolishedDeepslateWall {
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 28377 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 28379 {
            return Some(PolishedDeepslateWall {
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 28507 {
            return Some(PolishedDeepslateWall {
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 28516 {
            return Some(PolishedDeepslateWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 28357 {
            return Some(PolishedDeepslateWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 28524 {
            return Some(PolishedDeepslateWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28448 {
            return Some(PolishedDeepslateWall {
                r#south: South::None,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        return None;
    }
}

