use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MudBrickWall {
    pub r#south: South,
    pub r#north: North,
    pub up: bool,
    pub waterlogged: bool,
    pub r#west: West,
    pub r#east: East,
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

impl BlockState for MudBrickWall {
    fn to_id(self) -> i32 {
        if block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 18518; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 18488; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 18374; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Low { return 18429; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 18532; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 18553; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#up == true { return 18432; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::Low { return 18366; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 18298; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == false { return 18330; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == true { return 18392; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low { return 18509; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#south == South::Low { return 18365; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true { return 18252; }
        if block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true { return 18314; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::None { return 18485; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == false { return 18511; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::Tall { return 18340; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false { return 18341; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false { return 18293; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#up == true { return 18526; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 18543; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 18300; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == false { return 18403; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 18431; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 18548; }
        if block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 18369; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None { return 18530; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::None { return 18274; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 18447; }
        if block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::None { return 18491; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true { return 18397; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::None { return 18248; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true { return 18472; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 18451; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#up == false { return 18495; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == false { return 18315; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Low { return 18483; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 18411; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 18410; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::None { return 18260; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 18452; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 18554; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 18480; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None { return 18481; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == true { return 18549; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::None { return 18494; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::Low { return 18384; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 18442; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 18289; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 18328; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::None { return 18461; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 18484; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 18558; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true { return 18349; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Low { return 18433; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == false { return 18460; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 18258; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 18515; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true { return 18249; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 18310; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 18406; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 18449; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#up == false { return 18327; }
        if block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Tall { return 18533; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false { return 18474; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true { return 18287; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 18332; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 18436; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 18250; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == false { return 18458; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 18454; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#east == East::None { return 18326; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 18469; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true { return 18541; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 18531; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 18415; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None { return 18281; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#west == West::None { return 18473; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Tall { return 18247; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 18273; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 18385; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::None { return 18378; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 18516; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == true { return 18339; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == false { return 18482; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 18437; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 18271; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Low { return 18408; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 18555; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::None { return 18286; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 18353; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false { return 18412; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None { return 18496; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::None { return 18276; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Tall { return 18438; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 18435; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::Tall { return 18368; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Low { return 18361; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == false { return 18354; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == true { return 18381; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true { return 18457; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 18280; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false { return 18245; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 18537; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 18414; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::Low { return 18545; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == false { return 18557; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#up == false { return 18534; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::Tall { return 18424; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::None { return 18278; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 18463; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false { return 18377; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 18476; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 18305; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == false { return 18318; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 18356; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::None { return 18373; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Low { return 18329; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::Low { return 18407; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#up == true { return 18453; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == false { return 18520; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#west == West::Tall { return 18487; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 18306; }
        if block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false { return 18367; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None { return 18236; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 18535; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Low { return 18371; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::None { return 18479; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false { return 18499; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 18285; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 18337; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true { return 18241; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None { return 18338; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::Tall { return 18259; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == true { return 18416; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 18325; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#up == true { return 18320; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 18523; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Tall { return 18497; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 18502; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Low { return 18291; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::None { return 18486; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 18358; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true { return 18387; }
        if block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 18465; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Low { return 18521; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall { return 18527; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Low { return 18546; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#north == North::None { return 18264; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true { return 18528; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::None { return 18470; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == false { return 18270; }
        if block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 18417; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::None { return 18272; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::Tall { return 18434; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None { return 18466; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == true { return 18333; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low { return 18503; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == true { return 18347; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Tall { return 18514; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::None { return 18355; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::None { return 18512; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 18284; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 18551; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low { return 18510; }
        if block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::None { return 18288; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::None { return 18253; }
        if block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 18265; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#west == West::Tall { return 18478; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Low { return 18324; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::None { return 18404; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == true { return 18313; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::None { return 18239; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::None { return 18255; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 18304; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == true { return 18490; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::Low { return 18283; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::Low { return 18331; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::None { return 18246; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true { return 18372; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Low { return 18383; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 18544; }
        if block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 18418; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true { return 18443; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == false { return 18522; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::None { return 18267; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None { return 18343; }
        if block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == false { return 18307; }
        if block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true { return 18380; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::None { return 18529; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == true { return 18536; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#waterlogged == true { return 18308; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 18422; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::None { return 18244; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 18421; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == false { return 18282; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true { return 18364; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 18363; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 18448; }
        if block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 18419; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 18524; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 18504; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 18428; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 18538; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 18513; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::None { return 18299; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 18519; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == false { return 18269; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true { return 18237; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true { return 18251; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == true { return 18393; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 18444; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == false { return 18508; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall { return 18552; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::None { return 18459; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Tall { return 18441; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == true { return 18243; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None { return 18294; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 18334; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Low { return 18506; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 18277; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::None { return 18275; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::Low { return 18389; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 18475; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::Low { return 18360; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 18290; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 18352; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low { return 18357; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::None { return 18261; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 18468; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == false { return 18295; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 18266; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false { return 18350; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false { return 18376; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall { return 18370; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == true { return 18445; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None { return 18254; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 18345; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 18346; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 18336; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low { return 18257; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == true { return 18309; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::None { return 18386; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Low { return 18388; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true { return 18396; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low { return 18398; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false { return 18425; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 18446; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#up == true { return 18455; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#up == false { return 18426; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 18348; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::None { return 18240; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low { return 18394; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 18430; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None { return 18467; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 18450; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 18440; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false { return 18400; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::Tall { return 18427; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::None { return 18321; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::None { return 18464; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 18540; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 18547; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 18301; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Low { return 18323; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Low { return 18399; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == true { return 18505; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#up == true { return 18344; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#up == false { return 18317; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#waterlogged == true { return 18302; }
        if block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 18362; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == false { return 18256; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Low { return 18359; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 18391; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true { return 18493; }
        if block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 18413; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true { return 18501; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == true { return 18405; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None { return 18462; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 18303; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::None { return 18311; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::None { return 18279; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 18316; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Low { return 18296; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Tall { return 18539; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 18262; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 18517; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 18420; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == false { return 18559; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 18319; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::None { return 18242; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#up == true { return 18312; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Tall { return 18335; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 18409; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Tall { return 18322; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 18556; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 18439; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::None { return 18238; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 18507; }
        if block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::None { return 18263; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 18268; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 18379; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::None { return 18292; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false { return 18342; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low { return 18402; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::Tall { return 18525; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Low { return 18542; }
        if block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == true { return 18382; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 18550; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 18489; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true { return 18477; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::None { return 18395; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Low { return 18351; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 18401; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 18471; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 18297; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 18390; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 18423; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 18375; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == true { return 18492; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 18498; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == true { return 18500; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false { return 18456; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 18518 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18488 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18374 {
            return Some(MudBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18429 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 18532 {
            return Some(MudBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 18553 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18432 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 18366 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 18298 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18330 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 18392 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 18509 {
            return Some(MudBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 18365 {
            return Some(MudBrickWall {
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 18252 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18314 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18485 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18511 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 18340 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18341 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18293 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 18526 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 18543 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18300 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18403 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 18431 {
            return Some(MudBrickWall {
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18548 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 18369 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18530 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18274 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18447 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 18491 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18397 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18248 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 18472 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18451 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18495 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 18315 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 18483 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 18411 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18410 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 18260 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 18452 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18554 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 18480 {
            return Some(MudBrickWall {
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18481 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 18549 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 18494 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18384 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 18442 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18289 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18328 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18461 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 18484 {
            return Some(MudBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18558 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18349 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 18433 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18460 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 18258 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18515 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18249 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 18310 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18406 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18449 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18327 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 18533 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18474 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 18287 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18332 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18436 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18250 {
            return Some(MudBrickWall {
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18458 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 18454 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 18326 {
            return Some(MudBrickWall {
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 18469 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18541 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 18531 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18415 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 18281 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 18473 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18247 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18273 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18385 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 18378 {
            return Some(MudBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18516 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18339 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18482 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 18437 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18271 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18408 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 18555 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18286 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 18353 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#up: false,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18412 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 18496 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 18276 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 18438 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 18435 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18368 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 18361 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 18354 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 18381 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 18457 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18280 {
            return Some(MudBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18245 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 18537 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 18414 {
            return Some(MudBrickWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 18545 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 18557 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 18534 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 18424 {
            return Some(MudBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 18278 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 18463 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18377 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18476 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 18305 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18318 {
            return Some(MudBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18356 {
            return Some(MudBrickWall {
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 18373 {
            return Some(MudBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 18329 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 18407 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 18453 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 18520 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 18487 {
            return Some(MudBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18306 {
            return Some(MudBrickWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18367 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18236 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::None,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 18535 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 18371 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 18479 {
            return Some(MudBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18499 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 18285 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18337 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18241 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18338 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18259 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18416 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18325 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18320 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 18523 {
            return Some(MudBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18497 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18502 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18291 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 18486 {
            return Some(MudBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18358 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18387 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18465 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#up: true,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 18521 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 18527 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18546 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 18264 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 18528 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18470 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 18270 {
            return Some(MudBrickWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18417 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18272 {
            return Some(MudBrickWall {
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 18434 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 18466 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 18333 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 18503 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18347 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 18514 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18355 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18512 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 18284 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 18551 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18510 {
            return Some(MudBrickWall {
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18288 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 18253 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 18265 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18478 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18324 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 18404 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18313 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 18239 {
            return Some(MudBrickWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 18255 {
            return Some(MudBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 18304 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18490 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 18283 {
            return Some(MudBrickWall {
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 18331 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 18246 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 18372 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 18383 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 18544 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18418 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18443 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 18522 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 18267 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 18343 {
            return Some(MudBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 18307 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 18380 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18529 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18536 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18308 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18422 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18244 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 18421 {
            return Some(MudBrickWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18282 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 18364 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18363 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18448 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18419 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 18524 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18504 {
            return Some(MudBrickWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 18428 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18538 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18513 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18299 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 18519 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18269 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 18237 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18251 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18393 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 18444 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 18508 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 18552 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 18459 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 18441 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 18243 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18294 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 18334 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18506 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18277 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 18275 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 18389 {
            return Some(MudBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 18475 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18360 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 18290 {
            return Some(MudBrickWall {
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 18352 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18357 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 18261 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 18468 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 18295 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 18266 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 18350 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 18376 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 18370 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18445 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 18254 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 18345 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18346 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 18336 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18257 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 18309 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18386 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 18388 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 18396 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18398 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 18425 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 18446 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18455 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 18426 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 18348 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18240 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 18394 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 18430 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18467 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18450 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 18440 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18400 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 18427 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: false,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18321 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 18464 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18540 {
            return Some(MudBrickWall {
                r#up: true,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18547 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18301 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18323 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 18399 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#up: false,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18505 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 18344 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 18317 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 18302 {
            return Some(MudBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18362 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18256 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 18359 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 18391 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18493 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 18413 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 18501 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18405 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 18462 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 18303 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18311 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 18279 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 18316 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18296 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 18539 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18262 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18517 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18420 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18559 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 18319 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18242 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 18312 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 18335 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18409 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18322 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18556 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18439 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18238 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 18507 {
            return Some(MudBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18263 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 18268 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#up: false,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18379 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 18292 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 18342 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18402 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 18525 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18542 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 18382 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18550 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18489 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 18477 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18395 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 18351 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 18401 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 18471 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18297 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18390 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18423 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18375 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18492 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 18498 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18500 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 18456 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

