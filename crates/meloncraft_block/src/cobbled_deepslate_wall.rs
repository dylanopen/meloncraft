use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CobbledDeepslateWall {
    pub r#north: North,
    pub waterlogged: bool,
    pub r#east: East,
    pub r#west: West,
    pub r#south: South,
    pub up: bool,
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

impl BlockState for CobbledDeepslateWall {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == true { return 27943; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == false { return 28050; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Tall { return 28066; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 27992; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::None { return 28114; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None { return 28042; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 28052; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Tall { return 28082; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true { return 27812; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true { return 28078; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#south == South::Tall { return 27880; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 27899; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Low { return 27986; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 27846; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 27981; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None { return 27837; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::Low { return 28016; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 28014; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Tall { return 28101; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None { return 28106; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#west == West::Tall { return 27954; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None { return 27926; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false { return 27881; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#waterlogged == true { return 27937; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true { return 27847; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low { return 27964; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 27970; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Tall { return 27835; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 27980; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true { return 28044; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 27932; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Tall { return 28086; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == false { return 27867; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Low { return 27815; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Low { return 27827; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == false { return 27953; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None { return 28012; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 27942; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 28132; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::None { return 27844; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 28003; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false { return 27832; }
        if block_state.r#west == West::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::None { return 27848; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 28008; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 27991; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::None { return 28060; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::Tall { return 28049; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 28058; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 28065; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 28097; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 28129; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 27918; }
        if block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::None { return 27930; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall { return 27936; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 28035; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::None { return 28068; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 27960; }
        if block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 27995; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 27974; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 28019; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == false { return 27814; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 27873; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::None { return 28048; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Low { return 27957; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Low { return 27863; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Low { return 27977; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 28119; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 27904; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::None { return 27836; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::None { return 27903; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::None { return 27820; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Low { return 27895; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false { return 27817; }
        if block_state.r#west == West::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Low { return 28043; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == true { return 27819; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 27821; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::None { return 27834; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 28051; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 27906; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::Low { return 28063; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Low { return 28080; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 28093; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 28127; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 28028; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == false { return 27976; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true { return 27923; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low { return 27866; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 28015; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false { return 27947; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::Low { return 27955; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::Tall { return 28120; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall { return 27993; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 27966; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 27941; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 27858; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::None { return 27822; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Low { return 28022; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 28072; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low { return 27962; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#waterlogged == false { return 27940; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::None { return 28036; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 28077; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall { return 28057; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false { return 27868; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::None { return 27921; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 27963; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 28098; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None { return 27838; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Tall { return 28053; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == true { return 28054; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::None { return 27915; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 27939; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Low { return 27938; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 27984; }
        if block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == true { return 28005; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 28123; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall { return 27891; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::Tall { return 28018; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::None { return 27887; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low { return 27969; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == true { return 27922; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#up == true { return 27823; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#south == South::None { return 27929; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 28084; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#up == true { return 27875; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == true { return 27912; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Tall { return 27839; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true { return 27874; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 28020; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 28089; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::None { return 27898; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false { return 27913; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 27879; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 27917; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::Tall { return 28002; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#up == true { return 27885; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 28095; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 27872; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 27852; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == false { return 27893; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall { return 27907; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::None { return 27931; }
        if block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true { return 28045; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::None { return 28047; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 28069; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::None { return 28046; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 28079; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false { return 28083; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Low { return 28067; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None { return 28074; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 28122; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false { return 27870; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::None { return 27818; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == false { return 27882; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall { return 28092; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::Low { return 28039; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 27888; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::None { return 27851; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::None { return 27845; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 28011; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Tall { return 28004; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Low { return 27869; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true { return 27902; }
        if block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 28124; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::None { return 28105; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::None { return 28032; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::Low { return 27830; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 27884; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::None { return 27996; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false { return 28038; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 28107; }
        if block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Low { return 27944; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false { return 27927; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low { return 27854; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low { return 27961; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 28001; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Low { return 28017; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 28007; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None { return 28099; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 28113; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true { return 28021; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false { return 28024; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 28025; }
        if block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false { return 28037; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false { return 28085; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 27946; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Tall { return 28111; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 27965; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Tall { return 28026; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::None { return 27890; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 28075; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 27894; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Low { return 27971; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#east == East::Tall { return 28102; }
        if block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == false { return 28034; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 27909; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Low { return 27968; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 28056; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low { return 27860; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None { return 27811; }
        if block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 27973; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 27989; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == false { return 28000; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Tall { return 27871; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::None { return 27914; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 28076; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 28094; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 27865; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true { return 27897; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 27983; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::None { return 28090; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true { return 27877; }
        if block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 28117; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == true { return 27945; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 28131; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None { return 27843; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false { return 27924; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true { return 27925; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 27876; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == true { return 28027; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::None { return 27988; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 27990; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Low { return 27878; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 27959; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::Low { return 28013; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 27975; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 28055; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 28130; }
        if block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 28073; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true { return 27841; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::Low { return 27864; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::None { return 27889; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 27979; }
        if block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 27972; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::Low { return 27829; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::None { return 27886; }
        if block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::None { return 28033; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::None { return 27856; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false { return 28059; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#up == false { return 28108; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 28126; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::None { return 27920; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == true { return 28029; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 28128; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true { return 27859; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 28071; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == true { return 27862; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false { return 28118; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::None { return 27985; }
        if block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None { return 27828; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 28103; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 28121; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::None { return 28031; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 28081; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == true { return 27956; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::Tall { return 27849; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 27896; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::None { return 27997; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true { return 28100; }
        if block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 27908; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::None { return 27813; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall { return 27911; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::None { return 27949; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 27950; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 27951; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#waterlogged == false { return 28030; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 28040; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false { return 28061; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None { return 27840; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 28116; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::None { return 27994; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::None { return 27934; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == false { return 27935; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true { return 28087; }
        if block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 27967; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true { return 27842; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None { return 27883; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#waterlogged == false { return 27850; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::None { return 27952; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 27948; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == true { return 27933; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 28023; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::None { return 27928; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::None { return 27998; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Tall { return 27910; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 28070; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::None { return 27826; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low { return 27987; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 27855; }
        if block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 27905; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 28096; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall { return 28110; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::Low { return 28115; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 28112; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None { return 28062; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 28134; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall { return 27978; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::None { return 27901; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 27958; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 28091; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::Tall { return 27825; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true { return 27853; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == false { return 27999; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 28006; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 28009; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::Low { return 27900; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 28041; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::None { return 27831; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 28010; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None { return 28109; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::None { return 27816; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 27824; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == false { return 27892; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true { return 28064; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 28125; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 27833; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Low { return 27857; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#up == true { return 28088; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false { return 28104; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 27861; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 27982; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Tall { return 27916; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true { return 27919; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 28133; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27943 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 28050 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 28066 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 27992 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28114 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 28042 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 28052 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28082 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 27812 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 28078 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 27880 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 27899 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 27986 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 27846 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 27981 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 27837 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 28016 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28014 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 28101 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 28106 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 27954 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 27926 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 27881 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 27937 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 27847 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 27964 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 27970 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 27835 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 27980 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28044 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 27932 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28086 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 27867 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 27815 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 27827 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 27953 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 28012 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 27942 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28132 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 27844 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 28003 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 27832 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 27848 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 28008 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 27991 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 28060 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 28049 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 28058 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28065 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 28097 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28129 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 27918 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 27930 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 27936 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28035 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28068 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 27960 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 27995 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 27974 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28019 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 27814 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 27873 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28048 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 27957 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 27863 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 27977 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28119 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 27904 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 27836 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 27903 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 27820 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#up: false,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 27895 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 27817 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 28043 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 27819 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 27821 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 27834 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 28051 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27906 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28063 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 28080 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28093 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 28127 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28028 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 27976 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 27923 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 27866 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28015 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 27947 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27955 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 28120 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 27993 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 27966 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 27941 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 27858 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 27822 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 28022 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 28072 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 27962 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 27940 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28036 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 28077 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 28057 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 27868 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 27921 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 27963 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28098 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 27838 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 28053 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28054 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 27915 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 27939 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 27938 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 27984 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 28005 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 28123 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 27891 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 28018 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 27887 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 27969 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 27922 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 27823 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 27929 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#up: false,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 28084 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 27875 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 27912 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 27839 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 27874 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 28020 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 28089 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 27898 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 27913 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 27879 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 27917 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28002 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 27885 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 28095 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 27872 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 27852 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 27893 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 27907 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 27931 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 28045 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28047 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 28069 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 28046 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 28079 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 28083 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 28067 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28074 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 28122 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 27870 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 27818 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 27882 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 28092 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 28039 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 27888 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 27851 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 27845 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 28011 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 28004 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 27869 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 27902 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28124 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 28105 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 28032 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 27830 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 27884 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 27996 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 28038 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28107 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 27944 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 27927 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 27854 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 27961 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 28001 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28017 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 28007 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28099 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 28113 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 28021 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 28024 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 28025 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28037 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 28085 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 27946 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28111 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 27965 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28026 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 27890 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 28075 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 27894 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 27971 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 28102 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 28034 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 27909 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 27968 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 28056 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 27860 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 27811 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 27973 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27989 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28000 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 27871 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 27914 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 28076 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28094 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 27865 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 27897 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 27983 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28090 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 27877 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 28117 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 27945 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 28131 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 27843 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 27924 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 27925 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 27876 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 28027 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 27988 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 27990 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 27878 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 27959 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 28013 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 27975 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 28055 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28130 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 28073 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 27841 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 27864 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 27889 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 27979 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 27972 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 27829 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 27886 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28033 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 27856 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 28059 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 28108 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 28126 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 27920 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 28029 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 28128 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 27859 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28071 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27862 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 28118 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 27985 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 27828 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#north: North::None,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 28103 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 28121 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 28031 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 28081 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 27956 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 27849 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 27896 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 27997 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 28100 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 27908 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 27813 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 27911 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 27949 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 27950 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 27951 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 28030 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28040 {
            return Some(CobbledDeepslateWall {
                r#north: North::None,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 28061 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 27840 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 28116 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 27994 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 27934 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 27935 {
            return Some(CobbledDeepslateWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28087 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 27967 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27842 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27883 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 27850 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 27952 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 27948 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 27933 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 28023 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 27928 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 27998 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 27910 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 28070 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 27826 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 27987 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 27855 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 27905 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 28096 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28110 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 28115 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 28112 {
            return Some(CobbledDeepslateWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 28062 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 28134 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 27978 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 27901 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 27958 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 28091 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 27825 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 27853 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 27999 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 28006 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 28009 {
            return Some(CobbledDeepslateWall {
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 27900 {
            return Some(CobbledDeepslateWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 28041 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 27831 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 28010 {
            return Some(CobbledDeepslateWall {
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 28109 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 27816 {
            return Some(CobbledDeepslateWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 27824 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 27892 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 28064 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 28125 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 27833 {
            return Some(CobbledDeepslateWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 27857 {
            return Some(CobbledDeepslateWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 28088 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 28104 {
            return Some(CobbledDeepslateWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 27861 {
            return Some(CobbledDeepslateWall {
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 27982 {
            return Some(CobbledDeepslateWall {
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 27916 {
            return Some(CobbledDeepslateWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 27919 {
            return Some(CobbledDeepslateWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 28133 {
            return Some(CobbledDeepslateWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

