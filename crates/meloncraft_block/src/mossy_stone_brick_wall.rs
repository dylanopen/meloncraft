use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MossyStoneBrickWall {
    pub r#north: North,
    pub up: bool,
    pub waterlogged: bool,
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

impl BlockState for MossyStoneBrickWall {
    fn to_id(self) -> i32 {
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#waterlogged == false { return 17411; }
        if block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true { return 17480; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::None { return 17306; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == false { return 17587; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#up == true { return 17553; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::None { return 17330; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 17428; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::None { return 17549; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::None { return 17272; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::Low { return 17385; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None { return 17340; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 17402; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == false { return 17426; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Tall { return 17514; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall { return 17533; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None { return 17264; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Low { return 17391; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == true { return 17436; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true { return 17577; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 17575; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 17586; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Low { return 17427; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::None { return 17379; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::None { return 17507; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 17518; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == true { return 17364; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None { return 17488; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None { return 17360; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true { return 17541; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 17479; }
        if block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 17558; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 17349; }
        if block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Low { return 17352; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None { return 17412; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::None { return 17308; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::Tall { return 17495; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 17400; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None { return 17512; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#up == false { return 17513; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 17578; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Tall { return 17500; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Low { return 17310; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 17344; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 17431; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Low { return 17439; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#up == false { return 17414; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == true { return 17409; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == true { return 17408; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 17447; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false { return 17484; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 17567; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true { return 17336; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#north == North::Low { return 17550; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::None { return 17390; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == true { return 17302; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true { return 17303; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true { return 17265; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::None { return 17282; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false { return 17415; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::None { return 17274; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::Tall { return 17389; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 17440; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Low { return 17471; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 17353; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 17346; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Low { return 17461; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Tall { return 17362; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true { return 17523; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::Low { return 17517; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low { return 17530; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 17475; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None { return 17269; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#waterlogged == false { return 17496; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::Tall { return 17555; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::None { return 17275; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == false { return 17294; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == false { return 17323; }
        if block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::Tall { return 17510; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Tall { return 17562; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Low { return 17319; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true { return 17568; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::None { return 17370; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low { return 17538; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low { return 17432; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == true { return 17398; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None { return 17286; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::None { return 17477; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 17532; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 17437; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 17334; }
        if block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == true { return 17374; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#north == North::None { return 17494; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 17497; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 17547; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None { return 17325; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::None { return 17332; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None { return 17450; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 17540; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::None { return 17508; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Low { return 17321; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false { return 17335; }
        if block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#waterlogged == false { return 17489; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 17442; }
        if block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 17327; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::None { return 17401; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::Tall { return 17348; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::None { return 17405; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false { return 17404; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 17416; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Low { return 17441; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 17458; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Tall { return 17516; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Tall { return 17522; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 17457; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Low { return 17566; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::None { return 17276; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::None { return 17296; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 17397; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::None { return 17474; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 17466; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == false { return 17291; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::None { return 17289; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::Low { return 17429; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 17565; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == false { return 17320; }
        if block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 17539; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::None { return 17339; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 17478; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::Tall { return 17357; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall { return 17377; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::None { return 17419; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 17519; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Tall { return 17542; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true { return 17372; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 17454; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::None { return 17271; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::None { return 17333; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 17452; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false { return 17382; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == false { return 17430; }
        if block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false { return 17359; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 17383; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 17476; }
        if block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 17388; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::Tall { return 17467; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 17448; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 17365; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::None { return 17504; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Low { return 17529; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Tall { return 17569; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None { return 17394; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::None { return 17297; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 17556; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 17309; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None { return 17268; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 17301; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::Low { return 17311; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::None { return 17347; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None { return 17270; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 17468; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true { return 17351; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::None { return 17493; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false { return 17548; }
        if block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 17386; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 17481; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Tall { return 17295; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low { return 17531; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::Tall { return 17506; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Tall { return 17324; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 17356; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall { return 17366; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Low { return 17307; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false { return 17499; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true { return 17483; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Low { return 17422; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 17283; }
        if block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false { return 17376; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 17535; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 17543; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#east == East::Tall { return 17561; }
        if block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false { return 17285; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Low { return 17328; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 17453; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Tall { return 17551; }
        if block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true { return 17498; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 17410; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true { return 17318; }
        if block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 17392; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == true { return 17423; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::None { return 17491; }
        if block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::Tall { return 17524; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 17560; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::None { return 17361; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 17525; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 17298; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low { return 17438; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#up == true { return 17456; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None { return 17338; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 17341; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::None { return 17267; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 17473; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#up == true { return 17292; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Low { return 17505; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 17482; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Tall { return 17342; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Low { return 17378; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#north == North::None { return 17509; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Low { return 17315; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 17527; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 17443; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::None { return 17345; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 17446; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 17329; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Tall { return 17469; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Tall { return 17287; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low { return 17413; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == true { return 17579; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::None { return 17582; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 17433; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::None { return 17293; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 17424; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == false { return 17331; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Tall { return 17552; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Low { return 17358; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false { return 17574; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::None { return 17300; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall { return 17570; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Low { return 17420; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 17421; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true { return 17460; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None { return 17492; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::None { return 17520; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 17322; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true { return 17581; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 17583; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#up == false { return 17490; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == false { return 17585; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::None { return 17343; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 17407; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == false { return 17486; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::None { return 17375; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false { return 17451; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 17290; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true { return 17521; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Tall { return 17288; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 17314; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Low { return 17395; }
        if block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 17564; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::None { return 17284; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall { return 17371; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Low { return 17537; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 17455; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 17470; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall { return 17572; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 17326; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 17418; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 17355; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true { return 17316; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 17277; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 17266; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::None { return 17279; }
        if block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low { return 17399; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false { return 17403; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 17425; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 17557; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 17584; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 17350; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == false { return 17380; }
        if block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::Tall { return 17396; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false { return 17534; }
        if block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 17393; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None { return 17503; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::Tall { return 17502; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Tall { return 17368; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 17463; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 17536; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low { return 17434; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Tall { return 17554; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::Low { return 17337; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 17445; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#up == true { return 17485; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 17278; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 17317; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false { return 17299; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::None { return 17280; }
        if block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Low { return 17384; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 17462; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::Tall { return 17464; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None { return 17559; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Tall { return 17501; }
        if block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 17576; }
        if block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == false { return 17571; }
        if block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == true { return 17373; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 17312; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == true { return 17459; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false { return 17465; }
        if block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 17528; }
        if block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::Low { return 17417; }
        if block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == false { return 17573; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 17580; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::None { return 17273; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::None { return 17281; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None { return 17363; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false { return 17354; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 17435; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 17544; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 17369; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == true { return 17545; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 17313; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None { return 17305; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::None { return 17367; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 17472; }
        if block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 17387; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 17406; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 17515; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::None { return 17381; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 17449; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 17487; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 17444; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false { return 17511; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 17526; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 17546; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#north == North::Low { return 17304; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::None { return 17563; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 17411 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17480 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17306 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 17587 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 17553 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 17330 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 17428 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17549 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 17272 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 17385 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17340 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 17402 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17426 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 17514 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17533 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17264 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 17391 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17436 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 17577 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17575 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 17586 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17427 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 17379 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 17507 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17518 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17364 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17488 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 17360 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17541 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 17479 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17558 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17349 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17352 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 17412 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 17308 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 17495 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 17400 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 17512 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 17513 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 17578 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17500 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 17310 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 17344 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17431 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 17439 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 17414 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 17409 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 17408 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17447 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17484 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17567 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17336 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17550 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17390 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::None,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 17302 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 17303 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17265 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17282 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 17415 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17274 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 17389 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 17440 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17471 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 17353 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17346 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17461 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17362 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17523 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17517 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 17530 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17475 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17269 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 17496 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17555 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17275 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 17294 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 17323 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 17510 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 17562 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17319 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 17568 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 17370 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 17538 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 17432 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 17398 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 17286 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 17477 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 17532 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17437 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17334 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17374 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17494 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 17497 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17547 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17325 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 17332 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 17450 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17540 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17508 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17321 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17335 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17489 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17442 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17327 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 17401 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17348 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17405 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 17404 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 17416 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17441 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 17458 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17516 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 17522 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 17457 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17566 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 17276 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 17296 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 17397 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17474 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 17466 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17291 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17289 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 17429 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 17565 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17320 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 17539 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 17339 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 17478 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17357 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17377 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17419 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17519 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 17542 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 17372 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 17454 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17271 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 17333 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 17452 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17382 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17430 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17359 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17383 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17476 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 17388 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 17467 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17448 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17365 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17504 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 17529 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17569 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17394 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 17297 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 17556 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 17309 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17268 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 17301 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17311 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 17347 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 17270 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 17468 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 17351 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 17493 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17548 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 17386 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17481 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17295 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17531 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 17506 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 17324 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17356 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17366 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17307 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 17499 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 17483 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17422 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17283 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17376 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17535 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 17543 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17561 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 17285 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 17328 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 17453 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17551 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17498 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17410 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17318 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17392 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17423 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 17491 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 17524 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 17560 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17361 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 17525 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17298 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17438 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17456 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 17338 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 17341 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17267 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17473 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17292 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 17505 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 17482 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17342 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17378 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 17509 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17315 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 17527 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17443 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17345 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 17446 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17329 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17469 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17287 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17413 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17579 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17582 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 17433 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17293 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17424 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17331 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17552 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17358 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 17574 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 17300 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 17570 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 17420 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 17421 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17460 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17492 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17520 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 17322 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17581 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17583 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17490 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17585 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 17343 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 17407 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17486 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 17375 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17451 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17290 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 17521 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17288 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 17314 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17395 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 17564 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#west: West::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 17284 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 17371 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17537 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 17455 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17470 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17572 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17326 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17418 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17355 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 17316 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 17277 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 17266 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17279 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 17399 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 17403 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 17425 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17557 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17584 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17350 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17380 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 17396 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 17534 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 17393 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17503 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17502 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17368 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17463 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17536 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17434 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17554 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 17337 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 17445 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17485 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 17278 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17317 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 17299 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17280 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 17384 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17462 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17464 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17559 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 17501 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17576 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 17571 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 17373 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17312 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17459 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17465 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 17528 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17417 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17573 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 17580 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17273 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 17281 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 17363 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17354 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 17435 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17544 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17369 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17545 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 17313 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17305 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 17367 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 17472 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17387 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17406 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 17515 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17381 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 17449 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17487 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17444 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 17511 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17526 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17546 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 17304 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17563 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        return None;
    }
}

