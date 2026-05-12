use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndStoneBrickWall {
    pub r#north: North,
    pub up: bool,
    pub waterlogged: bool,
    pub r#west: West,
    pub r#east: East,
    pub r#south: South,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum South {
    None,
    Low,
    Tall,
}

impl BlockState for EndStoneBrickWall {
    fn to_id(self) -> i32 {
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Tall { return 20095; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 19858; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 19920; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall { return 20030; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Low { return 20178; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#up == true { return 20133; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::None { return 19868; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#waterlogged == true { return 19893; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::None { return 19896; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 19921; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Low { return 20008; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 20169; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None { return 20075; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == true { return 19857; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true { return 19970; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Low { return 19923; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None { return 19944; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::None { return 19865; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Low { return 19903; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::Low { return 19941; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == true { return 19980; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::None { return 19986; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true { return 20002; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::None { return 19935; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false { return 19985; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == true { return 20062; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == false { return 20153; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 19909; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 20161; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == false { return 19926; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == false { return 19987; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 19950; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::Low { return 20054; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Low { return 19978; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 20106; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 19989; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 20082; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 19869; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::None { return 19910; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Low { return 20025; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Tall { return 19958; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::None { return 20093; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::Tall { return 20145; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false { return 20126; }
        if block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == true { return 19870; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::None { return 19886; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 19955; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::None { return 20090; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == true { return 19905; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::None { return 20088; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::None { return 20078; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 20019; }
        if block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#up == false { return 20046; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == false { return 19959; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low { return 19915; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Low { return 20016; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#west == West::Low { return 20034; }
        if block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 19913; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 20061; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 20076; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::None { return 19952; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Tall { return 19940; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 20121; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#up == false { return 19901; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::None { return 20045; }
        if block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Tall { return 20113; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Low { return 19979; }
        if block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::None { return 20114; }
        if block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 20141; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None { return 19995; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::None { return 19866; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 20142; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#north == North::Tall { return 20164; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Low { return 20056; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Tall { return 19888; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false { return 19874; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 19945; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 20172; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 20156; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 20130; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Tall { return 19894; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 20057; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 20063; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::Tall { return 19969; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::Low { return 19983; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == true { return 19918; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 20065; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::None { return 19879; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == true { return 20086; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 19908; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 20035; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Low { return 20092; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Low { return 19863; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false { return 20080; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall { return 20068; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 19881; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low { return 20069; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == true { return 19892; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#east == East::None { return 19871; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::Low { return 19977; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 19887; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 20097; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::None { return 19976; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == true { return 19885; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::Low { return 20009; }
        if block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false { return 19932; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::None { return 19961; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 20120; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::None { return 20066; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low { return 19914; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false { return 19876; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == false { return 19895; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Low { return 19916; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::None { return 20042; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false { return 20083; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 20143; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Low { return 20006; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 19942; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 20136; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 20117; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall { return 20107; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::None { return 19882; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 20131; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 20135; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::None { return 19902; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == true { return 19943; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 20152; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == false { return 19998; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 19981; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 20170; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 20179; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#west == West::None { return 19982; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#waterlogged == false { return 19991; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 19906; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::None { return 20102; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 20158; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 20174; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false { return 20070; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 19878; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::None { return 19899; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::Low { return 20022; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 19964; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::None { return 19862; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == true { return 19884; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::None { return 19934; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 20060; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 20004; }
        if block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == true { return 19897; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::Tall { return 20150; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall { return 19900; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::None { return 20024; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None { return 19859; }
        if block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == true { return 20089; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::None { return 19873; }
        if block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 19947; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 20067; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 20012; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 19951; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::None { return 19877; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::Low { return 20124; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 20177; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == false { return 19962; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false { return 19999; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 19990; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == true { return 20029; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true { return 20048; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::None { return 19930; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Low { return 19968; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 19963; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::None { return 19972; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::None { return 20039; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 19946; }
        if block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true { return 19880; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 19971; }
        if block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::None { return 20105; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::Low { return 20148; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::None { return 20171; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 20103; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::Tall { return 19984; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 20085; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == true { return 20159; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 20028; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 20132; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::Tall { return 20122; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 20118; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#north == North::None { return 20104; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == true { return 19917; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#waterlogged == false { return 20154; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Low { return 20013; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Low { return 20018; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == true { return 19883; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 19937; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 20015; }
        if block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == false { return 20115; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None { return 19911; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::None { return 20119; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low { return 20010; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 20037; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low { return 19907; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true { return 20014; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::None { return 19861; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Low { return 20003; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == false { return 20032; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Low { return 20050; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 20077; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true { return 20110; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::None { return 19928; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 19954; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 20127; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 20162; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::None { return 19931; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 19938; }
        if block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 19875; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None { return 19956; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None { return 20100; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#up == false { return 19867; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall { return 20074; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Tall { return 20108; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#west == West::Low { return 20139; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 20149; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#up == true { return 20040; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 19997; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 20160; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 20073; }
        if block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 20109; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::None { return 19889; }
        if block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 20027; }
        if block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 20017; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#up == true { return 20052; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == true { return 20001; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 20123; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Low { return 19924; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 20173; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None { return 20007; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 19925; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true { return 19936; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Low { return 20049; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == false { return 20176; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false { return 20023; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 19975; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false { return 19960; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None { return 19988; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Tall { return 20144; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low { return 20000; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 20087; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::None { return 19939; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Tall { return 20005; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::None { return 19967; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Low { return 20033; }
        if block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::None { return 20111; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 19993; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 20031; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::None { return 19973; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::None { return 20047; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 20099; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 20064; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 20079; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::None { return 19872; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 20165; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false { return 19994; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == false { return 20081; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::Tall { return 20168; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 19957; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == true { return 20026; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::Low { return 20128; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 20125; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 20058; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::None { return 20041; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 20044; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None { return 19974; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == false { return 19927; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall { return 20072; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 20140; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#west == West::None { return 20138; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 20163; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == false { return 19919; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 20175; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Low { return 20091; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 20071; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 20137; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::None { return 19890; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 20116; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 20059; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 19912; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 19965; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 20157; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 19860; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 19992; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 19996; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 20147; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::None { return 19898; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::None { return 20094; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 20134; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 20021; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall { return 20051; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Tall { return 20053; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Tall { return 19929; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == true { return 20084; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true { return 19856; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Tall { return 20129; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Low { return 20055; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#north == North::Tall { return 19949; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 19948; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 20020; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None { return 20098; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::None { return 20155; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Low { return 20166; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Low { return 20036; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Tall { return 20096; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#waterlogged == true { return 19904; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 19953; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 20101; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 20112; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 20146; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 20167; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 19922; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::Tall { return 19864; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 20151; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Low { return 20011; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == true { return 20038; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 20043; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#west == West::Tall { return 19891; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 19966; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None { return 19933; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20095 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 19858 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19920 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 20030 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 20178 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 20133 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 19868 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 19893 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19896 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 19921 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 20008 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 20169 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 20075 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 19857 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 19970 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19923 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 19944 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 19865 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 19903 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 19941 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 19980 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 19986 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 20002 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 19935 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19985 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 20062 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 20153 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 19909 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 20161 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 19926 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 19987 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 19950 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 20054 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 19978 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 20106 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 19989 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 20082 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19869 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19910 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 20025 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 19958 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 20093 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 20145 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 20126 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 19870 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 19886 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 19955 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 20090 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 19905 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 20088 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 20078 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 20019 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 20046 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 19959 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 19915 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 20016 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 20034 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 19913 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 20061 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 20076 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 19952 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 19940 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 20121 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19901 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 20045 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 20113 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19979 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 20114 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 20141 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 19995 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 19866 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 20142 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 20164 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 20056 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19888 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 19874 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 19945 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 20172 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 20156 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 20130 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19894 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 20057 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 20063 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 19969 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 19983 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 19918 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 20065 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 19879 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 20086 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 19908 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 20035 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 20092 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 19863 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 20080 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 20068 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19881 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 20069 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 19892 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 19871 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 19977 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 19887 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 20097 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19976 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19885 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 20009 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 19932 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19961 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 20120 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 20066 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 19914 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19876 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 19895 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19916 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 20042 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 20083 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 20143 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 20006 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19942 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 20136 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 20117 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 20107 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19882 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 20131 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 20135 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 19902 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 19943 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 20152 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 19998 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 19981 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 20170 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 20179 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 19982 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 19991 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19906 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 20102 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 20158 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20174 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 20070 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 19878 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19899 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 20022 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19964 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 19862 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 19884 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 19934 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 20060 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 20004 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19897 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 20150 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19900 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 20024 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 19859 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 20089 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 19873 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 19947 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 20067 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 20012 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19951 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19877 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 20124 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 20177 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 19962 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 19999 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 19990 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20029 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 20048 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 19930 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 19968 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 19963 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 19972 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 20039 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 19946 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 19880 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 19971 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 20105 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 20148 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 20171 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 20103 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19984 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 20085 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 20159 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 20028 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 20132 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20122 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 20118 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 20104 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 19917 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 20154 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20013 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 20018 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19883 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 19937 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20015 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20115 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 19911 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 20119 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 20010 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 20037 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19907 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 20014 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 19861 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 20003 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 20032 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 20050 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 20077 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 20110 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 19928 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 19954 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 20127 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 20162 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 19931 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 19938 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19875 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 19956 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 20100 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 19867 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 20074 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 20108 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 20139 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 20149 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 20040 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 19997 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20160 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 20073 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 20109 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 19889 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 20027 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20017 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 20052 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 20001 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 20123 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19924 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 20173 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 20007 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 19925 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 19936 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20049 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 20176 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 20023 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 19975 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#up: false,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19960 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 19988 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 20144 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 20000 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 20087 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 19939 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 20005 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19967 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 20033 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 20111 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 19993 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 20031 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19973 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 20047 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 20099 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 20064 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 20079 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 19872 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 20165 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19994 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 20081 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 20168 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19957 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 20026 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 20128 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 20125 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20058 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 20041 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 20044 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19974 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 19927 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 20072 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 20140 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 20138 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 20163 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 19919 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20175 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 20091 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 20071 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 20137 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 19890 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 20116 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 20059 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19912 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19965 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 20157 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 19860 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19992 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 19996 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 20147 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19898 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 20094 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 20134 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 20021 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 20051 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 20053 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 19929 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 20084 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 19856 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 20129 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 20055 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 19949 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 19948 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 20020 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 20098 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 20155 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 20166 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 20036 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 20096 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19904 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19953 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 20101 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 20112 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 20146 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 20167 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19922 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 19864 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 20151 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 20011 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 20038 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 20043 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 19891 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 19966 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19933 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        return None;
    }
}

